//! # Border Top Property
//!
//! This module provides a function for creating the CSS `border-top` property.
//! The `border-top` property is a shorthand property that sets the border-top-width, border-top-style, and border-top-color.
//!
//! ## Syntax
//!
//! ```css
//! /* style */
//! border-top: solid;
//!
//! /* width | style */
//! border-top: 2px dotted;
//!
//! /* style | color */
//! border-top: outset #f33;
//!
//! /* width | style | color */
//! border-top: medium dashed green;
//!
//! /* Global values */
//! border-top: inherit;
//! border-top: initial;
//! border-top: unset;
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use mew_css::properties::border_top;
//! use mew_css::values::{BorderStyle, Color, Size};
//!
//! // Style only
//! let prop = border_top::border_top(BorderStyle::Solid);
//! assert_eq!(prop.to_string(), "border-top: solid;");
//!
//! // Width and style
//! let prop = border_top::border_top_with_width(Size::Px(2), BorderStyle::Dotted);
//! assert_eq!(prop.to_string(), "border-top: 2px dotted;");
//!
//! // Style and color
//! let prop = border_top::border_top_with_color(BorderStyle::Outset, Color::Hex("#f33".to_string()));
//! assert_eq!(prop.to_string(), "border-top: outset #f33;");
//!
//! // Width, style, and color
//! let prop = border_top::border_top_with_width_and_color(Size::Px(3), BorderStyle::Dashed, Color::Green);
//! assert_eq!(prop.to_string(), "border-top: 3px dashed green;");
//! ```

use crate::properties::Property;
use crate::values::{BorderStyle, Color, Size};
use std::fmt;

/// A struct to represent border-top property values
struct BorderTopValue {
    width: Option<Size>,
    style: BorderStyle,
    color: Option<Color>,
}

