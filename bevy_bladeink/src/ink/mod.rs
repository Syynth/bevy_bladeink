//! ## Ink Bindings
//!
//! It is possible to define external functions inside ink scripts, which are
//! usually used to interact with the game engine. These functions are often
//! treated as commands, to do things like play audio or animations.
//!
//! For basic types (String, i32, f32, bool), you can use the derive macro:
//!
//! ```rust
//! use bevy::prelude::*;
//! use bevy_bladeink::prelude::*;
//! # use bladeink::value_type::ValueType; // Needed by the derive macro
//!
//! // Simple derive for basic types
//! #[derive(Event, Clone, InkBinding)]
//! struct DespawnEntity(pub String);
//! # fn main() {}
//! ```
//!
//! For custom parsing logic, implement the trait manually:
//!
//! ```rust
//! use bevy::prelude::*;
//! use bladeink::value_type::ValueType;
//! use bevy_bladeink::prelude::*;
//!
//! #[derive(Event, Clone)]
//! struct CustomEvent(pub String);
//!
//! impl InkBindingDefinition for CustomEvent {
//!     type Event = Self;
//!
//!     fn try_parse_event(args: &[ValueType]) -> Result<Self::Event, InkBindingError> {
//!         match args {
//!             [] => Err(InkBindingError::ArgumentsRequired),
//!             [ValueType::String(entity_id)] => Ok(CustomEvent(entity_id.string.clone())),
//!             [_] => Err(InkBindingError::InvalidArguments),
//!             _ => Err(InkBindingError::TooManyArguments),
//!         }
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! Then register it with your app:
//!
//! ```rust,no_run
//! # use bevy::prelude::*;
//! # use bevy_bladeink::prelude::*;
//! # use bladeink::value_type::ValueType; // Needed by the derive macro
//! # #[derive(Event, Clone, InkBinding)]
//! # struct DespawnEntity(pub String);
//! # fn handle_despawn_entity(despawn: On<DespawnEntity>, q_entities: Query<Entity>) {}
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(InkPlugin)
//!         .bind_ink_function::<DespawnEntity>("despawn_entity")
//!         .add_observer(handle_despawn_entity)
//!         .run();
//! }

mod bindings;
mod choice;
mod error;
mod ink_value;
mod state;
mod story;

pub use bindings::*;
pub use choice::*;
pub use error::*;
pub use ink_value::*;
pub use state::*;
pub(crate) use story::*;
