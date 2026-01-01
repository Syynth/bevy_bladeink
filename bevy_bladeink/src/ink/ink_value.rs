//! Ink value, intended to be mostly compatible with the native
//! `bladeink::value_type::ValueType`, but with the value semantics necessary
//! for use in bevy.

use bevy::prelude::*;
use bladeink::value_type::ValueType;
use serde::{Deserialize, Serialize};

/// An Ink value, tagged with its type.
#[repr(u8)]
#[derive(Clone, Debug, Reflect, Serialize, Deserialize)]
pub enum InkValue {
    Bool(bool),
    Int(i32),
    Float(f32),
    /// An Ink list value.
    List,
    /// Ink string, constructed with [`new_string`](ValueType::new::<&str>)
    String(String),
    /// Reference to an Ink divert.
    DivertTarget,
    /// Reference to an Ink variable.
    VariablePointer,
}

impl InkValue {
    pub fn get_bool(&self) -> Option<bool> {
        match self {
            InkValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn get_int(&self) -> Option<i32> {
        match self {
            InkValue::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn get_float(&self) -> Option<f32> {
        match self {
            InkValue::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn get_string(&self) -> Option<&String> {
        match self {
            InkValue::String(s) => Some(s),
            _ => None,
        }
    }
}

impl From<bool> for InkValue {
    fn from(b: bool) -> Self {
        InkValue::Bool(b)
    }
}

impl From<f32> for InkValue {
    fn from(f: f32) -> Self {
        InkValue::Float(f)
    }
}

impl From<String> for InkValue {
    fn from(s: String) -> Self {
        InkValue::String(s)
    }
}

impl From<&str> for InkValue {
    fn from(s: &str) -> Self {
        InkValue::String(s.to_string())
    }
}

impl From<i32> for InkValue {
    fn from(i: i32) -> Self {
        InkValue::Int(i)
    }
}

impl From<ValueType> for InkValue {
    fn from(value_type: ValueType) -> Self {
        match value_type {
            ValueType::Bool(b) => InkValue::Bool(b),
            ValueType::Int(i) => InkValue::Int(i),
            ValueType::Float(f) => InkValue::Float(f),
            ValueType::String(s) => InkValue::String(s.string),
            ValueType::List(_) => InkValue::List,
            ValueType::DivertTarget(_) => InkValue::DivertTarget,
            ValueType::VariablePointer(_) => InkValue::VariablePointer,
        }
    }
}

impl From<&InkValue> for ValueType {
    fn from(ink_value: &InkValue) -> Self {
        match ink_value {
            InkValue::Bool(b) => ValueType::Bool(*b),
            InkValue::Int(i) => ValueType::Int(*i),
            InkValue::Float(f) => ValueType::Float(*f),
            InkValue::String(s) => ValueType::from(s.as_str()),
            InkValue::List => todo!(),
            InkValue::DivertTarget => todo!(),
            InkValue::VariablePointer => todo!(),
        }
    }
}
