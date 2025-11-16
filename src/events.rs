use bevy::prelude::*;

use crate::ink::{ChoiceItem, InkState};

#[derive(Event, Clone, Debug)]
pub struct StoryReady;

#[derive(Event, Debug)]
pub(crate) struct InkStateChanged;

/// Published when the state of the ink story changes. This is a bit of a hack
/// for now, better options for persistence will be implemented in the future.
#[derive(Event, Clone, Debug)]
pub struct InkStateUpdate(pub InkState);

/// After a successful `BeginSequence` command is sent, this event is emitted.
#[derive(Event, Clone, Debug)]
pub struct SequenceBegin(pub String);

/// After a successful `ContinueSequenceCommand` is issued, if a new line of
/// content is produced, this event will be emitted containing the new line.
#[derive(Event, Clone, Debug)]
pub struct SequenceEnd;

/// After a successful `ContinueSequenceCommand` is issued, if a new line of
/// content is produced, this event will be emitted containing the new line.
#[derive(Event, Clone, Debug)]
pub struct DeliverLine {
    pub text: String,
    pub tags: Vec<String>,
}

impl DeliverLine {
    pub fn new(text: String, tags: Vec<String>) -> Self {
        Self { text, tags }
    }
}

/// After a successful `ContinueSequenceCommand` is issued, if a no new content
/// is produced because a choice is required, this event will be emitted once,
/// containing the available choices.
#[derive(Event, Clone, Debug)]
pub struct DeliverChoices {
    pub choices: Vec<ChoiceItem>,
}

impl DeliverChoices {
    pub fn new(choices: Vec<ChoiceItem>) -> Self {
        Self { choices }
    }
}
