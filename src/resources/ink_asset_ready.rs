use bevy::prelude::*;

use crate::assets::StoryJson;

#[derive(Resource)]
pub struct InkAssetReady(pub Handle<StoryJson>);
