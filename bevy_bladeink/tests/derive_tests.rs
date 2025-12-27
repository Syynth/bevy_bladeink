//! Integration tests for the InkBinding derive macro.
//!
//! These tests verify that derived implementations match the expected behavior
//! of manual implementations.

use bevy::prelude::*;
use bevy_bladeink::prelude::*;
use bladeink::value_type::ValueType;

// ============================================================================
// Unit struct (0 fields)
// ============================================================================

#[derive(Event, Clone, Debug, PartialEq, InkBinding)]
struct DerivedNoArgs;

#[test]
fn test_derived_no_args_accepts_empty() {
    let result = DerivedNoArgs::try_parse_event(&[]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), DerivedNoArgs);
}

#[test]
fn test_derived_no_args_rejects_args() {
    let result = DerivedNoArgs::try_parse_event(&[ValueType::from(42)]);
    assert!(matches!(result, Err(InkBindingError::TooManyArguments)));
}

// ============================================================================
// Tuple struct (single field)
// ============================================================================

#[derive(Event, Clone, Debug, PartialEq, InkBinding)]
struct DerivedSingleString(String);

#[test]
fn test_derived_single_string_parsing() {
    let args = vec![ValueType::from("test")];
    let result = DerivedSingleString::try_parse_event(&args);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), DerivedSingleString("test".to_string()));
}

#[test]
fn test_derived_single_string_requires_args() {
    let result = DerivedSingleString::try_parse_event(&[]);
    assert!(matches!(result, Err(InkBindingError::ArgumentsRequired)));
}

#[test]
fn test_derived_single_string_rejects_wrong_type() {
    let args = vec![ValueType::from(42)];
    let result = DerivedSingleString::try_parse_event(&args);
    assert!(matches!(result, Err(InkBindingError::InvalidArguments)));
}

#[test]
fn test_derived_single_string_rejects_too_many() {
    let args = vec![ValueType::from("test"), ValueType::from("extra")];
    let result = DerivedSingleString::try_parse_event(&args);
    assert!(matches!(result, Err(InkBindingError::TooManyArguments)));
}

// ============================================================================
// Tuple struct with int
// ============================================================================

#[derive(Event, Clone, Debug, PartialEq, InkBinding)]
struct DerivedSingleInt(i32);

#[test]
fn test_derived_single_int_parsing() {
    let args = vec![ValueType::from(42)];
    let result = DerivedSingleInt::try_parse_event(&args);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), DerivedSingleInt(42));
}

#[test]
fn test_derived_single_int_rejects_wrong_type() {
    let args = vec![ValueType::from("42")];
    let result = DerivedSingleInt::try_parse_event(&args);
    assert!(matches!(result, Err(InkBindingError::InvalidArguments)));
}

// ============================================================================
// Tuple struct with float
// ============================================================================

#[derive(Event, Clone, Debug, PartialEq, InkBinding)]
struct DerivedSingleFloat(f32);

#[test]
fn test_derived_single_float_parsing() {
    let args = vec![ValueType::from(3.14f32)];
    let result = DerivedSingleFloat::try_parse_event(&args);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), DerivedSingleFloat(3.14));
}

#[test]
fn test_derived_single_float_rejects_wrong_type() {
    let args = vec![ValueType::Int(3)];
    let result = DerivedSingleFloat::try_parse_event(&args);
    assert!(matches!(result, Err(InkBindingError::InvalidArguments)));
}

// ============================================================================
// Tuple struct with bool
// ============================================================================

#[derive(Event, Clone, Debug, PartialEq, InkBinding)]
struct DerivedSingleBool(bool);

#[test]
fn test_derived_single_bool_parsing() {
    let args = vec![ValueType::from(true)];
    let result = DerivedSingleBool::try_parse_event(&args);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), DerivedSingleBool(true));

    let args = vec![ValueType::from(false)];
    let result = DerivedSingleBool::try_parse_event(&args);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), DerivedSingleBool(false));
}

#[test]
fn test_derived_single_bool_rejects_wrong_type() {
    let args = vec![ValueType::from(1)];
    let result = DerivedSingleBool::try_parse_event(&args);
    assert!(matches!(result, Err(InkBindingError::InvalidArguments)));
}

// ============================================================================
// Named struct with multiple fields
// ============================================================================

#[derive(Event, Clone, Debug, PartialEq, InkBinding)]
struct DerivedMultiField {
    name: String,
    value: i32,
    enabled: bool,
}

#[test]
fn test_derived_multi_field_parsing() {
    let args = vec![
        ValueType::from("player"),
        ValueType::from(100),
        ValueType::from(true),
    ];
    let result = DerivedMultiField::try_parse_event(&args);
    assert!(result.is_ok());
    let event = result.unwrap();
    assert_eq!(event.name, "player");
    assert_eq!(event.value, 100);
    assert!(event.enabled);
}

