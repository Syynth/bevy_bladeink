//! Procedural derive macro for `InkBindingDefinition`.
//!
//! This crate provides the `#[derive(InkBinding)]` macro that automatically
//! implements the `InkBindingDefinition` trait for structs with basic field types.
//!
//! # Supported Field Types
//!
//! - `String` - Maps to `ValueType::String`
//! - `i32` - Maps to `ValueType::Int`
//! - `f32` - Maps to `ValueType::Float`
//! - `bool` - Maps to `ValueType::Bool`
//!
//! # Examples
//!
//! ```ignore
//! use bevy::prelude::*;
//! use bevy_bladeink::prelude::*;
//!
//! // Unit struct with no arguments
//! #[derive(Event, Clone, InkBinding)]
//! struct NoArgsEvent;
//!
//! // Named struct with multiple fields
//! #[derive(Event, Clone, InkBinding)]
//! struct PlayerEvent {
//!     name: String,
//!     health: i32,
//!     alive: bool,
//! }
//!
//! // Tuple struct
//! #[derive(Event, Clone, InkBinding)]
//! struct SimpleEvent(String);
//! ```
//!
//! # Generated Implementation
//!
//! The macro generates an implementation that:
//! 1. Validates the argument count matches the field count
//! 2. Validates each argument type matches the expected field type
//! 3. Extracts values and constructs the event struct
//! 4. Returns appropriate errors for invalid inputs
//!
//! # Error Handling
//!
//! - `InkBindingError::ArgumentsRequired` - No args provided but fields expected
//! - `InkBindingError::InvalidArguments` - Wrong types provided
//! - `InkBindingError::TooManyArguments` - More args than fields

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

/// Information about a struct field
struct FieldInfo {
    /// Field name (None for tuple structs)
    name: Option<syn::Ident>,
    /// Field type
    ty: Type,
}

