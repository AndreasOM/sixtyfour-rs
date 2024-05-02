use crate::Command;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use lazy_static::lazy_static;
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
    pub static ref COMMAND_QUEUE: CommandQueue = CommandQueue::default();
}

#[derive(Debug, Default)]
pub struct CommandQueue {
    queue: Arc<Mutex<VecDeque<Command>>>,
}

impl CommandQueue {
    pub fn send(&self, command: Command) -> Result<()> {
        let mut q = self
            .queue
            .lock()
            .map_err(|_e| eyre!("Couldn't lock queue"))?;

        q.push_back(command);

        Ok(())
    }

    pub fn next(&self) -> Option<Command> {
        let Ok(mut q) = self.queue.lock() else {
            return None;
        };

        q.pop_front()
    }
}
