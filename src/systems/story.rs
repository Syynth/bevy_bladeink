use bevy::prelude::*;

use crate::{
    assets::StoryJson,
    ink::{InkBindingMap, create_story},
    resources::{InkAssetReady, InkStory},
};

pub(crate) fn parse_story_asset(world: &mut World) {
    let Some(ink_project) = world.get_resource::<InkStory>() else {
        return;
    };

    let Some(InkAssetReady(story_handle)) = world.get_resource::<InkAssetReady>() else {
        return;
    };
    let story_assets = world.resource::<Assets<StoryJson>>();
    let Some(story_json) = story_assets.get(story_handle) else {
        return;
    };

    let binding_defs = world.get_non_send_resource::<InkBindingMap>().expect(
        "Failed to get binding definitions, did you forget to initialize the bevy_bladeink plugin?",
    );

    let Some(story) = create_story(&story_json.text, binding_defs, ink_project.state()) else {
        return;
    };

    world.insert_non_send_resource(story);
}
