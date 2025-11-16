use bevy::{color::palettes::tailwind::*, prelude::*};
use bevy_bladeink::{prelude::*, ui::events::InkUiConstruction};

const MARGIN: Val = Val::Px(12.);
const NONE: Val = Val::Px(0.);
const FULL: Val = Val::Percent(100.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((InkPlugin, InkUiPlugin))
        .insert_resource(InkStory::new("ink/TheIntercept.ink.json"))
        .add_systems(Startup, setup)
        .add_observer(on_story_ready)
        .add_observer(on_ink_ui_construction)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d));
}

fn on_story_ready(_: On<StoryReady>, mut commands: Commands) {
    commands.ink_begin_sequence("start");
}

fn on_ink_ui_construction(elements: On<InkUiConstruction>, mut commands: Commands) {
    info!("on_ink_ui_construction called");
    commands
        .entity(elements.root)
        .insert(dialogue_ui_container());
    commands
        .entity(elements.backdrop)
        .insert(dialogue_backdrop());
    commands.entity(elements.card).insert(dialogue_card());
}

fn dialogue_ui_container() -> impl Bundle {
    Node {
        width: FULL,
        height: FULL,
        position_type: PositionType::Absolute,
        justify_content: JustifyContent::Stretch,
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Stretch,
        padding: UiRect::all(MARGIN),
        row_gap: MARGIN,
        ..default()
    }
}

fn dialogue_backdrop() -> impl Bundle {
    (
        Node {
            top: NONE,
            left: NONE,
            right: NONE,
            bottom: NONE,
            position_type: PositionType::Absolute,
            ..default()
        },
        ZIndex(0),
        BackgroundColor(RED_400.with_alpha(0.5).into()),
    )
}

fn dialogue_card() -> impl Bundle {
    (
        Node {
            align_self: AlignSelf::End,
            align_items: AlignItems::Stretch,
            padding: UiRect::all(MARGIN),
            row_gap: MARGIN,
            flex_direction: FlexDirection::Column,
            flex_grow: 1.0,
            height: percent(20.0),
            ..default()
        },
        BackgroundColor(BLUE_800.into()),
        Outline::new(MARGIN, NONE, Color::WHITE),
        ZIndex(1),
    )
}
