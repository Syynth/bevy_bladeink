use bevy::{platform::collections::HashMap, prelude::*};

use crate::ink::InkValue;

#[derive(Resource, Default)]
pub struct InkVariables {
    pub(crate) tracked_variables: HashMap<String, InkValue>,
}

impl InkVariables {
    pub fn get_string(&self, variable_name: &str) -> Option<&String> {
        self.tracked_variables
            .get(variable_name)
            .and_then(|value| value.get_string())
    }

    pub fn get_bool(&self, variable_name: &str) -> Option<bool> {
        self.tracked_variables
            .get(variable_name)
            .and_then(InkValue::get_bool)
    }

    pub fn get_int(&self, variable_name: &str) -> Option<i32> {
        self.tracked_variables
            .get(variable_name)
            .and_then(InkValue::get_int)
    }

    pub fn get_float(&self, variable_name: &str) -> Option<f32> {
        self.tracked_variables
            .get(variable_name)
            .and_then(InkValue::get_float)
    }
}
