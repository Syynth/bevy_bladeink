use std::{cell::RefCell, rc::Rc};

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_crossbeam_event::{CrossbeamEventApp, CrossbeamEventSender};
use bladeink::{
    story::{external_functions::ExternalFunction, variable_observer::VariableObserver},
    value_type::ValueType,
};
use thiserror::Error;

/// Error type for ink bindings.
/// This error type is used to handle the possible failures that can occur
/// attempting to bind an ink function to a Bevy app.
#[derive(Error, Debug)]
pub enum InkBindingError {
    /// No arguments provided, but some arguments are required.
    #[error("Arguments Required")]
    ArgumentsRequired,

    /// Invalid argument values.
    #[error("Invalid argument values")]
    InvalidArguments,

    /// Too many arguments provided.
    #[error("Too many arguments provided")]
    TooManyArguments,
}

/// Ink binding trait
pub trait InkBindingDefinition: Clone + 'static {
    /// Event type for the binding.
    type Event: Event + Clone;

    /// Parses the event from the given arguments.
    fn try_parse_event(args: &[ValueType]) -> Result<Self::Event, InkBindingError>;

    /// Evaluates the parsed event, and optionally returns a value to the ink runtime.
    fn evaluate(_event: &Self::Event) -> Option<impl Into<ValueType>> {
        Option::<ValueType>::None
    }
}

/// Allows defining an observer/event for an ink function.
///
/// This trait provides a way to bind an ink function to a Bevy app.
/// The ink function can be called from within the ink script and will trigger the associated event.
pub trait AddInkBindingApp {
    /// Bind an ink command to the application.
    fn bind_ink_function<T: InkBindingDefinition + 'static>(
        &mut self,
        name: impl AsRef<str>,
    ) -> &mut Self
    where
        for<'a> <T::Event as Event>::Trigger<'a>: Default;
}

impl AddInkBindingApp for App {
    fn bind_ink_function<T: InkBindingDefinition + 'static>(
        &mut self,
        name: impl AsRef<str>,
    ) -> &mut Self
    where
        for<'a> <T::Event as Event>::Trigger<'a>: Default,
    {
        let world = self.world_mut();
        if world
            .get_resource::<CrossbeamEventSender<T::Event>>()
            .is_none()
        {
            self.add_crossbeam_event::<T::Event>();
        }

        let world = self.world_mut();

        let channel = world
            .get_resource::<CrossbeamEventSender<T::Event>>()
            .expect("CrossbeamEventSender initialized above. If you see this error, it means that the bevy_bladeink plugin was not initialized correctly.")
            .clone();

        let mut binding_map = world
            .get_non_send_resource_mut::<InkBindingMap>()
            .expect("Failed to locate ink binding definitions storage, did you forget to initialize the bevy_bladeink plugin?");

        binding_map.insert(
            name.as_ref().to_string(),
            InkBindingFn::<T>::to_binding(channel.clone()),
        );

        self
    }
}

/// Storage for ink bindings.
pub(crate) type InkBindingMap = HashMap<String, Rc<RefCell<dyn ExternalFunction>>>;

/// Phantom ink binding that does nothing.
#[derive(Clone)]
pub(crate) struct InkBindingFn<B: InkBindingDefinition> {
    sender: CrossbeamEventSender<B::Event>,
}

impl<B: InkBindingDefinition> InkBindingFn<B> {
    /// Create a new phantom ink binding.
    pub(crate) fn to_binding(
        sender: CrossbeamEventSender<B::Event>,
    ) -> Rc<RefCell<dyn ExternalFunction>> {
        Rc::new(RefCell::new(Self { sender }))
    }
}

impl<B: InkBindingDefinition> InkBindingFn<B> {
    /// Create a new phantom ink binding.
    pub(crate) fn to_observer(
        sender: CrossbeamEventSender<B::Event>,
    ) -> Rc<RefCell<dyn VariableObserver>> {
        Rc::new(RefCell::new(Self { sender }))
    }
}

