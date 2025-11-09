//! ### Introduction
//!
//! `bevy_bladeink` integrates [`bladeink`](https://crates.io/crates/bladeink)
//! crate with bevy. `bladeink` appears to be a direct port of the C#
//! Ink.Runtime library. As a result, it uses `Rc<RefCell<..>>` aggressively
//! internally, which means Ink `Story`s are both `!Send` as well as `!Sync`.
//!
//! To get around this, we assume a single `Story` will be
//! loaded for the entire application, stored in a `NonSend` resource, and all
//! communication will occur through events and messages.
//!
//! ### Primary features & goals
//! - Loading and managing .ink.json assets into `Story` resource
//! - Providing a mechanism for binding external functions to the Ink script
//! - Allowing the application to view and manage `Variables` in the Ink script
//! - Provide an unstyled UI controller to allow managing dialogue and choices
//! - Provide commands/events to enable easy integration with Bevy's ECS
//!
//! ### Possible future goals
//! - Potentially support rich text processing/formatting
//! - TBD regarding what level of responsibility this crate should have w.r.t.
//!   managing "save" data outside of making it easy to integrate with a
//!   different storage mechanism.
//!
//! ### Non-goals
//! - Allowing multiple `Story`s
//! - Supporting a workflow based on bevy's Asset system.
//! - Providing an out-of-the-box solution for UI

use bevy::prelude::*;

mod assets;
mod components;
mod plugin;
mod systems;

/// Commands for instructing the Ink story to change state.
pub mod commands;
/// Events that can be emitted by the Ink story.
pub mod events;
/// Core ink stuff
pub mod ink;
/// Pre-defined modules and types for easy import.
pub mod prelude;
/// Bevy resources for managing Ink stories and their associated data.
pub mod resources;

#[cfg(feature = "ui")]
/// A collection of UI components and systems for managing Ink dialogue, intended to be styled by the consumer.
pub mod ui;

/// System sets for `bevy_bladeink`
#[derive(SystemSet, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum InkSystems {
    /// Asset handling systems, intended to load and manage Ink story assets.
    AssetHandling,

    /// When commands are received from the application, this system will
    /// process them and update the Ink story accordingly.
    HandleCommands,

    /// When the Ink story needs to be rendered, this system will
    /// update the UI accordingly.
    Ui,
}
