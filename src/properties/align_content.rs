//! # Align Content Property
//!
//! This module provides a function for creating the CSS `align-content` property.
//! The `align-content` property aligns a flex container's lines within the flex container
//! when there is extra space in the cross-axis.
//!
//! ## Usage
//!
//! ```rust
//! use mew_css::properties::align_content;
//! use mew_css::values::AlignContent;
//!
//! let prop = align_content::align_content(AlignContent::Center);
//! assert_eq!(prop.to_string(), "align-content: center;");
//! ```

use crate::properties::Property;
use crate::values::AlignContent;

/// Creates a CSS `align-content` property.
///
/// The `align-content` property aligns a flex container's lines within the flex container
/// when there is extra space in the cross-axis.
///
/// # Arguments
///
/// * `value` - The align-content value to use
///
/// # Returns
///
/// A new `Property` instance representing the align-content property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::align_content;
/// use mew_css::values::AlignContent;
///
/// let prop = align_content::align_content(AlignContent::Center);
/// assert_eq!(prop.to_string(), "align-content: center;");
///
/// let prop = align_content::align_content(AlignContent::SpaceBetween);
/// assert_eq!(prop.to_string(), "align-content: space-between;");
/// ```
pub fn align_content(value: AlignContent) -> Property {
    Property::new("align-content", value)
}