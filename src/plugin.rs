use bevy::prelude::*;
use bevy_crossbeam_event::CrossbeamEventApp;

use crate::{
    InkSystems,
    assets::{InkStoryJsonLoader, StoryJson},
    commands::{VariableUpdated, on_variable_updated},
    ink::InkBindingMap,
    resources::InkVariables,
    systems::*,
};

/// Plugin for integrating Ink into Bevy.
pub struct InkPlugin;

impl Plugin for InkPlugin {
    fn build(&self, app: &mut App) {
        app.add_crossbeam_event::<VariableUpdated>()
            .init_resource::<InkVariables>()
            .add_observer(on_variable_updated)
            .world_mut()
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
