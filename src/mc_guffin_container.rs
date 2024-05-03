use crate::engine::McGuffin;
use egui::mutex::Mutex;
use egui::mutex::MutexGuard;
use std::sync::Arc;

#[derive(Default)]
pub struct McGuffinContainer(Arc<Mutex<McGuffin>>);
impl McGuffinContainer {
    pub fn lock(&self) -> MutexGuard<'_, McGuffin> {
        self.0.lock()
    }
    pub fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl core::fmt::Debug for McGuffinContainer {
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        Ok(())
    }
}