impl<B: InkBindingDefinition> ExternalFunction for InkBindingFn<B> {
    fn call(&mut self, name: &str, args: Vec<ValueType>) -> Option<ValueType> {
        let event = match B::try_parse_event(&args[..]) {
            Ok(event) => event,
            Err(err) => {
                error!("Failed to invoke ink binding '{name}': {err:?}");
                return None;
            }
        };
        let retval: Option<ValueType> = B::evaluate(&event).map(Into::into);
        self.sender.send(event);
        retval
    }
}

impl<B: InkBindingDefinition> VariableObserver for InkBindingFn<B> {
    fn changed(&mut self, name: &str, value: &ValueType) {
        let event = match B::try_parse_event(&[ValueType::from(name), value.clone()]) {
            Ok(event) => event,
            Err(err) => {
                error!("Failed to invoke ink binding '{name}': {err:?}");
                return;
            }
        };
        self.sender.send(event);
    }
}

#[cfg(test)]
mod tests {
    use std::f32;

    use super::*;
    use bladeink::value_type::ValueType;

    // Test event with no arguments
    #[derive(Event, Clone, Debug, PartialEq)]
    struct NoArgsEvent;

    impl InkBindingDefinition for NoArgsEvent {
        type Event = Self;

        fn try_parse_event(args: &[ValueType]) -> Result<Self::Event, InkBindingError> {
            match args {
                [] => Ok(NoArgsEvent),
                _ => Err(InkBindingError::TooManyArguments),
            }
        }
    }

    // Test event with a single string argument
    #[derive(Event, Clone, Debug, PartialEq)]
    struct SingleStringEvent(String);

    impl InkBindingDefinition for SingleStringEvent {
        type Event = Self;

        fn try_parse_event(args: &[ValueType]) -> Result<Self::Event, InkBindingError> {
            match args {
                [] => Err(InkBindingError::ArgumentsRequired),
                [ValueType::String(s)] => Ok(SingleStringEvent(s.string.clone())),
                [_] => Err(InkBindingError::InvalidArguments),
                _ => Err(InkBindingError::TooManyArguments),
            }
        }
    }

    // Test event with a single integer argument
    #[derive(Event, Clone, Debug, PartialEq)]
    struct SingleIntEvent(i32);

    impl InkBindingDefinition for SingleIntEvent {
        type Event = Self;

        fn try_parse_event(args: &[ValueType]) -> Result<Self::Event, InkBindingError> {
            match args {
                [] => Err(InkBindingError::ArgumentsRequired),
                [ValueType::Int(i)] => Ok(SingleIntEvent(*i)),
                [_] => Err(InkBindingError::InvalidArguments),
                _ => Err(InkBindingError::TooManyArguments),
            }
        }
    }

    // Test event with a single float argument
    #[derive(Event, Clone, Debug, PartialEq)]
    struct SingleFloatEvent(f32);

    impl InkBindingDefinition for SingleFloatEvent {
        type Event = Self;

        fn try_parse_event(args: &[ValueType]) -> Result<Self::Event, InkBindingError> {
            match args {
                [] => Err(InkBindingError::ArgumentsRequired),
                [ValueType::Float(f)] => Ok(SingleFloatEvent(*f)),
                [_] => Err(InkBindingError::InvalidArguments),
                _ => Err(InkBindingError::TooManyArguments),
            }
        }
    }

    // Test event with a single boolean argument
    #[derive(Event, Clone, Debug, PartialEq)]
    struct SingleBoolEvent(bool);

    impl InkBindingDefinition for SingleBoolEvent {
        type Event = Self;

        fn try_parse_event(args: &[ValueType]) -> Result<Self::Event, InkBindingError> {
            match args {
                [] => Err(InkBindingError::ArgumentsRequired),
                [ValueType::Bool(b)] => Ok(SingleBoolEvent(*b)),
                [_] => Err(InkBindingError::InvalidArguments),
                _ => Err(InkBindingError::TooManyArguments),
            }
        }
    }

    // Test event with multiple arguments
    #[derive(Event, Clone, Debug, PartialEq)]
    struct MultiArgEvent {
        name: String,
        value: i32,
        enabled: bool,
    }

    impl InkBindingDefinition for MultiArgEvent {
        type Event = Self;

