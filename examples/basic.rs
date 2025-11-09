use bevy::{color::palettes::tailwind::GRAY_400, prelude::*};
use bevy_bladeink::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InkPlugin)
        .insert_resource(InkStory::new("ink/TheIntercept.ink.json"))
        .add_systems(Startup, setup)
        .add_systems(Update, select_choice)
        .add_observer(on_story_ready)
        .add_observer(on_deliver_line)
        .add_observer(on_deliver_choices)
        .run();
}

const MARGIN: Val = Val::Px(12.);
const FULL: Val = Val::Percent(100.);

#[derive(Component)]
struct TextDisplay;

#[derive(Component)]
struct InstructionsDisplay;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Name::new("UI Root"),
        Node {
            width: FULL,
            height: FULL,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::all(MARGIN),
            row_gap: MARGIN,
            ..default()
        },
        children![
            (
                Node {
                    flex_grow: 1.0,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: FULL,
                    ..default()
                },
                children![(
                    TextDisplay,
                    TextColor(Color::WHITE),
                    Text::new("Loading story..."),
                    TextFont {
                        font_size: 24.0,
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                        ..default()
                    }
                )]
            ),
            (
                InstructionsDisplay,
                TextColor(GRAY_400.into()),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                Text::new("Press space to continue")
            )
        ],
    ));
}

fn select_choice(mut commands: Commands, key: Res<ButtonInput<KeyCode>>) {
    if key.just_pressed(KeyCode::Space) {
        commands.ink_continue_sequence();
    }

    if key.just_pressed(KeyCode::Digit1) {
        commands.ink_select_choice(0);
    }
    if key.just_pressed(KeyCode::Digit2) {
        commands.ink_select_choice(1);
    }
    if key.just_pressed(KeyCode::Digit3) {
        commands.ink_select_choice(2);
    }
    if key.just_pressed(KeyCode::Digit4) {
        commands.ink_select_choice(3);
    }
    if key.just_pressed(KeyCode::Digit5) {
        commands.ink_select_choice(4);
    }
    if key.just_pressed(KeyCode::Digit6) {
        commands.ink_select_choice(5);
    }
    if key.just_pressed(KeyCode::Digit7) {
        commands.ink_select_choice(6);
    }
    if key.just_pressed(KeyCode::Digit8) {
        commands.ink_select_choice(7);
    }
    if key.just_pressed(KeyCode::Digit9) {
        commands.ink_select_choice(8);
    }
}

// begin dialogue sequence
fn on_story_ready(
    _: On<StoryReady>,
    mut commands: Commands,
    mut q_text: Single<&mut Text, With<TextDisplay>>,
) {
    // start is the first knot defined in The Intercept
    commands.ink_begin_sequence("start");
    q_text.0 = "The Intercept.\n".to_string();
}

// update UI with new line of dialogue
fn on_deliver_line(
    line: On<DeliverLine>,
    mut q_text: Single<&mut Text, (With<TextDisplay>, Without<InstructionsDisplay>)>,
    mut instructions: Single<&mut Text, (With<InstructionsDisplay>, Without<TextDisplay>)>,
) {
    q_text.0 = line.text.clone();
    instructions.0 = "Press space to continue".to_string();
}

// update UI with new choices
fn on_deliver_choices(
    choices: On<DeliverChoices>,
    mut q_text: Single<&mut Text, (With<TextDisplay>, Without<InstructionsDisplay>)>,
    mut instructions: Single<&mut Text, (With<InstructionsDisplay>, Without<TextDisplay>)>,
) {
    let mut text = String::new();
    for (index, choice) in choices.choices.iter().enumerate() {
        text.push_str(&format!("{}: {}\n", index + 1, choice.text()));
    }
    q_text.0 = text;
    instructions.0 = "Press number to select choice".to_string();
}
