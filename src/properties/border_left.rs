//! # Border Left Property
//!
//! This module provides a function for creating the CSS `border-left` property.
//! The `border-left` property is a shorthand property that sets the border-left-width, border-left-style, and border-left-color.
//!
//! ## Syntax
//!
//! ```css
//! /* style */
//! border-left: solid;
//!
//! /* width | style */
//! border-left: 2px dotted;
//!
//! /* style | color */
//! border-left: outset #f33;
//!
//! /* width | style | color */
//! border-left: medium dashed green;
//!
//! /* Global values */
//! border-left: inherit;
//! border-left: initial;
//! border-left: unset;
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use mew_css::properties::border_left;
//! use mew_css::values::{BorderStyle, Color, Size};
//!
//! // Style only
//! let prop = border_left::border_left(BorderStyle::Solid);
//! assert_eq!(prop.to_string(), "border-left: solid;");
//!
//! // Width and style
//! let prop = border_left::border_left_with_width(Size::Px(2), BorderStyle::Dotted);
//! assert_eq!(prop.to_string(), "border-left: 2px dotted;");
//!
//! // Style and color
//! let prop = border_left::border_left_with_color(BorderStyle::Outset, Color::Hex("#f33".to_string()));
//! assert_eq!(prop.to_string(), "border-left: outset #f33;");
//!
//! // Width, style, and color
//! let prop = border_left::border_left_with_width_and_color(Size::Px(3), BorderStyle::Dashed, Color::Green);
//! assert_eq!(prop.to_string(), "border-left: 3px dashed green;");
//! ```

use crate::properties::Property;
use crate::values::{BorderStyle, Color, Size};
use std::fmt;

/// A struct to represent border-left property values
struct BorderLeftValue {
    width: Option<Size>,
    style: BorderStyle,
    color: Option<Color>,
}

impl fmt::Display for BorderLeftValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        if let Some(width) = &self.width {
            parts.push(width.to_string());
        }

        parts.push(self.style.to_string());

        if let Some(color) = &self.color {
            parts.push(color.to_string());
        }

        write!(f, "{}", parts.join(" "))
    }
}

/// Creates a CSS `border-left` property with only style.
///
/// The `border-left` property is a shorthand property that sets the border-left-width, border-left-style, and border-left-color.
/// This function sets only the border-left style.
///
/// ## Values
///
/// - `style`: The border-left style (solid, dashed, dotted, etc.)
///
/// # Arguments
///
/// * `style` - The border-left style to use
///
/// # Returns
///
/// A new `Property` instance representing the border-left property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::border_left;
/// use mew_css::values::BorderStyle;
///
/// let prop = border_left::border_left(BorderStyle::Solid);
/// assert_eq!(prop.to_string(), "border-left: solid;");
///
/// let prop = border_left::border_left(BorderStyle::Dashed);
/// assert_eq!(prop.to_string(), "border-left: dashed;");
/// ```
pub fn border_left(style: BorderStyle) -> Property {
    let value = BorderLeftValue {
        width: None,
        style,
        color: None,
    };

    Property::new("border-left", value)
}

/// Creates a CSS `border-left` property with style and width.
///
/// The `border-left` property is a shorthand property that sets the border-left-width, border-left-style, and border-left-color.
/// This function sets the border-left width and style.
///
/// ## Values
///
/// - `width`: The border-left width (px, em, rem, etc.)
/// - `style`: The border-left style (solid, dashed, dotted, etc.)
///
/// # Arguments
///
/// * `width` - The border-left width to use
/// * `style` - The border-left style to use
///
/// # Returns
///
/// A new `Property` instance representing the border-left property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::border_left;
/// use mew_css::values::{BorderStyle, Size};
///
/// let prop = border_left::border_left_with_width(Size::Px(2), BorderStyle::Dotted);
/// assert_eq!(prop.to_string(), "border-left: 2px dotted;");
///
/// let prop = border_left::border_left_with_width(Size::Rem(1.5), BorderStyle::Solid);
/// assert_eq!(prop.to_string(), "border-left: 1.5rem solid;");
/// ```
pub fn border_left_with_width(width: Size, style: BorderStyle) -> Property {
    let value = BorderLeftValue {
        width: Some(width),
        style,
        color: None,
    };

    Property::new("border-left", value)
}

/// Creates a CSS `border-left` property with style and color.
///
/// The `border-left` property is a shorthand property that sets the border-left-width, border-left-style, and border-left-color.
/// This function sets the border-left style and color.
///
/// ## Values
///
/// - `style`: The border-left style (solid, dashed, dotted, etc.)
/// - `color`: The border-left color (named colors, RGB, RGBA, Hex, etc.)
///
/// # Arguments
///
/// * `style` - The border-left style to use
/// * `color` - The border-left color to use
///
/// # Returns
///
/// A new `Property` instance representing the border-left property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::border_left;
/// use mew_css::values::{BorderStyle, Color};
///
/// let prop = border_left::border_left_with_color(BorderStyle::Outset, Color::Hex("#f33".to_string()));
/// assert_eq!(prop.to_string(), "border-left: outset #f33;");
///
/// let prop = border_left::border_left_with_color(BorderStyle::Solid, Color::Red);
/// assert_eq!(prop.to_string(), "border-left: solid red;");
/// ```
pub fn border_left_with_color(style: BorderStyle, color: Color) -> Property {
    let value = BorderLeftValue {
        width: None,
        style,
        color: Some(color),
    };

    Property::new("border-left", value)
}

