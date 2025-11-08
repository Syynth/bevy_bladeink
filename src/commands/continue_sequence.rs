use bevy::{ecs::system::RunSystemOnce, prelude::*};
use bladeink::story::Story;

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
        let _ = world.run_system_once(continue_sequence);
    }
}

fn continue_sequence(_commands: Commands, mut story: If<NonSendMut<Story>>) {
    if !story.can_continue() {
        let choices = story.get_current_choices();

        let choices: Vec<_> = choices
            .iter()
            // .map(|c| Dialogue::new(c.text.clone()))
            .collect();

        if choices.is_empty() {
            // commands.trigger(CompleteSequence);
            return;
        }

        // commands.trigger(ChoiceDelivery(choices));
        return;
    }
    match story.cont() {
        Ok(text) => {
            info!("continued successfully, publishing line delivery: {text}");
            // commands.trigger(LineDelivery(Dialogue::new(text)));
        }
        Err(err) => {
            error!("Error continuing story: {}", err);
        }
    };
    if let Ok(_story_state) = story.save_state() {
        // commands.trigger(Save(story_state));
    }
}

/// Helper trait for adding `ContinueSequenceCommand` to a `Commands` instance.
pub trait ContinueSequenceCommandsExt {
    /// Attempts to advance the story to the next step in the current sequence.
    fn continue_sequence(&mut self, sequence: impl Into<String>) -> &mut Self;
}

impl<'w, 's> ContinueSequenceCommandsExt for Commands<'w, 's> {
    fn continue_sequence(&mut self, _sequence: impl Into<String>) -> &mut Self {
        self.queue(ContinueSequenceCommand::new());
        self
    }
}
