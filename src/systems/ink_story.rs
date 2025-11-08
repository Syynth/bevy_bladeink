use bevy::prelude::*;

use crate::{
    assets::StoryJson,
    resources::{InkAssetReady, InkStory},
};

pub(crate) fn load_ink_project(
    asset_server: Res<AssetServer>,
    ink_project: Option<ResMut<InkStory>>,
) {
    let Some(mut ink_project) = ink_project else {
        return;
    };

    if ink_project.handle().is_none() {
        let path = ink_project.asset_path().to_string();
        ink_project.set_handle(asset_server.load(path));
    }
}

pub(crate) fn process_story_json_events(
    mut commands: Commands,
    mut asset_messages: MessageReader<AssetEvent<StoryJson>>,
    ink_project: Option<Res<InkStory>>,
) {
    let Some(ink_project) = ink_project else {
        return;
    };

    for message in asset_messages.read() {
        match message {
            AssetEvent::Modified { id } | AssetEvent::LoadedWithDependencies { id } => {
                if let Some(handle) = ink_project.handle()
                    && handle.id() == *id
                {
                    commands.insert_resource(InkAssetReady(handle.clone()));
                }
            }
            AssetEvent::Added { .. } | AssetEvent::Removed { .. } | AssetEvent::Unused { .. } => {}
        }
    }
}
