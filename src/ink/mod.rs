//! ## Ink Bindings
//!
//! It is possible to define external functions inside ink scripts, which are
//! usually used to interact with the game engine. These functions are often
//! treated as commands, to do things like play audio or animations.
//!
//!
//! ```rust
//! use bevy::prelude::*;
//! use bladeink::value_type::ValueType;
//! use bevy_bladeink::*;
//! use bevy_bladeink::AddInkBindingApp;
//!
//! #[derive(Event, Clone)]
//! struct DespawnEntity(pub String);
//!
//! impl InkBindingDefinition for DespawnEntity {
//!     type Event = Self;
//!
//!     fn try_parse_event(args: Vec<ValueType>) -> Result<Self::Event, InkBindingError> {
//!         match &args[..] {
//!             [] => Err(InkBindingError::ArgumentsRequired),
//!             [ValueType::String(entity_id)] => Ok(DespawnEntity(entity_id.string.clone())),
//!             [_] => Err(InkBindingError::InvalidArguments),
//!             _ => Err(InkBindingError::TooManyArguments),
//!         }
//!     }
//! }
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(InkPlugin)
//!         .bind_ink_function::<DespawnEntity>("despawn_entity")
//!         .add_observer(handle_despawn_entity)
//!         .run();
//! }
//!
//! fn handle_despawn_entity(despawn: On<DespawnEntity>, q_entities: Query<Entity>) {
//!    // find the correct entity and despawn it
//! }

mod bindings;
mod error;
mod ink_value;
mod state;
mod story;

pub use bindings::*;
pub use error::*;
pub use ink_value::*;
pub use state::*;
pub(crate) use story::*;
