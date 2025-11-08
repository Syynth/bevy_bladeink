use bevy::prelude::*;

use crate::{
    InkSystems,
    assets::{InkStoryJsonLoader, StoryJson},
    ink::InkBindingMap,
    systems::*,
};

/// Plugin for integrating Ink into Bevy.
pub struct InkPlugin;

impl Plugin for InkPlugin {
    fn build(&self, app: &mut App) {
        app.world_mut()
            .insert_non_send_resource(InkBindingMap::default());
        app.init_asset::<StoryJson>()
            .register_asset_loader(InkStoryJsonLoader);

        app.add_systems(
            Update,
            (
                load_ink_project,
                process_story_json_events,
                parse_story_asset,
            )
                .chain()
                .in_set(InkSystems::AssetHandling),
        );
    }
}
