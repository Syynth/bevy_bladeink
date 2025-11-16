use bevy::{platform::collections::HashMap, reflect::Reflect};
use bladeink::{story::Story, story_error::StoryError};

use crate::{ink::InkValue, prelude::InkVariables};

#[derive(Debug, Clone, Reflect)]
pub struct InkState {
    pub(crate) serialized_state: String,
    pub(crate) tracked_variables: HashMap<String, InkValue>,
}

impl InkState {
    pub fn new(state: String, variables: HashMap<String, InkValue>) -> Self {
        InkState {
            serialized_state: state,
            tracked_variables: variables,
        }
    }
}

impl InkState {
    pub fn from_story(story: &mut Story, variables: &InkVariables) -> Result<Self, StoryError> {
        let state = story.save_state()?;
        let mut variables = variables.clone();

        for (name, value) in variables.tracked_variables.iter_mut() {
            if let Some(ink_value) = story.get_variable(name) {
                *value = ink_value.clone().into();
            }
        }

        Ok(Self::new(state, variables.tracked_variables))
    }
}
