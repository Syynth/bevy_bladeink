use bevy::prelude::*;
use bladeink::choice::*;

#[derive(Debug, Clone, Reflect, PartialEq, Eq, Hash)]
pub struct ChoiceItem {
    pub(crate) text: String,
    pub(crate) index: usize,
    pub(crate) tags: Vec<String>,
}

impl ChoiceItem {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn tags(&self) -> &[String] {
        &self.tags
    }
}

impl From<Choice> for ChoiceItem {
    fn from(choice: Choice) -> Self {
        Self {
            text: choice.text,
            index: *choice.index.borrow(),
            tags: choice.tags,
        }
    }
}

impl From<&Choice> for ChoiceItem {
    fn from(choice: &Choice) -> Self {
        Self {
            text: choice.text.clone(),
            index: *choice.index.borrow(),
            tags: choice.tags.clone(),
        }
    }
}
