// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use anyhow::Context;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::types::Result;

pub struct Client {
    pub api_key: String,
    pub api_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "role")]
pub enum Message {
    #[serde(rename = "system")]
    System {
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    #[serde(rename = "user")]
    User {
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    #[serde(rename = "assistant")]
    Assistant {
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<ToolCall>>,
    },
    #[serde(rename = "tool")]
    Tool {
        content: String,
        tool_call_id: String,
    },
}

impl TryFrom<crate::repo::messages::Message> for Message {
    type Error = anyhow::Error;

    fn try_from(message: crate::repo::messages::Message) -> std::result::Result<Self, Self::Error> {
        Ok(match message.role {
            crate::repo::messages::Role::System => Message::System {
                content: message
                    .content
                    .with_context(|| "Failed to get message content")?,
                name: None,
            },
            crate::repo::messages::Role::User => Message::User {
                content: message
                    .content
                    .with_context(|| "Failed to get message content")?,
                name: None,
            },
            crate::repo::messages::Role::CodeInterpreter => Message::User {
                content: message
                    .content
                    .with_context(|| "Failed to get message content")?,
                name: Some("Code-Interpreter".to_string()),
            },
            crate::repo::messages::Role::Assistant => Message::Assistant {
                content: message.content,
                name: None,
                tool_calls: match message.tool_calls {
                    Some(tool_calls) => Some(
                        serde_json::from_str(&tool_calls)
                            .with_context(|| "Failed to parse tool calls")?,
                    ),
                    None => None,
                },
            },
            crate::repo::messages::Role::Tool => Message::Tool {
                content: message
                    .content
                    .with_context(|| "Failed to get message content")?,
                tool_call_id: message
                    .tool_call_id
                    .with_context(|| "Failed to get tool call id")?,
            },
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: ToolType,
    pub function: FunctionCall,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ToolType {
    #[serde(rename = "function")]
    Function,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tool {
    #[serde(rename = "type")]
    pub type_: String,
    pub function: Function,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Function {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<FunctionParameters>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionParameters {
    #[serde(rename = "type")]
    pub type_: String,
    pub properties: HashMap<String, FunctionPropertyValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionPropertyValue {
    #[serde(rename = "type")]
    pub type_: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<FunctionParameters>,
}

#[derive(Debug, Serialize, Default)]
pub struct CreateChatCompletionRequest<'a> {
    pub model: &'a str,
    pub messages: Vec<Message>,
    pub tools: Option<Vec<Tool>>,
    pub stream: bool,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub system_fingerprint: Option<String>,
    pub choices: Vec<ChunkChoice>,
}

#[derive(Debug, Deserialize)]
pub struct ChunkChoice {
    pub index: u32,
    pub delta: Message,
    pub finish_reason: Option<String>,
    pub logprobs: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletion {
    pub created: u32,
    pub id: String,
    pub model: String,
    pub object: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub finish_reason: String,
    pub index: u32,
    pub message: Message,
    pub logprobs: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub completion_tokens: u32,
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}

impl<'a> Client {
    #[must_use]
    pub fn new(api_key: &'a str, api_url: &'a str) -> Self {
        Self {
            api_key: api_key.to_string(),
            api_url: api_url.to_string(),
        }
    }

    /// Creates a streaming chat completion.
    ///
    /// # Errors
    ///
    /// Returns error if there was a problem while making the API call.
    pub async fn create_chat_completion_stream(
        &self,
        mut request: CreateChatCompletionRequest<'_>,
    ) -> Result<Response> {
        request.stream = true;

        Ok(self
            .post_stream("chat/completions", &request)
            .await
            .with_context(|| "Failed to make inference API call")?)
    }

    /// Creates a chat completion.
    ///
    /// # Errors
    ///
    /// Returns error if there was a problem while making the API call.
    pub async fn create_chat_completion(
        &self,
        request: CreateChatCompletionRequest<'_>,
    ) -> Result<ChatCompletion> {
        Ok(self
            .post("chat/completions", &request)
            .await
            .with_context(|| "Failed to make inference API call")?)
    }

    /// Sends a stream POST request, returns the response for further processing.
    ///
    /// # Errors
    ///
    /// Returns error if there was a problem while sending the request or
    /// deserializing the response.
    pub async fn post_stream<B>(&self, endpoint: &str, body: B) -> Result<Response>
    where
        B: serde::Serialize,
    {
        let url = format!("{}{endpoint}", self.api_url);
        let client = reqwest::Client::new();

        let body =
            serde_json::to_value(body).with_context(|| "Failed to serialize request body")?;

        debug!("Inference API request: {:?}", body.to_string());

        Ok(client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header(
                "User-Agent",
                format!("StarfleetAI-Bridge/{}", env!("CARGO_PKG_VERSION")),
            )
            .json(&body)
            .send()
            .await
            .with_context(|| "Failed to send request")?)
    }

    /// Sends a POST request, deserializes the response to the given type.
    ///
    /// # Errors
    ///
    /// Returns error if there was a problem while sending the request or
    /// deserializing the response.
    pub async fn post<T, B>(&self, endpoint: &str, body: B) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize,
    {
        let url = format!("{}{endpoint}", self.api_url);
        let client = reqwest::Client::new();

        let body =
            serde_json::to_value(body).with_context(|| "Failed to serialize request body")?;
        debug!("Inference API request: {:?}", body.to_string());

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header(
                "User-Agent",
                format!("StarfleetAI-Bridge/{}", env!("CARGO_PKG_VERSION")),
            )
            .json(&body)
            .send()
            .await
            .with_context(|| "Failed to send request")?
            .text()
            .await
            .with_context(|| "Failed to get response text")?;

        debug!("Inference API response: {:?}", response);

        Ok(serde_json::from_str(&response).with_context(|| "Failed to deserialize response")?)
    }
}
