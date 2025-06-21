//! # Border Right Property
//!
//! This module provides a function for creating the CSS `border-right` property.
//! The `border-right` property is a shorthand property that sets the border-right-width, border-right-style, and border-right-color.
//!
//! ## Syntax
//!
//! ```css
//! /* style */
//! border-right: solid;
//!
//! /* width | style */
//! border-right: 2px dotted;
//!
//! /* style | color */
//! border-right: outset #f33;
//!
//! /* width | style | color */
//! border-right: medium dashed green;
//!
//! /* Global values */
//! border-right: inherit;
//! border-right: initial;
//! border-right: unset;
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use mew_css::properties::border_right;
//! use mew_css::values::{BorderStyle, Color, Size};
//!
//! // Style only
//! let prop = border_right::border_right(BorderStyle::Solid);
//! assert_eq!(prop.to_string(), "border-right: solid;");
//!
//! // Width and style
//! let prop = border_right::border_right_with_width(Size::Px(2), BorderStyle::Dotted);
//! assert_eq!(prop.to_string(), "border-right: 2px dotted;");
//!
//! // Style and color
//! let prop = border_right::border_right_with_color(BorderStyle::Outset, Color::Hex("#f33".to_string()));
//! assert_eq!(prop.to_string(), "border-right: outset #f33;");
//!
//! // Width, style, and color
//! let prop = border_right::border_right_with_width_and_color(Size::Px(3), BorderStyle::Dashed, Color::Green);
//! assert_eq!(prop.to_string(), "border-right: 3px dashed green;");
//! ```

use crate::properties::Property;
use crate::values::{BorderStyle, Color, Size};
use std::fmt;

/// A struct to represent border-right property values
struct BorderRightValue {
    width: Option<Size>,
    style: BorderStyle,
    color: Option<Color>,
}

