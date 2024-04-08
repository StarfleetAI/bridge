// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Context};
use askama::Template;
use serde::Deserialize;
use serde_json::json;
use tracing::{debug, error, instrument, trace};

use crate::browser::{Browser, BrowserBuilder};
use crate::chats::construct_tools;
use crate::clients::openai::{Client, CreateChatCompletionRequest, Message, ToolCall};
use crate::repo::abilities::Ability;
use crate::repo::models::Model;
use crate::types::Result;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum WebBrowsingResult {
    Failure(String),
    Text(String),
}

#[derive(Debug)]
pub struct Builder<'a> {
    app_local_data_dir: &'a str,
    objective: String,
    model: Option<&'a Model>,
    api_key: String,
}

#[derive(Debug)]
pub struct WebBrowsing<'a> {
    browser: Browser,
    notebook: String,
    objective: String,
    model: &'a Model,
    api_key: String,
    messages: Vec<Message>,
    is_active: bool,
    history: Vec<String>,
}

#[derive(Deserialize)]
pub struct GotoArgs {
    pub url: String,
}

#[derive(Deserialize)]
pub struct SendKeysArgs {
    pub id: i64,
    pub text: String,
}

#[derive(Deserialize)]
pub struct ClickArgs {
    pub id: i64,
}

#[derive(Deserialize)]
pub struct AppendNotebookArgs {
    pub text: String,
}

#[derive(Deserialize)]
pub struct ReplaceNotebookArgs {
    pub text: String,
}

#[derive(Deserialize)]
pub struct FailArgs {
    pub reason: String,
}

#[derive(Template)]
#[template(path = "web_browsing/system_message.md", escape = "none")]
struct SystemMessageTemplate<'a> {
    objective: &'a str,
    notebook: &'a str,
}

#[derive(Template)]
#[template(path = "web_browsing/viewport_message.md", escape = "none")]
struct ViewportMessageTemplate<'a> {
    current_url: &'a str,
    scroll_position: i64,
    elements: &'a str,
    history: &'a [String],
}

#[derive(Template)]
#[template(path = "web_browsing/self_reflection_message.md", escape = "none")]
struct SelfReflectionMessageTemplate {}

impl<'a> Default for Builder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Builder<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            app_local_data_dir: "",
            objective: String::new(),
            model: None,
            api_key: String::new(),
        }
    }

    #[must_use]
    pub fn with_app_local_data_dir(mut self, app_local_data_dir: &'a str) -> Self {
        self.app_local_data_dir = app_local_data_dir;
        self
    }

    #[must_use]
    pub fn with_objective(mut self, objective: &str) -> Self {
        self.objective = objective.to_string();
        self
    }

    #[must_use]
    pub fn with_model(mut self, model: &'a Model) -> Self {
        self.model = Some(model);
        self
    }

    #[must_use]
    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = api_key.to_string();
        self
    }

    /// Build a new `WebBrowsing` instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the browser fails to connect.
    pub async fn build(self) -> Result<WebBrowsing<'a>> {
        let mut browser = BrowserBuilder::new(self.app_local_data_dir)
            .connect()
            .await?;
        browser.goto("https://google.com").await?;

        Ok(WebBrowsing {
            browser,
            notebook: String::new(),
            objective: self.objective,
            model: self.model.context("Model not provided")?,
            api_key: self.api_key,
            messages: vec![],
            is_active: false,
            history: vec![],
        })
    }
}

