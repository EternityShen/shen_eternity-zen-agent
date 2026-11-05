use crate::logger::handle::LoggerHandle;
use ambi::{Agent, AgentState, ChatRunner};
use smol::stream::StreamExt;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

pub struct AgentRunTimer {
    agent: Arc<Agent>,
    state: Arc<RwLock<AgentState>>,
    logger_handle: Arc<Mutex<LoggerHandle>>,
}

impl AgentRunTimer {
    pub fn new(
        agent: Arc<Agent>,
        state: Arc<RwLock<AgentState>>,
        logger_handle: Arc<Mutex<LoggerHandle>>,
    ) -> Self {
        Self {
            agent,
            state,
            logger_handle,
        }
    }

    pub fn chat(&mut self, prompt: String) {
        let agent = self.agent.clone();
        let state = self.state.clone();

        tokio::spawn(async move {
            let runner = ChatRunner::default();
            match runner.chat_stream(&agent, &state, &prompt).await {
                Ok(mut chunks) => while let Some(chunk) = chunks.next().await {},
                Err(e) => {}
            }
        });
    }
}