#[test]
fn test_derived_multi_field_requires_all_args() {
    // No arguments
    let result = DerivedMultiField::try_parse_event(&[]);
    assert!(matches!(result, Err(InkBindingError::ArgumentsRequired)));

    // Only one argument
    let result = DerivedMultiField::try_parse_event(&[ValueType::from("player")]);
    assert!(matches!(result, Err(InkBindingError::ArgumentsRequired)));

    // Only two arguments
    let result = DerivedMultiField::try_parse_event(&[
        ValueType::from("player"),
        ValueType::from(100),
    ]);
    assert!(matches!(result, Err(InkBindingError::ArgumentsRequired)));
}

#[test]
fn test_derived_multi_field_rejects_wrong_types() {
    let args = vec![
        ValueType::from(42), // Wrong type for first arg
        ValueType::from(100),
        ValueType::from(true),
    ];
    let result = DerivedMultiField::try_parse_event(&args);
    assert!(matches!(result, Err(InkBindingError::InvalidArguments)));
}

#[test]
fn test_derived_multi_field_rejects_too_many() {
    let args = vec![
        ValueType::from("player"),
        ValueType::from(100),
        ValueType::from(true),
        ValueType::from(999), // Extra argument
    ];
    let result = DerivedMultiField::try_parse_event(&args);
    assert!(matches!(result, Err(InkBindingError::TooManyArguments)));
}

// ============================================================================
// Tuple struct with multiple fields
// ============================================================================

#[derive(Event, Clone, Debug, PartialEq, InkBinding)]
struct DerivedTupleMulti(String, i32, bool);

#[test]
fn test_derived_tuple_multi_parsing() {
    let args = vec![
        ValueType::from("data"),
        ValueType::from(42),
        ValueType::from(false),
    ];
    let result = DerivedTupleMulti::try_parse_event(&args);
    assert!(result.is_ok());
    let event = result.unwrap();
    assert_eq!(event.0, "data");
    assert_eq!(event.1, 42);
    assert!(!event.2);
}

#[test]
fn test_derived_tuple_multi_requires_all_args() {
    let result = DerivedTupleMulti::try_parse_event(&[]);
    assert!(matches!(result, Err(InkBindingError::ArgumentsRequired)));

    let result = DerivedTupleMulti::try_parse_event(&[ValueType::from("data")]);
    assert!(matches!(result, Err(InkBindingError::ArgumentsRequired)));
}

#[test]
fn test_derived_tuple_multi_rejects_wrong_types() {
    let args = vec![
        ValueType::from("data"),
        ValueType::from("wrong"), // Wrong type
        ValueType::from(false),
    ];
    let result = DerivedTupleMulti::try_parse_event(&args);
    assert!(matches!(result, Err(InkBindingError::InvalidArguments)));
}

// ============================================================================
// Complex named struct with all types
// ============================================================================

#[derive(Event, Clone, Debug, PartialEq, InkBinding)]
struct DerivedAllTypes {
    message: String,
    count: i32,
    ratio: f32,
    active: bool,
}

#[test]
fn test_derived_all_types_parsing() {
    let args = vec![
        ValueType::from("hello"),
        ValueType::from(7),
        ValueType::from(0.5f32),
        ValueType::from(true),
    ];
    let result = DerivedAllTypes::try_parse_event(&args);
    assert!(result.is_ok());
    let event = result.unwrap();
    assert_eq!(event.message, "hello");
    assert_eq!(event.count, 7);
    assert_eq!(event.ratio, 0.5);
    assert!(event.active);
}

#[test]
fn test_derived_all_types_validates_each_field() {
    // Wrong type for first field
    let args = vec![
        ValueType::from(42),
        ValueType::from(7),
        ValueType::from(0.5f32),
        ValueType::from(true),
    ];
    let result = DerivedAllTypes::try_parse_event(&args);
    assert!(matches!(result, Err(InkBindingError::InvalidArguments)));

    // Wrong type for second field
    let args = vec![
        ValueType::from("hello"),
        ValueType::from("wrong"),
        ValueType::from(0.5f32),
        ValueType::from(true),
    ];
    let result = DerivedAllTypes::try_parse_event(&args);
    assert!(matches!(result, Err(InkBindingError::InvalidArguments)));

    // Wrong type for third field
    let args = vec![
        ValueType::from("hello"),
        ValueType::from(7),
        ValueType::from(true), // bool instead of float
        ValueType::from(true),
    ];
    let result = DerivedAllTypes::try_parse_event(&args);
    assert!(matches!(result, Err(InkBindingError::InvalidArguments)));

    // Wrong type for fourth field
    let args = vec![
        ValueType::from("hello"),
        ValueType::from(7),
        ValueType::from(0.5f32),
        ValueType::from(1), // int instead of bool
    ];
    let result = DerivedAllTypes::try_parse_event(&args);
    assert!(matches!(result, Err(InkBindingError::InvalidArguments)));
}
