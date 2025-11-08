use bevy::prelude::*;
use bevy_bladeink::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InkPlugin)
        .insert_resource(InkStory::new("basic_story.ink.json"))
        .run();
}