/// Derives the `InkBindingDefinition` trait for event structs.
///
/// This macro automatically implements the `try_parse_event` method based on
/// the struct's field types, generating appropriate pattern matching and error
/// handling for ink function bindings.
#[proc_macro_derive(InkBinding)]
pub fn derive_ink_binding(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Extract struct name and generics
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Parse fields
    let fields = match &input.data {
        Data::Struct(data) => extract_fields(&data.fields),
        _ => {
            return syn::Error::new_spanned(
                &input,
                "InkBinding can only be derived for structs",
            )
            .to_compile_error()
            .into();
        }
    };

    // Generate match arms based on field count
    let match_arms = if fields.is_empty() {
        // Unit struct: accept empty args only
        quote! {
            [] => Ok(#name),
            _ => Err(InkBindingError::TooManyArguments),
        }
    } else {
        generate_match_arms_for_fields(name, &fields)
    };

    // Generate the complete impl
    let expanded = quote! {
        impl #impl_generics InkBindingDefinition for #name #ty_generics #where_clause {
            type Event = Self;

            fn try_parse_event(args: &[ValueType]) -> Result<Self::Event, InkBindingError> {
                match args {
                    #match_arms
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Extracts field information from struct fields
fn extract_fields(fields: &Fields) -> Vec<FieldInfo> {
    match fields {
        Fields::Named(named) => named
            .named
            .iter()
            .map(|field| FieldInfo {
                name: field.ident.clone(),
                ty: field.ty.clone(),
            })
            .collect(),
        Fields::Unnamed(unnamed) => unnamed
            .unnamed
            .iter()
            .map(|field| FieldInfo {
                name: None,
                ty: field.ty.clone(),
            })
            .collect(),
        Fields::Unit => vec![],
    }
}

/// Generates match arms for structs with fields
fn generate_match_arms_for_fields(
    name: &syn::Ident,
    fields: &[FieldInfo],
) -> proc_macro2::TokenStream {
    // Generate binding names (v0, v1, v2, ...)
    let binding_names: Vec<_> = (0..fields.len())
        .map(|i| syn::Ident::new(&format!("v{}", i), proc_macro2::Span::call_site()))
        .collect();

    // Generate patterns and extractors for each field
    let patterns_and_extractors: Result<Vec<_>, _> = fields
        .iter()
        .zip(&binding_names)
        .map(|(field, binding)| type_to_value_type_pattern(&field.ty, binding))
        .collect();

    let patterns_and_extractors = match patterns_and_extractors {
        Ok(p) => p,
        Err(e) => return e.to_compile_error(),
    };

    let patterns: Vec<_> = patterns_and_extractors.iter().map(|(pat, _)| pat).collect();

    // Generate struct constructor
    let constructor = generate_constructor(name, fields, &binding_names);

    // Generate "too few args" patterns for counts < field_count
    let too_few_patterns = if fields.len() > 1 {
        let patterns: Vec<_> = (0..fields.len())
            .map(|count| {
                let underscores = (0..count).map(|_| quote! { _ });
                quote! { [#(#underscores),*] }
            })
            .collect();
        quote! { #(#patterns)|* }
    } else {
        quote! { [] }
    };

    // Generate "wrong types" pattern (same count, but wrong types)
    let wrong_types_pattern = {
        let underscores = (0..fields.len()).map(|_| quote! { _ });
        quote! { [#(#underscores),*] }
    };

    quote! {
        // Empty args case (if fields exist)
        [] => Err(InkBindingError::ArgumentsRequired),

        // Correct pattern with type checks
        [#(#patterns),*] => {
            Ok(#constructor)
        },

        // Wrong types but correct count
        #wrong_types_pattern => Err(InkBindingError::InvalidArguments),

        // Too few args
        #too_few_patterns => Err(InkBindingError::ArgumentsRequired),

        // Too many args
        _ => Err(InkBindingError::TooManyArguments),
    }
}

/// Maps Rust type to (`ValueType` pattern, extraction expression)
fn type_to_value_type_pattern(
    ty: &Type,
    binding: &syn::Ident,
) -> Result<(proc_macro2::TokenStream, proc_macro2::TokenStream), syn::Error> {
    // Get the type as a string to match against
    let type_str = quote!(#ty).to_string();

    let (pattern, extractor) = match type_str.as_str() {
        "String" => (
            quote! { ValueType::String(#binding) },
            quote! { #binding.string.clone() },
        ),
        "i32" => (quote! { ValueType::Int(#binding) }, quote! { *#binding }),
        "f32" => (quote! { ValueType::Float(#binding) }, quote! { *#binding }),
        "bool" => (quote! { ValueType::Bool(#binding) }, quote! { *#binding }),
        _ => {
            return Err(syn::Error::new_spanned(
                ty,
                format!(
                    "Unsupported type '{}' for InkBinding derive. \
                     Supported types: String, i32, f32, bool. \
                     For custom types, implement InkBindingDefinition manually.",
                    type_str
                ),
            ));
        }
    };

    Ok((pattern, extractor))
}

/// Generates struct constructor based on field names
fn generate_constructor(
    name: &syn::Ident,
    fields: &[FieldInfo],
    bindings: &[syn::Ident],
) -> proc_macro2::TokenStream {
    if fields.is_empty() {
        // Unit struct
        quote! { #name }
    } else if fields[0].name.is_some() {
        // Named struct
        let field_names: Vec<_> = fields.iter().map(|f| f.name.as_ref().unwrap()).collect();
        let field_values: Vec<_> = fields
            .iter()
            .zip(bindings)
            .map(|(field, binding)| {
                let (_, extractor) = type_to_value_type_pattern(&field.ty, binding).unwrap();
                extractor
            })
            .collect();

        quote! {
            #name {
                #(#field_names: #field_values),*
            }
        }
    } else {
        // Tuple struct
        let field_values: Vec<_> = fields
            .iter()
            .zip(bindings)
            .map(|(field, binding)| {
                let (_, extractor) = type_to_value_type_pattern(&field.ty, binding).unwrap();
                extractor
            })
            .collect();

        quote! {
            #name(#(#field_values),*)
        }
    }
}