impl WebBrowsing<'_> {
    #[instrument(skip(self))]
    pub async fn perform(&mut self) -> Result<WebBrowsingResult> {
        debug!("Objective: `{}`", self.objective);
        self.is_active = true;

        loop {
            // Construct messages for the LLM
            let mut messages = self.messages().await?;
            trace!("Messages: {:?}", messages);

            // Send request to LLM
            let client = Client::new(&self.api_key, self.model.api_url_or_default());
            let response = client
                .create_chat_completion(CreateChatCompletionRequest {
                    model: &self.model.name,
                    messages: messages.clone(),
                    tools: construct_tools(Self::abilities()).await?,
                    ..Default::default()
                })
                .await
                .context("Failed to create chat completion")?;

            let has_content;

            // Process LLM function calls
            let choice = response.choices.first().context("No response from LLM")?;
            match &choice.message {
                Message::Assistant {
                    tool_calls,
                    content,
                    ..
                } => {
                    has_content = content.is_some();

                    if has_content {
                        debug!("LLM Response: {}", content.as_ref().unwrap());
                    }

                    messages.push(choice.message.clone());
                    self.messages.push(choice.message.clone());

                    self.call_tools(tool_calls).await?;
                }
                _ => return Err(anyhow!("Unexpected response from LLM").into()),
            }

            if has_content {
                // Reflect on text response from LLM
                messages.push(Self::self_reflection_message()?);
                let response = client
                    .create_chat_completion(CreateChatCompletionRequest {
                        model: &self.model.name,
                        messages,
                        tools: construct_tools(Self::self_reflection_abilities()).await?,
                        ..Default::default()
                    })
                    .await
                    .context("Failed to create chat completion")?;

                // Process self-reflection function calls
                let choice = response.choices.first().context("No response from LLM")?;
                match &choice.message {
                    Message::Assistant {
                        tool_calls,
                        content,
                        ..
                    } => {
                        debug!("Self-reflection: {:?}", content);
                        self.messages.push(choice.message.clone());

                        self.call_self_reflection_tools(tool_calls)?;
                    }
                    _ => return Err(anyhow!("Unexpected response from LLM").into()),
                }
            }

            if !self.is_active {
                break;
            }
        }

        // TODO: return result
        Ok(WebBrowsingResult::Text(self.objective.clone()))
    }

    fn push_tool_message(&mut self, content: &str, tool_call_id: &str) {
        self.messages.push(Message::Tool {
            content: format!("```\n{content}\n```"),
            tool_call_id: tool_call_id.to_string(),
        });
    }

    async fn call_tools(&mut self, maybe_tool_calls: &Option<Vec<ToolCall>>) -> Result<()> {
        let Some(tool_calls) = maybe_tool_calls else {
            return Ok(());
        };

        for tool_call in tool_calls {
            match tool_call.function.name.as_str() {
                "scroll_down" => {
                    debug!("Scrolling down");

                    self.messages.clear();
                    self.browser.scroll_down().await?;
                    self.browser.save_screenshot().await?;
                    self.history.push("scroll_down".to_string());
                }
                // "scroll_up" => {
                //     self.messages.clear();
                //     self.browser.scroll_up().await?;
                //     self.browser.save_screenshot().await?;
                // }
                "goto" => {
                    self.messages.clear();

                    let args: GotoArgs = serde_json::from_str(&tool_call.function.arguments)?;
                    debug!("Navigating to: {}", args.url);
                    self.browser.goto(&args.url).await?;
                    self.browser.save_screenshot().await?;
                    self.history.push(args.url.clone());
                }
                "send_keys" => {
                    let args: SendKeysArgs = serde_json::from_str(&tool_call.function.arguments)?;
                    debug!("Sending keys: {}", args.text);
                    self.browser.save_screenshot().await?;
                    self.browser.send_keys(args.id, &args.text).await?;
                    self.push_tool_message("Keys sent", &tool_call.id);
                    self.browser.save_screenshot().await?;
                }
                "click" => {
                    let current_url = self.browser.get_current_url().await?;
                    let args: ClickArgs = serde_json::from_str(&tool_call.function.arguments)?;
                    debug!("Clicking element: {}", args.id);
                    self.browser.click(args.id).await?;
                    self.push_tool_message("Clicked", &tool_call.id);
                    self.browser.save_screenshot().await?;

                    if current_url != self.browser.get_current_url().await? {
                        debug!("Navigated to: {}", self.browser.get_current_url().await?);
                        self.history.push(current_url.clone());
                        self.messages.clear();
                    }
                }
                "append_notebook" => {
                    let args: AppendNotebookArgs =
                        serde_json::from_str(&tool_call.function.arguments)?;
                    debug!("Appending to notebook: {}", args.text);
                    self.notebook.push_str("\n\n---\n\n");
                    self.notebook
                        .push_str(self.browser.get_current_url().await?.as_str());
                    self.notebook.push_str("\n\n");
                    self.notebook.push_str(&args.text);
                    self.push_tool_message("Appended to notebook", &tool_call.id);
                }
                "clear_notebook" => {
                    debug!("Clearing notebook");
                    self.notebook.clear();
                    self.push_tool_message("Notebook cleared", &tool_call.id);
                }
                _ => return Err(anyhow!("Unknown tool call: {}", tool_call.function.name).into()),
            }
        }

        Ok(())
    }

    fn call_self_reflection_tools(
        &mut self,
        maybe_tool_calls: &Option<Vec<ToolCall>>,
    ) -> Result<()> {
        let Some(tool_calls) = maybe_tool_calls else {
            return Ok(());
        };

        for tool_call in tool_calls {
            match tool_call.function.name.as_str() {
                "done" => self.is_active = false,
                "fail" => {
                    let args: FailArgs = serde_json::from_str(&tool_call.function.arguments)?;
                    error!("Objective failed: {}", args.reason);
                    self.is_active = false;
                }
                _ => return Err(anyhow!("Unknown tool call: {}", tool_call.function.name).into()),
            }
        }

        Ok(())
    }

    fn self_reflection_message() -> Result<Message> {
        let self_reflection_message_content = SelfReflectionMessageTemplate {}.render()?;

        Ok(Message::User {
            content: self_reflection_message_content,
            name: None,
        })
    }

    async fn messages(&self) -> Result<Vec<Message>> {
        let elements = self.browser.list_viewport_elements().await?;
        let elements_json = serde_json::to_string_pretty(&elements)?;

        let system_message_content = SystemMessageTemplate {
            objective: &self.objective,
            notebook: &self.notebook,
        }
        .render()?;

        let viewport_message_content = ViewportMessageTemplate {
            current_url: self.browser.get_current_url().await?.as_str(),
            scroll_position: self.browser.get_scroll_position().await?,
            elements: &elements_json,
            history: &self.history,
        }
        .render()?;

        let mut messages = vec![
            Message::System {
                content: system_message_content,
                name: None,
            },
            Message::User {
                content: viewport_message_content,
                name: None,
            },
        ];

        messages.extend_from_slice(&self.messages);

        Ok(messages)
    }

    fn abilities() -> Vec<Ability> {
        vec![
            Ability::for_fn("Scroll one page down", &json!({ "name": "scroll_down" })),
            // Ability::for_fn("Scroll one page up", &json!({ "name": "scroll_up" })),
            Ability::for_fn(
                "Go to URL",
                &json!({
                    "name": "goto",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "url": {
                                "type": "string",
                                "description": "URL to navigate to"
                            }
                        }
                    }
                }),
            ),
            Ability::for_fn(
                "Type text into an element",
                &json!({
                    "name": "send_keys",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "id": {
                                "type": "integer",
                                "description": "Element ID to type into"
                            },
                            "text": {
                                "type": "string",
                                "description": "Text to type"
                            }
                        }
                    }
                }),
            ),
            Ability::for_fn(
                "Click an element",
                &json!({
                    "name": "click",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "id": {
                                "type": "integer",
                                "description": "Element ID to click"
                            }
                        }
                    }
                }),
            ),
            Ability::for_fn(
                "Append text to notebook",
                &json!({
                    "name": "append_notebook",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "text": {
                                "type": "string",
                                "description": "Text to append to notebook"
                            }
                        }
                    }
                }),
            ),
            // Ability::for_fn(
            //     "Replace notebook text",
            //     &json!({
            //         "name": "replace_notebook",
            //         "parameters": {
            //             "type": "object",
            //             "properties": {
            //                 "text": {
            //                     "type": "string",
            //                     "description": "Text to replace notebook with"
            //                 }
            //             }
            //         }
            //     }),
            // ),
            Ability::for_fn("Clear notebook", &json!({ "name": "clear_notebook" })),
        ]
    }

    fn self_reflection_abilities() -> Vec<Ability> {
        vec![
            Ability::for_fn(
                "Mark current objective as complete",
                &json!({ "name": "done" }),
            ),
            Ability::for_fn(
                "Mark current objective as failed",
                &json!({
                    "name": "fail",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "reason": {
                                "type": "string",
                                "description": "Reason for failure"
                            }
                        }
                    }
                }),
            ),
        ]
    }
}