impl fmt::Display for BorderTopValue {
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

/// Creates a CSS `border-top` property with only style.
///
/// The `border-top` property is a shorthand property that sets the border-top-width, border-top-style, and border-top-color.
/// This function sets only the border-top style.
///
/// ## Values
///
/// - `style`: The border-top style (solid, dashed, dotted, etc.)
///
/// # Arguments
///
/// * `style` - The border-top style to use
///
/// # Returns
///
/// A new `Property` instance representing the border-top property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::border_top;
/// use mew_css::values::BorderStyle;
///
/// let prop = border_top::border_top(BorderStyle::Solid);
/// assert_eq!(prop.to_string(), "border-top: solid;");
///
/// let prop = border_top::border_top(BorderStyle::Dashed);
/// assert_eq!(prop.to_string(), "border-top: dashed;");
/// ```
pub fn border_top(style: BorderStyle) -> Property {
    let value = BorderTopValue {
        width: None,
        style,
        color: None,
    };

    Property::new("border-top", value)
}

/// Creates a CSS `border-top` property with style and width.
///
/// The `border-top` property is a shorthand property that sets the border-top-width, border-top-style, and border-top-color.
/// This function sets the border-top width and style.
///
/// ## Values
///
/// - `width`: The border-top width (px, em, rem, etc.)
/// - `style`: The border-top style (solid, dashed, dotted, etc.)
///
/// # Arguments
///
/// * `width` - The border-top width to use
/// * `style` - The border-top style to use
///
/// # Returns
///
/// A new `Property` instance representing the border-top property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::border_top;
/// use mew_css::values::{BorderStyle, Size};
///
/// let prop = border_top::border_top_with_width(Size::Px(2), BorderStyle::Dotted);
/// assert_eq!(prop.to_string(), "border-top: 2px dotted;");
///
/// let prop = border_top::border_top_with_width(Size::Rem(1.5), BorderStyle::Solid);
/// assert_eq!(prop.to_string(), "border-top: 1.5rem solid;");
/// ```
pub fn border_top_with_width(width: Size, style: BorderStyle) -> Property {
    let value = BorderTopValue {
        width: Some(width),
        style,
        color: None,
    };

    Property::new("border-top", value)
}

/// Creates a CSS `border-top` property with style and color.
///
/// The `border-top` property is a shorthand property that sets the border-top-width, border-top-style, and border-top-color.
/// This function sets the border-top style and color.
///
/// ## Values
///
/// - `style`: The border-top style (solid, dashed, dotted, etc.)
/// - `color`: The border-top color (named colors, RGB, RGBA, Hex, etc.)
///
/// # Arguments
///
/// * `style` - The border-top style to use
/// * `color` - The border-top color to use
///
/// # Returns
///
/// A new `Property` instance representing the border-top property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::border_top;
/// use mew_css::values::{BorderStyle, Color};
///
/// let prop = border_top::border_top_with_color(BorderStyle::Outset, Color::Hex("#f33".to_string()));
/// assert_eq!(prop.to_string(), "border-top: outset #f33;");
///
/// let prop = border_top::border_top_with_color(BorderStyle::Solid, Color::Red);
/// assert_eq!(prop.to_string(), "border-top: solid red;");
/// ```
pub fn border_top_with_color(style: BorderStyle, color: Color) -> Property {
    let value = BorderTopValue {
        width: None,
        style,
        color: Some(color),
    };

    Property::new("border-top", value)
}

/// Creates a CSS `border-top` property with width, style, and color.
///
/// The `border-top` property is a shorthand property that sets the border-top-width, border-top-style, and border-top-color.
/// This function sets all three properties.
///
/// ## Values
///
/// - `width`: The border-top width (px, em, rem, etc.)
/// - `style`: The border-top style (solid, dashed, dotted, etc.)
/// - `color`: The border-top color (named colors, RGB, RGBA, Hex, etc.)
///
/// # Arguments
///
/// * `width` - The border-top width to use
/// * `style` - The border-top style to use
/// * `color` - The border-top color to use
///
/// # Returns
///
/// A new `Property` instance representing the border-top property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::border_top;
/// use mew_css::values::{BorderStyle, Color, Size};
///
/// let prop = border_top::border_top_with_width_and_color(Size::Px(3), BorderStyle::Dashed, Color::Green);
/// assert_eq!(prop.to_string(), "border-top: 3px dashed green;");
///
/// let prop = border_top::border_top_with_width_and_color(Size::Px(4), BorderStyle::Double, Color::Rgb(50, 161, 206));
/// assert_eq!(prop.to_string(), "border-top: 4px double rgb(50, 161, 206);");
/// ```
pub fn border_top_with_width_and_color(width: Size, style: BorderStyle, color: Color) -> Property {
    let value = BorderTopValue {
        width: Some(width),
        style,
        color: Some(color),
    };

    Property::new("border-top", value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::{BorderStyle, Color, Size};

    #[test]
    fn test_border_top_style_only() {
        let prop = border_top(BorderStyle::Solid);
        assert_eq!(prop.to_string(), "border-top: solid;");

        let prop = border_top(BorderStyle::Dashed);
        assert_eq!(prop.to_string(), "border-top: dashed;");

        let prop = border_top(BorderStyle::Dotted);
        assert_eq!(prop.to_string(), "border-top: dotted;");

        let prop = border_top(BorderStyle::Double);
        assert_eq!(prop.to_string(), "border-top: double;");

        let prop = border_top(BorderStyle::Groove);
        assert_eq!(prop.to_string(), "border-top: groove;");

        let prop = border_top(BorderStyle::Ridge);
        assert_eq!(prop.to_string(), "border-top: ridge;");

        let prop = border_top(BorderStyle::Inset);
        assert_eq!(prop.to_string(), "border-top: inset;");

        let prop = border_top(BorderStyle::Outset);
        assert_eq!(prop.to_string(), "border-top: outset;");

        let prop = border_top(BorderStyle::None);
        assert_eq!(prop.to_string(), "border-top: none;");
    }

    #[test]
    fn test_border_top_with_width() {
        let prop = border_top_with_width(Size::Px(2), BorderStyle::Dotted);
        assert_eq!(prop.to_string(), "border-top: 2px dotted;");

        let prop = border_top_with_width(Size::Rem(1.5), BorderStyle::Solid);
        assert_eq!(prop.to_string(), "border-top: 1.5rem solid;");

        let prop = border_top_with_width(Size::Em(0.5), BorderStyle::Dashed);
        assert_eq!(prop.to_string(), "border-top: 0.5em dashed;");

        let prop = border_top_with_width(Size::Percent(100.0), BorderStyle::Double);
        assert_eq!(prop.to_string(), "border-top: 100% double;");
    }

    #[test]
    fn test_border_top_with_color() {
        let prop = border_top_with_color(BorderStyle::Outset, Color::Hex("#f33".to_string()));
        assert_eq!(prop.to_string(), "border-top: outset #f33;");

        let prop = border_top_with_color(BorderStyle::Solid, Color::Red);
        assert_eq!(prop.to_string(), "border-top: solid red;");

        let prop = border_top_with_color(BorderStyle::Dashed, Color::Rgb(255, 0, 0));
        assert_eq!(prop.to_string(), "border-top: dashed rgb(255, 0, 0);");

        let prop = border_top_with_color(BorderStyle::Dotted, Color::Rgba(0, 0, 255, 0.5));
        assert_eq!(prop.to_string(), "border-top: dotted rgba(0, 0, 255, 0.5);");
    }

    #[test]
    fn test_border_top_with_width_and_color() {
        let prop = border_top_with_width_and_color(Size::Px(3), BorderStyle::Dashed, Color::Green);
        assert_eq!(prop.to_string(), "border-top: 3px dashed green;");

        let prop = border_top_with_width_and_color(Size::Px(4), BorderStyle::Double, Color::Rgb(50, 161, 206));
        assert_eq!(prop.to_string(), "border-top: 4px double rgb(50, 161, 206);");

        let prop = border_top_with_width_and_color(Size::Rem(0.25), BorderStyle::Ridge, Color::Rgba(211, 220, 50, 0.6));
        assert_eq!(prop.to_string(), "border-top: 0.25rem ridge rgba(211, 220, 50, 0.6);");
    }
}