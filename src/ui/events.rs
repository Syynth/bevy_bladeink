use bevy::prelude::*;

/// After the Ink UI has been constructed,
#[derive(Event)]
pub struct InkUiConstruction {
    pub root: Entity,
    pub backdrop: Entity,
    pub card: Entity,
    pub title: Entity,
    pub content: Entity,
}

#[derive(Event)]
pub struct InkUiReady;

#[derive(Event)]
pub struct InkUiLeft;