        fn try_parse_event(args: &[ValueType]) -> Result<Self::Event, InkBindingError> {
            match args {
                [
                    ValueType::String(name),
                    ValueType::Int(value),
                    ValueType::Bool(enabled),
                ] => Ok(MultiArgEvent {
                    name: name.string.clone(),
                    value: *value,
                    enabled: *enabled,
                }),
                [_, _, _] => Err(InkBindingError::InvalidArguments),
                [] | [_] | [_, _] => Err(InkBindingError::ArgumentsRequired),
                _ => Err(InkBindingError::TooManyArguments),
            }
        }
    }

    #[test]
    fn test_no_args_event_parsing() {
        let result = NoArgsEvent::try_parse_event(&[]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), NoArgsEvent);
    }

    #[test]
    fn test_no_args_event_rejects_arguments() {
        let args = vec![ValueType::from(42)];
        let result = NoArgsEvent::try_parse_event(&args);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InkBindingError::TooManyArguments
        ));
    }

    #[test]
    fn test_single_string_event_parsing() {
        let args = vec![ValueType::from("test")];
        let result = SingleStringEvent::try_parse_event(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), SingleStringEvent("test".to_string()));
    }

    #[test]
    fn test_single_string_event_requires_arguments() {
        let result = SingleStringEvent::try_parse_event(&[]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InkBindingError::ArgumentsRequired
        ));
    }

    #[test]
    fn test_single_string_event_rejects_wrong_type() {
        let args = vec![ValueType::from(42)];
        let result = SingleStringEvent::try_parse_event(&args);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InkBindingError::InvalidArguments
        ));
    }

    #[test]
    fn test_single_string_event_rejects_too_many_args() {
        let args = vec![ValueType::from("test"), ValueType::from("extra")];
        let result = SingleStringEvent::try_parse_event(&args);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InkBindingError::TooManyArguments
        ));
    }

    #[test]
    fn test_single_int_event_parsing() {
        let args = vec![ValueType::from(42)];
        let result = SingleIntEvent::try_parse_event(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), SingleIntEvent(42));
    }

    #[test]
    fn test_single_int_event_rejects_wrong_type() {
        let args = vec![ValueType::from("42")];
        let result = SingleIntEvent::try_parse_event(&args);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InkBindingError::InvalidArguments
        ));
    }

    #[test]
    fn test_single_float_event_parsing() {
        let args = vec![ValueType::from(f32::consts::PI)];
        let result = SingleFloatEvent::try_parse_event(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), SingleFloatEvent(f32::consts::PI));
    }

    #[test]
    fn test_single_float_event_rejects_wrong_type() {
        let args = vec![ValueType::Int(3)];
        let result = SingleFloatEvent::try_parse_event(&args);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InkBindingError::InvalidArguments
        ));
    }

    #[test]
    fn test_single_bool_event_parsing() {
        let args = vec![ValueType::from(true)];
        let result = SingleBoolEvent::try_parse_event(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), SingleBoolEvent(true));

        let args = vec![ValueType::from(false)];
        let result = SingleBoolEvent::try_parse_event(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), SingleBoolEvent(false));
    }

    #[test]
    fn test_single_bool_event_rejects_wrong_type() {
        let args = vec![ValueType::from(1)];
        let result = SingleBoolEvent::try_parse_event(&args);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InkBindingError::InvalidArguments
        ));
    }

    #[test]
    fn test_multi_arg_event_parsing() {
        let args = vec![
            ValueType::from("player"),
            ValueType::from(100),
            ValueType::from(true),
        ];
        let result = MultiArgEvent::try_parse_event(&args);
        assert!(result.is_ok());
        let event = result.unwrap();
        assert_eq!(event.name, "player");
        assert_eq!(event.value, 100);
        assert!(event.enabled);
    }

    #[test]
    fn test_multi_arg_event_requires_all_arguments() {
        // No arguments
        let result = MultiArgEvent::try_parse_event(&[]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InkBindingError::ArgumentsRequired
        ));

        // Only one argument
        let result = MultiArgEvent::try_parse_event(&[ValueType::from("player")]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InkBindingError::ArgumentsRequired
        ));

        // Only two arguments
        let result =
            MultiArgEvent::try_parse_event(&[ValueType::from("player"), ValueType::from(100)]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InkBindingError::ArgumentsRequired
        ));
    }

    #[test]
    fn test_multi_arg_event_rejects_wrong_types() {
        let args = vec![
            ValueType::from(42), // Wrong type for first arg
            ValueType::from(100),
            ValueType::from(true),
        ];
        let result = MultiArgEvent::try_parse_event(&args);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InkBindingError::InvalidArguments
        ));
    }

    #[test]
    fn test_multi_arg_event_rejects_too_many_args() {
        let args = vec![
            ValueType::from("player"),
            ValueType::from(100),
            ValueType::from(true),
            ValueType::from(999), // Extra argument
        ];
        let result = MultiArgEvent::try_parse_event(&args);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InkBindingError::TooManyArguments
        ));
    }

    #[test]
    fn test_bind_ink_command_adds_binding() {
        let mut app = App::new();
        app.init_non_send_resource::<InkBindingMap>();
        app.bind_ink_function::<NoArgsEvent>("test_command");

        let world = app.world();
        let binding_map = world
            .get_non_send_resource::<InkBindingMap>()
            .expect("InkBindingMap should exist");

        assert!(binding_map.contains_key("test_command"));
    }

    #[test]
    fn test_bind_multiple_commands() {
        let mut app = App::new();
        app.init_non_send_resource::<InkBindingMap>();

        app.bind_ink_function::<NoArgsEvent>("command1");
        app.bind_ink_function::<SingleStringEvent>("command2");
        app.bind_ink_function::<SingleIntEvent>("command3");

        let world = app.world();
        let binding_map = world
            .get_non_send_resource::<InkBindingMap>()
            .expect("InkBindingMap should exist");

        assert!(binding_map.contains_key("command1"));
        assert!(binding_map.contains_key("command2"));
        assert!(binding_map.contains_key("command3"));
        assert_eq!(binding_map.len(), 3);
    }

    #[test]
    fn test_external_function_call_with_valid_args() {
        let mut app = App::new();
        app.init_non_send_resource::<InkBindingMap>();
        app.bind_ink_function::<SingleStringEvent>("test_func");

        let world = app.world();
        let binding_map = world
            .get_non_send_resource::<InkBindingMap>()
            .expect("InkBindingMap should exist");

        let binding = binding_map
            .get("test_func")
            .expect("Binding should exist")
            .clone();

        // Call the external function with valid arguments
        let args = vec![ValueType::from("hello")];
        let result = binding.borrow_mut().call("test_func", args);

        // Should return None (functions don't return values in this implementation)
        assert!(result.is_none());
    }

    #[test]
    fn test_external_function_call_with_invalid_args() {
        let mut app = App::new();
        app.init_non_send_resource::<InkBindingMap>();
        app.bind_ink_function::<SingleStringEvent>("test_func");

        let world = app.world();
        let binding_map = world
            .get_non_send_resource::<InkBindingMap>()
            .expect("InkBindingMap should exist");

        let binding = binding_map
            .get("test_func")
            .expect("Binding should exist")
            .clone();

        // Call with wrong argument type (should log error but not panic)
        let args = vec![ValueType::Int(42)];
        let result = binding.borrow_mut().call("test_func", args);

        // Should return None even on error
        assert!(result.is_none());
    }

    #[test]
    fn test_binding_can_be_called_multiple_times() {
        let mut app = App::new();
        app.init_non_send_resource::<InkBindingMap>();
        app.bind_ink_function::<SingleIntEvent>("increment");

        let world = app.world();
        let binding_map = world
            .get_non_send_resource::<InkBindingMap>()
            .expect("InkBindingMap should exist");

        let binding = binding_map
            .get("increment")
            .expect("Binding should exist")
            .clone();

        // Call multiple times
        for i in 0..5 {
            let args = vec![ValueType::Int(i)];
            let result = binding.borrow_mut().call("increment", args);
            assert!(result.is_none());
        }
    }
}
