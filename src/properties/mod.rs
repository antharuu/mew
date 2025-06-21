//! # CSS Properties Module
//!
//! This module defines the CSS properties that can be set on a style. It provides
//! a structured way to create and manage CSS properties with type safety.
//!
//! ## Module Organization
//!
//! The properties are organized into individual files, one for each property:
//!
//! - `align_content`: Align content property for flex and grid containers
//! - `align_items`: Align items property for flex and grid containers
//!
//! ## Usage
//!
//! While you can use this module directly to create properties, it's generally
//! easier to use the methods on the `Style` struct, which will call these
//! functions for you.
//!
//! ```rust
//! use mew_css::properties::{Property, align_content};
//! use mew_css::values::AlignContent;
//!
//! // Direct usage
//! let property = align_content::align_content(AlignContent::Center);
//!
//! // More commonly, through the Style API
//! use mew_css::style;
//! let css = style().align_content(AlignContent::Center).apply();
//! ```

use std::fmt;

/// Represents a single CSS property with a name and value.
///
/// A `Property` is the fundamental building block of CSS styles in this library.
/// Each property has a name (like "color" or "margin-top") and a value that has
/// been converted to a string representation.
///
/// Properties are typically created using the functions in the submodules of this
/// module, rather than being constructed directly.
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::Property;
///
/// // Create a property directly
/// let color_prop = Property::new("color", "blue");
/// let font_size_prop = Property::new("font-size", "16px");
///
/// // The string representation includes the semicolon
/// assert_eq!(color_prop.to_string(), "color: blue;");
/// ```
#[derive(Debug, Clone)]
pub struct Property {
    /// The CSS property name (e.g., "color", "margin-top")
    name: String,
    /// The CSS property value as a string (e.g., "blue", "20px")
    value: String,
}

impl Property {
    /// Creates a new CSS property with the given name and value.
    ///
    /// This method converts the value to a string using the `Display` trait.
    ///
    /// # Arguments
    ///
    /// * `name` - The CSS property name
    /// * `value` - The property value, which can be any type that implements `Display`
    ///
    /// # Returns
    ///
    /// A new `Property` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mew_css::properties::Property;
    ///
    /// let color_prop = Property::new("color", "blue");
    /// let margin_prop = Property::new("margin", "10px");
    /// let opacity_prop = Property::new("opacity", 0.5);
    /// ```
    pub fn new<T: fmt::Display>(name: &str, value: T) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {};", self.name, self.value)
    }
}

// Export property modules
pub mod align_content;
pub mod align_items;
pub mod background_color;
pub mod border;
pub mod border_bottom;
pub mod border_top;
pub mod border_left;
pub mod border_right;
