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
//! ## Basic Usage
//!
//! ```rust
//! use mew_css::style;
//! use mew_css::values::{Color, Size, Display};
//!
//! let css = style()
//!     .color(Color::White)
//!     .background_color(Color::Rgba(255, 0, 0, 0.5))
//!     .font_size(Size::Px(16))
//!     .display(Display::Flex)
//!     .apply();
//!
//! // Produces: "color: white; background-color: rgba(255, 0, 0, 0.5); font-size: 16px; display: flex;"
//! ```
//!
//! ## CSS Variables
//!
//! ```rust
//! use mew_css::{style, var};
//! use mew_css::values::Color;
//!
//! // Define a CSS variable
//! let primary_color = var("primary-color");
//!
//! // Use the variable in a style
//! let css = style()
//!     .color(primary_color.into())
//!     .apply();
//!
//! // Produces: "color: var(--primary-color);"
//! ```
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

// Include integration tests
#[cfg(test)]
mod tests;

// Re-export the main API entry point
pub use style::style;
pub use variable::{CssVar, var};
