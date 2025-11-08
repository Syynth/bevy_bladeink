use bevy::prelude::*;
use bladeink::story::Story;

use super::{InkBindingMap, InkErrorHandler};

pub(crate) fn create_story(
    story_text: impl AsRef<str>,
    binding_defs: &InkBindingMap,
    existing_state: &Option<String>,
) -> Option<Story> {
    let mut story = match Story::new(story_text.as_ref()) {
        Ok(story) => story,
        Err(err) => {
            info!("Failed to parse story: {}", err);
            return None;
        }
    };

    story.set_error_handler(InkErrorHandler::boxed());

    for (name, binding_def) in binding_defs.iter() {
        match story.bind_external_function(name, binding_def.clone(), false) {
            Ok(_) => {}
            Err(err) => {
                info!("Failed to bind ink command '{name}': {err}");
            }
        }
    }

    let should_init = if let Some(existing_state) = existing_state {
        match story
            .reset_state()
            .and_then(|_| story.load_state(existing_state))
        {
            Ok(_) => false,
            Err(err) => {
                info!("Failed to load existing state: {err}");
                true
            }
        }
    } else {
        true
    };

    if should_init && story.can_continue() {
        match story.continue_maximally() {
            Ok(_) => {}
            Err(err) => {
                info!("Failed to continue story: {}", err);
            }
        }
    }

    Some(story)
}
