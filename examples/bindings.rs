use bevy::{color::palettes::tailwind::GRAY_400, prelude::*};
use bevy_bladeink::{prelude::*, resources::InkVariables};
use bladeink::value_type::ValueType;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InkPlugin)
        .insert_resource(InkStory::new("ink/bindings.ink.json"))
        .bind_ink_function::<SetTextColor>("set_text_color")
        .add_systems(Startup, setup)
        .add_systems(Update, check_input)
        .add_observer(on_set_text_color)
        .add_observer(on_story_ready)
        .add_observer(on_deliver_line)
        .run();
}

const MARGIN: Val = Val::Px(12.);
const FULL: Val = Val::Percent(100.);

#[derive(Component)]
struct TextDisplay;

#[derive(Component)]
struct InstructionsDisplay;

#[derive(Event, Clone)]
struct SetTextColor(Color);

impl InkBindingDefinition for SetTextColor {
    type Event = SetTextColor;

    fn try_parse_event(args: &[ValueType]) -> Result<Self::Event, InkBindingError> {
        match args {
            [] => Err(InkBindingError::ArgumentsRequired),
            [ValueType::String(color)] => Srgba::hex(&color.string)
                .map(|c| SetTextColor(c.into()))
                .map_err(|_| InkBindingError::InvalidArguments),
            [_] => Err(InkBindingError::InvalidArguments),
            _ => Err(InkBindingError::TooManyArguments),
        }
    }
}

fn on_set_text_color(
    text_color: On<SetTextColor>,
    mut commands: Commands,
    q_text: Single<Entity, With<TextDisplay>>,
) {
    commands.entity(*q_text).insert(TextColor(text_color.0));
}

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

fn check_input(mut commands: Commands, key: Res<ButtonInput<KeyCode>>) {
    if key.just_pressed(KeyCode::Space) {
        commands.ink_continue_sequence();
    }
}

fn on_story_ready(
    _: On<StoryReady>,
    mut commands: Commands,
    mut q_text: Single<&mut Text, With<TextDisplay>>,
) {
    commands.ink_begin_sequence("start");
    commands.ink_track_variable("color_change_count");
    q_text.0 = "Binding demo.\n".to_string();
}

fn on_deliver_line(
    line: On<DeliverLine>,
    mut q_text: Single<&mut Text, With<TextDisplay>>,
    ink_vars: Res<InkVariables>,
) {
    let mut next_line = line.text.clone();

    if let Some(count) = ink_vars.get_int("color_change_count") {
        next_line.push_str(&format!("\nColor changed {} times.", count));
    }

    q_text.0 = next_line;
}
