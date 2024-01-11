// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::types::Result;

const API_URL: &str = "https://api.openai.com/v1/";

pub struct Client<'a> {
    pub api_key: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
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
            crate::repo::messages::Role::Assistant => Message::Assistant {
                content: message.content,
                name: None,
                tool_calls: None,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: ToolType,
    pub function: FunctionCall,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ToolType {
    #[serde(rename = "function")]
    Function,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tool {
    #[serde(rename = "type")]
    pub type_: String,
    pub function: Function,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Function {
    pub name: String,
    pub parameters: FunctionParameters,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionParameters {
    #[serde(rename = "type")]
    pub type_: String,
    pub properties: HashMap<String, FunctionPropetryValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionPropetryValue {
    #[serde(rename = "type")]
    pub type_: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct CreateChatCompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub tools: Vec<Tool>,
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

impl<'a> Client<'a> {
    #[must_use]
    pub fn new(api_key: &'a str) -> Self {
        Self { api_key }
    }

    /// Creates a chat completion.
    ///
    /// # Errors
    ///
    /// Returns error if there was a problem while making the API call.
    pub async fn create_chat_completion(
        &self,
        request: CreateChatCompletionRequest,
    ) -> Result<ChatCompletion> {
        Ok(self
            .post("chat/completions", &request)
            .await
            .with_context(|| "Failed to make OpenAI API call")?)
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
        let url = format!("{API_URL}{endpoint}");
        let client = reqwest::Client::new();

        Ok(client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .with_context(|| "Failed to send request")?
            .json::<T>()
            .await
            .with_context(|| "Failed to deserialize response")?)
    }
}
