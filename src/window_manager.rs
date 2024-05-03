use crate::window::Window;
use core::slice::IterMut;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct WindowManager {
    windows: Vec<Box<dyn Window>>,
}

impl WindowManager {
    pub fn add(&mut self, window: Box<dyn Window>) {
        self.windows.push(window);
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Box<dyn Window>> {
        self.windows.iter_mut()
    }

    pub fn serialize(&self) -> String {
        let mut save = WindowManagerSave::default();

        for w in self.windows.iter() {
            let s = w.serialize();
            eprintln!("{} -> {s}", w.name());

            save.windows.insert(w.name().to_string(), s);
        }

        ron::ser::to_string(&save).unwrap_or_default()
    }

    pub fn deserialize(&mut self, data: &str) {
        let save: WindowManagerSave = ron::from_str(&data).unwrap_or_default();

        for (n, wd) in save.windows.iter() {
            if let Some(window) = self.windows.iter_mut().find(|w| w.name() == n) {
                eprintln!("Window {n} found.");
                window.deserialize(wd);
            } else {
                eprintln!("Window {n} not found!");
            }
        }
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
struct WindowManagerSave {
    windows: HashMap<String, String>,
}
