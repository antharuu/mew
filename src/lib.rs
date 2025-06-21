//! # Mew CSS Style Builder
//!
//! A fluent, chainable API for building CSS styles with strong typing in Rust.
//!
//! This library provides a type-safe way to generate CSS with comprehensive validation,
//! no external dependencies, and an extensible design following SOLID principles.
//!
//! ## Core Features
//!
//! - **Type Safety**: All CSS properties and values are strongly typed, preventing invalid CSS
//! - **Fluent API**: Chain method calls for a clean, declarative style
//! - **No Dependencies**: Zero external dependencies for a lightweight footprint
//! - **Comprehensive**: Supports a wide range of CSS properties and values
//! - **CSS Variables**: First-class support for CSS custom properties
//!
//!
//! ## Module Organization
//!
//! - `style`: Core style builder implementation with fluent API
//! - `values`: CSS value types (Color, Size, Display, etc.)
//! - `properties`: CSS property definitions
//! - `variable`: CSS variables support

// Make modules public
pub mod style;
pub mod values;
pub mod properties;
pub mod variable;

// Re-export the main API entry point
pub use style::style;
pub use variable::{CssVar, var};
