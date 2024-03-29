// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use async_recursion::async_recursion;
use serde::Deserialize;
use serde_json::json;
use tauri::{AppHandle, Manager, State};
use tokio::sync::RwLock;
use tracing::info;

use crate::chats::construct_tools;
use crate::clients::openai::{
    ChatCompletion, Client, CreateChatCompletionRequest, Message, ToolCall,
};
use crate::repo;
use crate::repo::abilities::Ability;

use crate::repo::models;
use crate::repo::tasks::{CreateParams, Task};
use crate::settings::Settings;
use crate::types::{DbPool, Result};

const PROMPT: &str = r#"You are a project manager with the objective of orchestrating task execution using your team effectively.

## Planning Guidelines

1. Ensure each task is a discrete, manageable unit of work. Avoid splitting broad concepts like "research" and "understanding" into separate sub-tasks.
2. Assign each task to only one agent.
3. A task can have multiple sub-tasks without any limit on nesting levels.
4. Parent tasks have visibility over the outcomes of their sub-tasks.
5. Sub-tasks have visibility over the outcomes of their sibling tasks.
6. Tasks should be executed in a sequential manner.

## Examples

1. Simple tasks like writing a straight-forward script should not be divided into sub-tasks.
2. Complex tasks, such as those requiring internet data retrieval and script writing, should be split into two sub-tasks: data gathering and script development.
3. Straightforward queries like "tell me about Ruby on Rails" do not require planning. Avoid unnecessary task creation for such direct questions.

## Additional Notes

1. Use the web browser sparingly to minimize user billing. Avoid researching well-known topics.
2. Eliminate "review" steps from tasks; the user will review the final results. Focus on creating meaningful, actionable tasks.
3. Plan at a single level of depth only.
4. Do not include tasks for delivering results like "save a file" or "provide a URL."
5. Keep task titles succinct and to the point.

## Response Format

Approach each task methodically and devise a plan to achieve it. Respond with concise task titles and assigned agents only, omitting any additional explanations."#;

pub struct TaskPlanner<'a> {
    app_handle: &'a AppHandle,
}

#[derive(Debug, Deserialize)]
pub struct ExecutionPlanTask {
    pub title: String,
    pub agent_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct ExecutionPlan {
    pub tasks: Vec<ExecutionPlanTask>,
}

#[derive(Debug, Deserialize)]
struct SfaiAssignToAgentArgs {
    agent_id: i64,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Planning is not available for tasks with status: {0:?}")]
    PlanningUnavailable(repo::tasks::Status),
    #[error("No tool call received from LLM")]
    NoToolCallReceived,
    #[error("Non-assistant message received from LLM")]
    NonAssistantMessage,
    #[error("Empty plan received from LLM")]
    EmptyPlan,
}

impl<'a> TaskPlanner<'a> {
    #[must_use]
    pub fn new(app_handle: &'a AppHandle) -> Self {
        Self { app_handle }
    }

    /// Plan task execution
    ///
    /// # Errors
    ///
    /// Returns error if planning is unavailable for the task status, or if there was a problem while planning the task execution.
    #[async_recursion]
    pub async fn plan(&self, task: &mut Task) -> Result<()> {
        match task.status {
            repo::tasks::Status::ToDo | repo::tasks::Status::InProgress => {
                return Err(Error::PlanningUnavailable(task.status).into())
            }
            _ => {}
        }

        info!("Planning task: {}", task.id);

        let pool: State<'_, DbPool> = self.app_handle.state();
        let settings: State<'_, RwLock<Settings>> = self.app_handle.state();
        let settings_guard = settings.read().await;

        let messages = self.messages(task).await?;
        let tools = construct_tools(Self::abilities()).await?;

        let model_full_name = settings_guard.default_model();

        let model = models::get(&*pool, model_full_name)
            .await
            .context("Failed to get model")?;

        let api_key = settings_guard
            .api_keys
            .get(&model.provider)
            .with_context(|| format!("Failed to get api key for provider: {:?}", model.provider))?;

        // Send request to LLM
        let client = Client::new(api_key, model.api_url_or_default());
        let response = client
            .create_chat_completion(CreateChatCompletionRequest {
                model: &model.name,
                messages,
                stream: false,
                tools,
            })
            .await
            .context("Failed to create chat completion")?;

        let plan = Self::plan_from_response(&response, task)
            .context("Failed to plan a task execution")?
            .context("Empty plan received")?;

        if plan.tasks.is_empty() {
            // TODO: retry planning
            return Err(Error::EmptyPlan.into());
        }

        if plan.tasks.len() == 1 {
            task.agent_id = plan.tasks[0].agent_id;
            repo::tasks::assign(&*pool, task.id, task.agent_id).await?;

            self.app_handle.emit_all("tasks:updated", &task)?;

            return Ok(());
        }

        for sub_task in plan.tasks {
            let mut task = repo::tasks::create(
                &*pool,
                CreateParams {
                    title: &sub_task.title,
                    summary: Some(""),
                    agent_id: sub_task.agent_id,
                    ancestry: Some(&task.children_ancestry()),
                    ..Default::default()
                },
            )
            .await?;

            self.app_handle.emit_all("tasks:created", &task)?;

            // Plan sub-tasks
            Self::new(self.app_handle).plan(&mut task).await?;
        }

        Ok(())
    }