impl fmt::Display for BorderRightValue {
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

/// Creates a CSS `border-right` property with only style.
///
/// The `border-right` property is a shorthand property that sets the border-right-width, border-right-style, and border-right-color.
/// This function sets only the border-right style.
///
/// ## Values
///
/// - `style`: The border-right style (solid, dashed, dotted, etc.)
///
/// # Arguments
///
/// * `style` - The border-right style to use
///
/// # Returns
///
/// A new `Property` instance representing the border-right property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::border_right;
/// use mew_css::values::BorderStyle;
///
/// let prop = border_right::border_right(BorderStyle::Solid);
/// assert_eq!(prop.to_string(), "border-right: solid;");
///
/// let prop = border_right::border_right(BorderStyle::Dashed);
/// assert_eq!(prop.to_string(), "border-right: dashed;");
/// ```
pub fn border_right(style: BorderStyle) -> Property {
    let value = BorderRightValue {
        width: None,
        style,
        color: None,
    };

    Property::new("border-right", value)
}

/// Creates a CSS `border-right` property with style and width.
///
/// The `border-right` property is a shorthand property that sets the border-right-width, border-right-style, and border-right-color.
/// This function sets the border-right width and style.
///
/// ## Values
///
/// - `width`: The border-right width (px, em, rem, etc.)
/// - `style`: The border-right style (solid, dashed, dotted, etc.)
///
/// # Arguments
///
/// * `width` - The border-right width to use
/// * `style` - The border-right style to use
///
/// # Returns
///
/// A new `Property` instance representing the border-right property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::border_right;
/// use mew_css::values::{BorderStyle, Size};
///
/// let prop = border_right::border_right_with_width(Size::Px(2), BorderStyle::Dotted);
/// assert_eq!(prop.to_string(), "border-right: 2px dotted;");
///
/// let prop = border_right::border_right_with_width(Size::Rem(1.5), BorderStyle::Solid);
/// assert_eq!(prop.to_string(), "border-right: 1.5rem solid;");
/// ```
pub fn border_right_with_width(width: Size, style: BorderStyle) -> Property {
    let value = BorderRightValue {
        width: Some(width),
        style,
        color: None,
    };

    Property::new("border-right", value)
}

/// Creates a CSS `border-right` property with style and color.
///
/// The `border-right` property is a shorthand property that sets the border-right-width, border-right-style, and border-right-color.
/// This function sets the border-right style and color.
///
/// ## Values
///
/// - `style`: The border-right style (solid, dashed, dotted, etc.)
/// - `color`: The border-right color (named colors, RGB, RGBA, Hex, etc.)
///
/// # Arguments
///
/// * `style` - The border-right style to use
/// * `color` - The border-right color to use
///
/// # Returns
///
/// A new `Property` instance representing the border-right property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::border_right;
/// use mew_css::values::{BorderStyle, Color};
///
/// let prop = border_right::border_right_with_color(BorderStyle::Outset, Color::Hex("#f33".to_string()));
/// assert_eq!(prop.to_string(), "border-right: outset #f33;");
///
/// let prop = border_right::border_right_with_color(BorderStyle::Solid, Color::Red);
/// assert_eq!(prop.to_string(), "border-right: solid red;");
/// ```
pub fn border_right_with_color(style: BorderStyle, color: Color) -> Property {
    let value = BorderRightValue {
        width: None,
        style,
        color: Some(color),
    };

    Property::new("border-right", value)
}

/// Creates a CSS `border-right` property with width, style, and color.
///
/// The `border-right` property is a shorthand property that sets the border-right-width, border-right-style, and border-right-color.
/// This function sets all three properties.
///
/// ## Values
///
/// - `width`: The border-right width (px, em, rem, etc.)
/// - `style`: The border-right style (solid, dashed, dotted, etc.)
/// - `color`: The border-right color (named colors, RGB, RGBA, Hex, etc.)
///
/// # Arguments
///
/// * `width` - The border-right width to use
/// * `style` - The border-right style to use
/// * `color` - The border-right color to use
///
/// # Returns
///
/// A new `Property` instance representing the border-right property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::border_right;
/// use mew_css::values::{BorderStyle, Color, Size};
///
/// let prop = border_right::border_right_with_width_and_color(Size::Px(3), BorderStyle::Dashed, Color::Green);
/// assert_eq!(prop.to_string(), "border-right: 3px dashed green;");
///
/// let prop = border_right::border_right_with_width_and_color(Size::Px(4), BorderStyle::Double, Color::Rgb(50, 161, 206));
/// assert_eq!(prop.to_string(), "border-right: 4px double rgb(50, 161, 206);");
/// ```
pub fn border_right_with_width_and_color(width: Size, style: BorderStyle, color: Color) -> Property {
    let value = BorderRightValue {
        width: Some(width),
        style,
        color: Some(color),
    };

    Property::new("border-right", value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::{BorderStyle, Color, Size};

    #[test]
    fn test_border_right_style_only() {
        let prop = border_right(BorderStyle::Solid);
        assert_eq!(prop.to_string(), "border-right: solid;");

        let prop = border_right(BorderStyle::Dashed);
        assert_eq!(prop.to_string(), "border-right: dashed;");

        let prop = border_right(BorderStyle::Dotted);
        assert_eq!(prop.to_string(), "border-right: dotted;");

        let prop = border_right(BorderStyle::Double);
        assert_eq!(prop.to_string(), "border-right: double;");

        let prop = border_right(BorderStyle::Groove);
        assert_eq!(prop.to_string(), "border-right: groove;");

        let prop = border_right(BorderStyle::Ridge);
        assert_eq!(prop.to_string(), "border-right: ridge;");

        let prop = border_right(BorderStyle::Inset);
        assert_eq!(prop.to_string(), "border-right: inset;");

        let prop = border_right(BorderStyle::Outset);
        assert_eq!(prop.to_string(), "border-right: outset;");

        let prop = border_right(BorderStyle::None);
        assert_eq!(prop.to_string(), "border-right: none;");
    }

    #[test]
    fn test_border_right_with_width() {
        let prop = border_right_with_width(Size::Px(2), BorderStyle::Dotted);
        assert_eq!(prop.to_string(), "border-right: 2px dotted;");

        let prop = border_right_with_width(Size::Rem(1.5), BorderStyle::Solid);
        assert_eq!(prop.to_string(), "border-right: 1.5rem solid;");

        let prop = border_right_with_width(Size::Em(0.5), BorderStyle::Dashed);
        assert_eq!(prop.to_string(), "border-right: 0.5em dashed;");

        let prop = border_right_with_width(Size::Percent(100.0), BorderStyle::Double);
        assert_eq!(prop.to_string(), "border-right: 100% double;");
    }

    #[test]
    fn test_border_right_with_color() {
        let prop = border_right_with_color(BorderStyle::Outset, Color::Hex("#f33".to_string()));
        assert_eq!(prop.to_string(), "border-right: outset #f33;");

        let prop = border_right_with_color(BorderStyle::Solid, Color::Red);
        assert_eq!(prop.to_string(), "border-right: solid red;");

        let prop = border_right_with_color(BorderStyle::Dashed, Color::Rgb(255, 0, 0));
        assert_eq!(prop.to_string(), "border-right: dashed rgb(255, 0, 0);");

        let prop = border_right_with_color(BorderStyle::Dotted, Color::Rgba(0, 0, 255, 0.5));
        assert_eq!(prop.to_string(), "border-right: dotted rgba(0, 0, 255, 0.5);");
    }

    #[test]
    fn test_border_right_with_width_and_color() {
        let prop = border_right_with_width_and_color(Size::Px(3), BorderStyle::Dashed, Color::Green);
        assert_eq!(prop.to_string(), "border-right: 3px dashed green;");

        let prop = border_right_with_width_and_color(Size::Px(4), BorderStyle::Double, Color::Rgb(50, 161, 206));
        assert_eq!(prop.to_string(), "border-right: 4px double rgb(50, 161, 206);");

        let prop = border_right_with_width_and_color(Size::Rem(0.25), BorderStyle::Ridge, Color::Rgba(211, 220, 50, 0.6));
        assert_eq!(prop.to_string(), "border-right: 0.25rem ridge rgba(211, 220, 50, 0.6);");
    }
}