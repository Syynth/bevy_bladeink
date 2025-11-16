use bevy::prelude::*;
use bladeink::story::Story;

use crate::{
    events::{InkStateChanged, InkStateUpdate},
    ink::InkState,
    prelude::InkVariables,
};

pub(crate) fn on_state_changed(
    _: On<InkStateChanged>,
    mut commands: Commands,
    mut story: NonSendMut<Story>,
    ink_vars: Res<InkVariables>,
) {
    let Ok(next_state) = InkState::from_story(&mut story, &ink_vars) else {
        return;
    };
    commands.trigger(InkStateUpdate(next_state));
}