    fn assistant_message_tool_calls(response: &ChatCompletion) -> Result<&Vec<ToolCall>> {
        match &response.choices[0].message {
            Message::Assistant { tool_calls, .. } => match tool_calls {
                Some(tc) => Ok(tc),
                _ => Err(Error::NoToolCallReceived.into()),
            },
            _ => Err(Error::NonAssistantMessage.into()),
        }
    }

    fn plan_from_response(response: &ChatCompletion, task: &Task) -> Result<Option<ExecutionPlan>> {
        let tool_calls = Self::assistant_message_tool_calls(response)?;
        let mut plan = None;

        for tool_call in tool_calls {
            match tool_call.function.name.as_str() {
                "sfai_plan_task_execution" => {
                    plan = Some(
                        serde_json::from_str(&tool_call.function.arguments)
                            .context("Failed to parse plan")?,
                    );
                }
                "sfai_assign_to_agent" => {
                    let args: SfaiAssignToAgentArgs =
                        serde_json::from_str(&tool_call.function.arguments)
                            .context("Failed to parse `sfai_assign_to_agent` arguments")?;

                    plan = Some(ExecutionPlan {
                        tasks: vec![ExecutionPlanTask {
                            title: task.title.clone(),
                            agent_id: args.agent_id,
                        }],
                    });
                }
                _ => {}
            }
        }

        Ok(plan)
    }

    async fn messages(&self, task: &Task) -> Result<Vec<Message>> {
        let pool: State<'_, DbPool> = self.app_handle.state();

        let agents = repo::agents::list_enabled(&*pool)
            .await
            .context("Failed to list agents")?
            .into_iter()
            .map(|agent| format!("- ID: {}. {}: {}", agent.id, agent.name, agent.description))
            .collect::<Vec<String>>();

        let agents = if agents.is_empty() {
            "No agents available".to_string()
        } else {
            agents.join("\n")
        };

        let summary = if task.summary.is_empty() {
            String::new()
        } else {
            format!("\n\n{}", task.summary)
        };

        Ok(vec![
            Message::System {
                content: PROMPT.to_string(),
                name: None,
            },
            Message::User {
                content: format!(
                    "## Available Agents\n\n{}\n\n## Task: {}{}\n\n## Attachments\n\nNo attachments provided.",
                    agents,
                    task.title,
                    summary
                ),
                name: None,
            },
        ])
    }

    fn abilities() -> Vec<Ability> {
        vec![
            Ability::for_fn(
                "No plan required. Assign task to an agent",
                &json!({
                    "name": "sfai_assign_to_agent",
                    "parameters": {
                        "type":"object",
                        "properties": {
                            "agent_id": {
                                "type": "integer",
                                "description": "ID of the agent to assign the task to"
                            }
                        }
                    }
                }),
            ),
            Ability::for_fn(
                "Plan task execution",
                &json!({
                    "name": "sfai_plan_task_execution",
                    "parameters": {
                        "type":"object",
                        "properties": {
                            "tasks": {
                                "type": "array",
                                "description": "List of planned sub-tasks",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "title": {
                                            "type": "string",
                                            "description": "Title of the task"
                                        },
                                        "agent_id": {
                                            "type": "integer",
                                            "description": "ID of the agent to assign the task to"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }),
            ),
        ]
    }
}
