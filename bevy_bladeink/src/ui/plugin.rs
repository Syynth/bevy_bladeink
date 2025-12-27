use bevy::prelude::*;

use super::systems::*;

pub struct InkUiPlugin;

impl Plugin for InkUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_begin_sequence);
    }
}
