use std::{cell::RefCell, rc::Rc};

use bevy::prelude::*;
use bladeink::story::errors::{ErrorHandler, ErrorType};

/// A custom error handler for `BladeInk` stories.
pub struct InkErrorHandler;

impl InkErrorHandler {
    pub(crate) fn boxed() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self))
    }
}

impl ErrorHandler for InkErrorHandler {
    fn error(&mut self, message: &str, error_type: ErrorType) {
        if error_type == ErrorType::Warning {
            warn!("Ink warning: {}", message);
        } else {
            error!("Ink error: {}", message);
        }
    }
}
