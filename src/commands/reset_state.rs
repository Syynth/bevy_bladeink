use bevy::prelude::*;
use bladeink::story::Story;

/// Represents a command to start an ink sequence.
#[derive(Default)]
pub(crate) struct ResetStateCommand;

impl ResetStateCommand {
    /// Creates a new `ResetStateCommand` with the given state.
    pub(crate) fn new() -> Self {
        ResetStateCommand
    }
}

impl Command for ResetStateCommand {
    fn apply(self, world: &mut World) {
        let Some(mut story) = world.get_non_send_resource_mut::<Story>() else {
            error!(
                "Failed to load state: Story resource not found. Did you forget to insert the InkProject resource?",
            );
            return;
        };
        match story.reset_state() {
            Ok(_) => (),
            Err(err) => {
                warn!("Failed to reset state: {err}");
            }
        };
    }
}

/// Helper trait for adding `LoadStateCommand` to `Commands`.
pub trait ResetStateCommandsExt {
    fn ink_reset_state(&mut self) -> &mut Self;
}

impl ResetStateCommandsExt for Commands<'_, '_> {
    fn ink_reset_state(&mut self) -> &mut Self {
        self.queue(ResetStateCommand::new());
        self
    }
}
