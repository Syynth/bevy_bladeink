use bevy::{platform::collections::HashMap, reflect::Reflect};

use crate::ink::InkValue;

#[derive(Debug, Clone, Reflect)]
pub struct InkState {
    pub(crate) serialized_state: String,
    pub(crate) tracked_variables: HashMap<String, InkValue>,
}
