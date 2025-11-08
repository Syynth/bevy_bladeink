use bevy::prelude::*;
use bladeink::story::Story;

use crate::ink::InkState;

/// Represents a command to load the ink story state, and variables.
pub(crate) struct LoadStateCommand {
    state: InkState,
}

impl LoadStateCommand {
    /// Creates a new `LoadStateCommand` with the given state.
    pub(crate) fn new(state: InkState) -> Self {
        LoadStateCommand { state }
    }
}

impl Command for LoadStateCommand {
    fn apply(self, world: &mut World) {
        let Some(mut story) = world.get_non_send_resource_mut::<Story>() else {
            error!(
                "Failed to load state: Story resource not found. Did you forget to insert the InkProject resource?",
            );
            return;
        };
        match story.load_state(&self.state.serialized_state) {
            Ok(_) => (),
            Err(err) => {
                warn!("Failed to load state: {err}");
                warn!("- contents: {:?}", self.state);
            }
        };
    }
}

/// Helper trait for adding `LoadStateCommand` to `Commands`.
pub trait LoadStateCommandsExt {
    fn load_ink_state(&mut self, state: InkState) -> &mut Self;
}

impl LoadStateCommandsExt for Commands<'_, '_> {
    fn load_ink_state(&mut self, state: InkState) -> &mut Self {
        self.queue(LoadStateCommand::new(state));
        self
    }
}
