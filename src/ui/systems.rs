use bevy::prelude::*;

use crate::{
    events::SequenceBegin,
    ui::{
        components::{InkDialogueBackdrop, InkDialogueCard, InkDialogueRoot, InkUiOf},
        events::InkUiConstruction,
    },
};

pub(crate) fn on_begin_sequence(
    _seq: On<SequenceBegin>,
    mut commands: Commands,
    q_existing: Query<Entity, With<InkDialogueRoot>>,
) {
    #[cfg(feature = "debug_log")]
    {
        info!("Sequence began: {} - spawning UI", _seq.0);
    }
    for existing in q_existing {
        warn!("Found existing dialogue root ({existing:?}), despawning...");
        commands.entity(existing).despawn();
    }

    let root = commands
        .spawn((Name::new("Ink UI Dialogue Root"), InkDialogueRoot))
        .id();

    let backdrop = commands
        .spawn((
            Name::new("Ink UI Dialogue Backdrop"),
            ChildOf(root),
            InkUiOf(root),
            ZIndex(0),
            InkDialogueBackdrop,
        ))
        .id();

    let card = commands
        .spawn((
            Name::new("Ink UI Dialogue Card"),
            ChildOf(root),
            InkUiOf(root),
            InkDialogueCard,
        ))
        .id();

    let title = commands
        .spawn((
            Name::new("Ink UI Dialogue Title"),
            ChildOf(card),
            InkUiOf(root),
        ))
        .id();

    let content = commands
        .spawn((
            Name::new("Ink UI Dialogue Content"),
            ChildOf(card),
            InkUiOf(root),
        ))
        .id();

    commands.trigger(InkUiConstruction {
        root,
        backdrop,
        card,
        title,
        content,
    });
}
