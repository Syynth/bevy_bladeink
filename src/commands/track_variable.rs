use bevy::prelude::*;
use bevy_crossbeam_event::CrossbeamEventSender;
use bladeink::{story::Story, value_type::ValueType};

use crate::{
    ink::{InkBindingDefinition, InkBindingError, InkBindingFn, InkValue},
    resources::InkVariables,
};

#[derive(Debug, Clone)]
pub(crate) struct TrackVariableCommand {
    pub name: String,
}

impl TrackVariableCommand {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

#[derive(Event, Clone)]
pub(crate) struct VariableUpdated {
    pub name: String,
    pub value: InkValue,
}

pub(crate) fn on_variable_updated(
    variable: On<VariableUpdated>,
    mut ink_variables: ResMut<InkVariables>,
) {
    ink_variables
        .tracked_variables
        .insert(variable.name.clone(), variable.value.clone());
}

impl InkBindingDefinition for VariableUpdated {
    type Event = VariableUpdated;

    fn try_parse_event(args: &[ValueType]) -> Result<Self::Event, InkBindingError> {
        match args {
            [ValueType::String(name), value] => Ok(VariableUpdated {
                name: name.string.clone(),
                value: value.clone().into(),
            }),
            _ => Err(InkBindingError::InvalidArguments),
        }
    }
}

impl Command for TrackVariableCommand {
    fn apply(self, world: &mut World) {
        let Some(channel) = world.get_resource::<CrossbeamEventSender<VariableUpdated>>() else {
            error!(
                "Failed to load state: CrossbeamEventSender resource not found. Did you forget to initialize the InkPlugin?"
            );
            return;
        };
        let channel = channel.clone();

        let Some(mut story) = world.get_non_send_resource_mut::<Story>() else {
            error!(
                "Failed to load state: Story resource not found. Did you forget to insert the InkProject resource?",
            );
            return;
        };

        match story.observe_variable(
            &self.name,
            InkBindingFn::<VariableUpdated>::to_observer(channel.clone()),
        ) {
            Ok(_) => {
                let current_value = story.get_variable(&self.name);
                let Some(mut ink_variables) = world.get_resource_mut::<InkVariables>() else {
                    error!("Failed to load state: InkVariables resource not found.");
                    return;
                };
                if let Some(current_value) = current_value {
                    ink_variables
                        .tracked_variables
                        .insert(self.name, current_value.into());
                }
            }
            Err(err) => {
                error!("Failed to track variable: {}", err);
            }
        }
    }
}

pub trait TrackVariableCommandsExt {
    fn ink_track_variable(&mut self, name: impl Into<String>) -> &mut Self;
}

impl<'w, 's> TrackVariableCommandsExt for Commands<'w, 's> {
    fn ink_track_variable(&mut self, name: impl Into<String>) -> &mut Self {
        self.queue(TrackVariableCommand::new(name));
        self
    }
}
