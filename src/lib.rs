//! CSS Style Builder Library
//!
//! A fluent, chainable API for building CSS styles with strong typing.
//!
//! # Example
//!
//! ```
//! use mew::style;
//! use mew::values::{Color, Size, Display};
//!
//! let css = style()
//!     .color(Color::White)
//!     .background_color(Color::Rgba(255, 0, 0, 0.5))
//!     .font_size(Size::Px(16))
//!     .display(Display::Flex)
//!     .apply();
//! ```

// Make modules public
pub mod style;
pub mod values;
pub mod properties;

// Include integration tests
#[cfg(test)]
mod tests;

// Re-export the main API entry point
pub use style::style;
