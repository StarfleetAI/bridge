// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use async_trait::async_trait;
use bridge_common::{
    channel::{Emitter, Event},
    types::Result,
};
use tauri::{AppHandle, Manager};
use tracing::{debug, instrument, trace};

#[allow(clippy::module_name_repetitions)]
pub struct TauriChannel(AppHandle);

#[async_trait]
impl Emitter for TauriChannel {
    #[instrument(skip(self, event))]
    async fn emit<'a>(&self, _user_id: i32, event: Event<'a>) -> Result<()> {
        debug!("Emitting event");
        trace!("Event: {:?}", event);

        self.0
            .emit_all(&Self::event_name(&event), &event)
            .context("Failed to emit event")?;

        Ok(())
    }
}

impl TauriChannel {
    #[must_use]
    pub fn new(app_handle: AppHandle) -> Self {
        Self(app_handle)
    }

    fn event_name(event: &Event) -> String {
        match event {
            Event::ChatUpdated(_) => "chats:updated".to_string(),
            Event::TaskCreated(_) => "tasks:created".to_string(),
            Event::TaskUpdated(_) => "tasks:updated".to_string(),
            Event::MessageCreated(_) => "messages:created".to_string(),
            Event::MessageUpdated(_) => "messages:updated".to_string(),
            Event::TaskResultCreated(_) => "task_results:created".to_string(),
        }
    }
}
