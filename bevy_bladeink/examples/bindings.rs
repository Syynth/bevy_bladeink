use bevy::{color::palettes::tailwind::GRAY_400, prelude::*};
use bevy_bladeink::prelude::*;
use bladeink::value_type::ValueType;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InkPlugin)
        .insert_resource(InkStory::new("ink/bindings.ink.json"))
        .bind_ink_function::<SetTextColor>("set_text_color")
        .bind_ink_function::<NoArgsEvent>("no_args_event")
        .bind_ink_function::<SimpleEvent>("simple_event")
        .bind_ink_function::<TupleEvent>("tuple_event")
        .add_systems(Startup, setup)
        .add_systems(Update, update)
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
struct CounterDisplay;

#[derive(Component)]
struct InstructionsDisplay;

// Example of MANUAL implementation for custom parsing logic.
// This is used when you need to do complex parsing (like hex color parsing).
// For simple types, use the derive macro instead (see examples below).
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

// Examples of DERIVED implementations for basic types.
// The derive macro automatically generates the try_parse_event implementation
// for structs with basic field types (String, i32, f32, bool).

// Example 1: Simple event with basic types
#[derive(Event, Clone, InkBinding)]
pub struct SimpleEvent {
    pub message: String,
    pub count: i32,
}

// Example 2: Tuple struct
#[derive(Event, Clone, InkBinding)]
pub struct TupleEvent(pub String);

// Example 3: Unit struct (no arguments)
#[derive(Event, Clone, InkBinding)]
pub struct NoArgsEvent;

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
                CounterDisplay,
                TextColor(GRAY_400.into()),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                Text::new("Color changed 0 times.")
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

fn update(
    mut commands: Commands,
    key: Res<ButtonInput<KeyCode>>,
    mut q_text: Single<&mut Text, With<CounterDisplay>>,
    ink_vars: If<Res<InkVariables>>,
) {
    if key.just_pressed(KeyCode::Space) {
        commands.ink_continue_sequence();
    }

    let count = ink_vars.get_int("color_change_count").unwrap_or(0);
    q_text.0 = format!("Color changed {} times.", count);
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

fn on_deliver_line(line: On<DeliverLine>, mut q_text: Single<&mut Text, With<TextDisplay>>) {
    q_text.0 = line.text.clone();
}
