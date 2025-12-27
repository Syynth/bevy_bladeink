use bevy::prelude::*;

#[derive(Component, Debug, Reflect, Default)]
pub struct InkElement;

#[derive(Component, Debug, Reflect, Default)]
#[require(InkElement)]
pub struct InkDialogueRoot;

#[derive(Component, Debug, Reflect, Default)]
#[require(InkElement)]
pub struct InkDialogueBackdrop;

#[derive(Component, Debug, Reflect, Default)]
#[require(InkElement)]
pub struct InkDialogueCard;

#[derive(Component, Debug, Reflect, Default)]
#[require(InkElement)]
pub struct InkDialogueTitle;

#[derive(Component, Debug, Reflect, Default)]
#[require(InkElement)]
pub struct InkDialogueContents;

#[derive(Component, Debug, Reflect, Default)]
#[require(InkElement)]
pub struct InkDialogueIndicator;

#[derive(Component)]
#[relationship_target(relationship = InkUiOf)]
pub struct InkUiParent(Vec<Entity>);

#[derive(Component)]
#[relationship(relationship_target = InkUiParent)]
pub struct InkUiOf(pub Entity);
