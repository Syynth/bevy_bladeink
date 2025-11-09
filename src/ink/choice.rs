use bevy::prelude::*;
use bladeink::choice::*;

#[derive(Debug, Clone, Reflect, PartialEq, Eq, Hash)]
pub struct ChoiceItem {
    pub(crate) text: String,
    pub(crate) index: usize,
    pub(crate) tags: Vec<String>,
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
