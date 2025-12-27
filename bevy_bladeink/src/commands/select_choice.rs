use bevy::prelude::*;
use bladeink::story::Story;

use crate::prelude::ContinueSequenceCommandsExt;

/// Represents a command to continue an ink sequence.
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct SelectChoiceCommand(usize);

impl SelectChoiceCommand {
    /// Creates a new `SelectChoiceCommand`.
    pub(crate) fn new(choice_index: usize) -> Self {
        SelectChoiceCommand(choice_index)
    }
}

impl Command for SelectChoiceCommand {
    fn apply(self, world: &mut World) {
        let Some(mut story) = world.get_non_send_resource_mut::<Story>() else {
            error!(
                "Failed to load state: Story resource not found. Did you forget to insert the InkProject resource?",
            );
            return;
        };
        let choices = story.get_current_choices();
        if self.0 >= choices.len() {
            error!(
                "Invalid choice index: {}, only {} choices available",
                self.0,
                choices.len()
            );
            return;
        }

        match story.choose_choice_index(self.0) {
            Ok(_) => {
                let mut commands = world.commands();
                commands.ink_continue_sequence();
                world.flush();
            }
            Err(err) => error!("Failed to select choice: {}", err),
        }
    }
}

/// Helper trait for adding `SelectChoiceCommand` to a `Commands` instance.
pub trait SelectChoiceCommandsExt {
    /// Attempts to advance the story to the next step in the current sequence.
    fn ink_select_choice(&mut self, choice_index: usize) -> &mut Self;
}

impl<'w, 's> SelectChoiceCommandsExt for Commands<'w, 's> {
    fn ink_select_choice(&mut self, choice_index: usize) -> &mut Self {
        self.queue(SelectChoiceCommand::new(choice_index));
        self
    }
}
