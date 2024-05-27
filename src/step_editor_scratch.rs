use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct StepEditorScratch {
    strings: HashMap<String, String>,
}

impl StepEditorScratch {
    pub fn clear(&mut self) {
        self.strings.clear();
    }

    pub fn string_mut(&mut self, name: &str, default_value: &str) -> &mut String {
        if !self.strings.contains_key(name) {
            self.strings
                .insert(String::from(name), String::from(default_value));
        }
        self.strings.get_mut(name).unwrap()
    }
}
