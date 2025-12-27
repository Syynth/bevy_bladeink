use bevy::prelude::*;
use bladeink::story::Story;

use crate::{
    events::{DeliverChoices, DeliverLine, InkStateChanged, SequenceEnd},
    ink::ChoiceItem,
};

/// Represents a command to continue an ink sequence.
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct ContinueSequenceCommand;

impl ContinueSequenceCommand {
    /// Creates a new `ContinueSequenceCommand`.
    pub(crate) fn new() -> Self {
        ContinueSequenceCommand
    }
}

impl Command for ContinueSequenceCommand {
    fn apply(self, world: &mut World) {
        #[cfg(feature = "debug_log")]
        info!("Continuing ink sequence");
        let Some(mut story) = world.get_non_send_resource_mut::<Story>() else {
            error!(
                "Failed to load state: Story resource not found. Did you forget to insert the InkProject resource?",
            );
            return;
        };

        if !story.can_continue() {
            #[cfg(feature = "debug_log")]
            info!("Continuing: No more content available");
            let choices = story.get_current_choices();

            let choices: Vec<ChoiceItem> = choices.iter().map(|c| c.as_ref().into()).collect();

            if choices.is_empty() {
                #[cfg(feature = "debug_log")]
                info!("Continuing: No choices available");
                world.trigger(SequenceEnd);
                return;
            }

            #[cfg(feature = "debug_log")]
            info!("Continuing: Delivering {} choices", choices.len());

            world.trigger(DeliverChoices::new(choices));
            world.trigger(InkStateChanged);
            return;
        }

        match story.cont() {
            Ok(text) => {
                let tags = story.get_current_tags().unwrap_or_default();

                #[cfg(feature = "debug_log")]
                {
                    info!("Continuing: Delivering line - {}", text);
                    info!("Continuing: Tags - {:?}", tags);
                }
                world.trigger(DeliverLine::new(text, tags));
            }
            Err(err) => {
                error!("Error continuing story: {}", err);
                return;
            }
        };
        world.trigger(InkStateChanged);
    }
}

/// Helper trait for adding `ContinueSequenceCommand` to a `Commands` instance.
pub trait ContinueSequenceCommandsExt {
    /// Attempts to advance the story to the next step in the current sequence.
    fn ink_continue_sequence(&mut self) -> &mut Self;
}

impl<'w, 's> ContinueSequenceCommandsExt for Commands<'w, 's> {
    fn ink_continue_sequence(&mut self) -> &mut Self {
        self.queue(ContinueSequenceCommand::new());
        self
    }
}
