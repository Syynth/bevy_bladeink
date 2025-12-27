use bevy::prelude::*;
use bladeink::story::Story;

use crate::events::SequenceBegin;

/// Represents a command to start an ink sequence.
pub(crate) struct BeginSequenceCommand {
    sequence: String,
}

impl BeginSequenceCommand {
    /// Creates a new `BeginSequenceCommand` with the given sequence.
    pub(crate) fn path(sequence: impl Into<String>) -> Self {
        BeginSequenceCommand {
            sequence: sequence.into(),
        }
    }
}

impl Command for BeginSequenceCommand {
    fn apply(self, world: &mut World) {
        #[cfg(feature = "debug_log")]
        info!("Starting ink sequence '{}'", self.sequence);
        let Some(mut story) = world.get_non_send_resource_mut::<Story>() else {
            error!(
                "Failed to start ink sequence '{}': Story resource not found. Did you forget to insert the InkProject resource?",
                self.sequence
            );
            return;
        };
        match story.choose_path_string(&self.sequence, true, None) {
            Ok(_) => {
                world.trigger(SequenceBegin(self.sequence));
            }
            Err(err) => {
                warn!("Failed to start ink sequence '{}': {}", self.sequence, err);
            }
        };
    }
}

/// Helper trait for adding `BeginSequenceCommand` to a `Commands` instance.
pub trait BeginSequenceCommandsExt {
    /// Taken from bladeink docs:
    /// The path string is a dot-separated path as used internally by the
    /// engine. These examples should work:
    ///
    /// ```ink
    ///    myKnot
    ///    myKnot.myStitch
    /// ```
    ///
    /// Note however that this won't necessarily work:
    ///
    /// ```ink
    ///    myKnot.myStitch.myLabelledChoice
    /// ```
    ///
    /// ...because of the way that content is nested within a weave
    /// structure.
    fn ink_begin_sequence(&mut self, sequence: impl Into<String>) -> &mut Self;
}

impl<'w, 's> BeginSequenceCommandsExt for Commands<'w, 's> {
    fn ink_begin_sequence(&mut self, sequence: impl Into<String>) -> &mut Self {
        self.queue(BeginSequenceCommand::path(sequence));
        self
    }
}