/// Creates a CSS `border-left` property with width, style, and color.
///
/// The `border-left` property is a shorthand property that sets the border-left-width, border-left-style, and border-left-color.
/// This function sets all three properties.
///
/// ## Values
///
/// - `width`: The border-left width (px, em, rem, etc.)
/// - `style`: The border-left style (solid, dashed, dotted, etc.)
/// - `color`: The border-left color (named colors, RGB, RGBA, Hex, etc.)
///
/// # Arguments
///
/// * `width` - The border-left width to use
/// * `style` - The border-left style to use
/// * `color` - The border-left color to use
///
/// # Returns
///
/// A new `Property` instance representing the border-left property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::border_left;
/// use mew_css::values::{BorderStyle, Color, Size};
///
/// let prop = border_left::border_left_with_width_and_color(Size::Px(3), BorderStyle::Dashed, Color::Green);
/// assert_eq!(prop.to_string(), "border-left: 3px dashed green;");
///
/// let prop = border_left::border_left_with_width_and_color(Size::Px(4), BorderStyle::Double, Color::Rgb(50, 161, 206));
/// assert_eq!(prop.to_string(), "border-left: 4px double rgb(50, 161, 206);");
/// ```
pub fn border_left_with_width_and_color(width: Size, style: BorderStyle, color: Color) -> Property {
    let value = BorderLeftValue {
        width: Some(width),
        style,
        color: Some(color),
    };

    Property::new("border-left", value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::{BorderStyle, Color, Size};

    #[test]
    fn test_border_left_style_only() {
        let prop = border_left(BorderStyle::Solid);
        assert_eq!(prop.to_string(), "border-left: solid;");

        let prop = border_left(BorderStyle::Dashed);
        assert_eq!(prop.to_string(), "border-left: dashed;");

        let prop = border_left(BorderStyle::Dotted);
        assert_eq!(prop.to_string(), "border-left: dotted;");

        let prop = border_left(BorderStyle::Double);
        assert_eq!(prop.to_string(), "border-left: double;");

        let prop = border_left(BorderStyle::Groove);
        assert_eq!(prop.to_string(), "border-left: groove;");

        let prop = border_left(BorderStyle::Ridge);
        assert_eq!(prop.to_string(), "border-left: ridge;");

        let prop = border_left(BorderStyle::Inset);
        assert_eq!(prop.to_string(), "border-left: inset;");

        let prop = border_left(BorderStyle::Outset);
        assert_eq!(prop.to_string(), "border-left: outset;");

        let prop = border_left(BorderStyle::None);
        assert_eq!(prop.to_string(), "border-left: none;");
    }

    #[test]
    fn test_border_left_with_width() {
        let prop = border_left_with_width(Size::Px(2), BorderStyle::Dotted);
        assert_eq!(prop.to_string(), "border-left: 2px dotted;");

        let prop = border_left_with_width(Size::Rem(1.5), BorderStyle::Solid);
        assert_eq!(prop.to_string(), "border-left: 1.5rem solid;");

        let prop = border_left_with_width(Size::Em(0.5), BorderStyle::Dashed);
        assert_eq!(prop.to_string(), "border-left: 0.5em dashed;");

        let prop = border_left_with_width(Size::Percent(100.0), BorderStyle::Double);
        assert_eq!(prop.to_string(), "border-left: 100% double;");
    }

    #[test]
    fn test_border_left_with_color() {
        let prop = border_left_with_color(BorderStyle::Outset, Color::Hex("#f33".to_string()));
        assert_eq!(prop.to_string(), "border-left: outset #f33;");

        let prop = border_left_with_color(BorderStyle::Solid, Color::Red);
        assert_eq!(prop.to_string(), "border-left: solid red;");

        let prop = border_left_with_color(BorderStyle::Dashed, Color::Rgb(255, 0, 0));
        assert_eq!(prop.to_string(), "border-left: dashed rgb(255, 0, 0);");

        let prop = border_left_with_color(BorderStyle::Dotted, Color::Rgba(0, 0, 255, 0.5));
        assert_eq!(prop.to_string(), "border-left: dotted rgba(0, 0, 255, 0.5);");
    }

    #[test]
    fn test_border_left_with_width_and_color() {
        let prop = border_left_with_width_and_color(Size::Px(3), BorderStyle::Dashed, Color::Green);
        assert_eq!(prop.to_string(), "border-left: 3px dashed green;");

        let prop = border_left_with_width_and_color(Size::Px(4), BorderStyle::Double, Color::Rgb(50, 161, 206));
        assert_eq!(prop.to_string(), "border-left: 4px double rgb(50, 161, 206);");

        let prop = border_left_with_width_and_color(Size::Rem(0.25), BorderStyle::Ridge, Color::Rgba(211, 220, 50, 0.6));
        assert_eq!(prop.to_string(), "border-left: 0.25rem ridge rgba(211, 220, 50, 0.6);");
    }
}