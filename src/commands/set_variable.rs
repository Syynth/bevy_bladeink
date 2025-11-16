use bevy::prelude::*;
use bladeink::story::Story;

use crate::{events::InkStateChanged, ink::InkValue};

#[derive(Debug, Clone)]
pub(crate) struct SetVariableCommand {
    pub name: String,
    pub value: InkValue,
}

impl SetVariableCommand {
    pub fn new(name: String, value: impl Into<InkValue>) -> Self {
        Self {
            name,
            value: value.into(),
        }
    }
}

impl Command for SetVariableCommand {
    fn apply(self, world: &mut World) {
        let Some(mut story) = world.get_non_send_resource_mut::<Story>() else {
            error!(
                "Failed to load state: Story resource not found. Did you forget to insert the InkProject resource?",
            );
            return;
        };
        match story.set_variable(&self.name, &(&self.value).into()) {
            Ok(_) => {
                world.trigger(InkStateChanged);
            }
            Err(err) => error!(
                "Failed to set variable '{}' to value '{:?}': {}",
                self.name, &self.value, err
            ),
        };
    }
}

pub trait SetVariableCommandsExt {
    fn ink_set_variable(&mut self, name: String, value: impl Into<InkValue>) -> &mut Self;
}

impl<'w, 's> SetVariableCommandsExt for Commands<'w, 's> {
    fn ink_set_variable(&mut self, name: String, value: impl Into<InkValue>) -> &mut Self {
        self.queue(SetVariableCommand::new(name, value));
        self
    }
}
