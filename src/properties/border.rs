//! # Border Property
//!
//! This module provides a function for creating the CSS `border` property.
//! The `border` property is a shorthand property that sets the border-width, border-style, and border-color.
//!
//! ## Syntax
//!
//! ```css
//! /* style */
//! border: solid;
//!
//! /* width | style */
//! border: 2px dotted;
//!
//! /* style | color */
//! border: outset #f33;
//!
//! /* width | style | color */
//! border: medium dashed green;
//!
//! /* Global values */
//! border: inherit;
//! border: initial;
//! border: unset;
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use mew_css::properties::border;
//! use mew_css::values::{BorderStyle, Color, Size};
//!
//! // Style only
//! let prop = border::border(BorderStyle::Solid);
//! assert_eq!(prop.to_string(), "border: solid;");
//!
//! // Width and style
//! let prop = border::border_with_width(Size::Px(2), BorderStyle::Dotted);
//! assert_eq!(prop.to_string(), "border: 2px dotted;");
//!
//! // Style and color
//! let prop = border::border_with_color(BorderStyle::Outset, Color::Hex("#f33".to_string()));
//! assert_eq!(prop.to_string(), "border: outset #f33;");
//!
//! // Width, style, and color
//! let prop = border::border_with_width_and_color(Size::Px(3), BorderStyle::Dashed, Color::Green);
//! assert_eq!(prop.to_string(), "border: 3px dashed green;");
//! ```

use crate::properties::Property;
use crate::values::{BorderStyle, Color, Size};
use std::fmt;

/// A struct to represent border property values
struct BorderValue {
    width: Option<Size>,
    style: BorderStyle,
    color: Option<Color>,
}

impl fmt::Display for BorderValue {
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

/// Creates a CSS `border` property with only style.
///
/// The `border` property is a shorthand property that sets the border-width, border-style, and border-color.
/// This function sets only the border style.
///
/// ## Values
///
/// - `style`: The border style (solid, dashed, dotted, etc.)
///
/// # Arguments
///
/// * `style` - The border style to use
///
/// # Returns
///
/// A new `Property` instance representing the border property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::border;
/// use mew_css::values::BorderStyle;
///
/// let prop = border::border(BorderStyle::Solid);
/// assert_eq!(prop.to_string(), "border: solid;");
///
/// let prop = border::border(BorderStyle::Dashed);
/// assert_eq!(prop.to_string(), "border: dashed;");
/// ```
pub fn border(style: BorderStyle) -> Property {
    let value = BorderValue {
        width: None,
        style,
        color: None,
    };

    Property::new("border", value)
}

/// Creates a CSS `border` property with style and width.
///
/// The `border` property is a shorthand property that sets the border-width, border-style, and border-color.
/// This function sets the border width and style.
///
/// ## Values
///
/// - `width`: The border width (px, em, rem, etc.)
/// - `style`: The border style (solid, dashed, dotted, etc.)
///
/// # Arguments
///
/// * `width` - The border width to use
/// * `style` - The border style to use
///
/// # Returns
///
/// A new `Property` instance representing the border property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::border;
/// use mew_css::values::{BorderStyle, Size};
///
/// let prop = border::border_with_width(Size::Px(2), BorderStyle::Dotted);
/// assert_eq!(prop.to_string(), "border: 2px dotted;");
///
/// let prop = border::border_with_width(Size::Rem(1.5), BorderStyle::Solid);
/// assert_eq!(prop.to_string(), "border: 1.5rem solid;");
/// ```
pub fn border_with_width(width: Size, style: BorderStyle) -> Property {
    let value = BorderValue {
        width: Some(width),
        style,
        color: None,
    };

    Property::new("border", value)
}

/// Creates a CSS `border` property with style and color.
///
/// The `border` property is a shorthand property that sets the border-width, border-style, and border-color.
/// This function sets the border style and color.
///
/// ## Values
///
/// - `style`: The border style (solid, dashed, dotted, etc.)
/// - `color`: The border color (named colors, RGB, RGBA, Hex, etc.)
///
/// # Arguments
///
/// * `style` - The border style to use
/// * `color` - The border color to use
///
/// # Returns
///
/// A new `Property` instance representing the border property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::border;
/// use mew_css::values::{BorderStyle, Color};
///
/// let prop = border::border_with_color(BorderStyle::Outset, Color::Hex("#f33".to_string()));
/// assert_eq!(prop.to_string(), "border: outset #f33;");
///
/// let prop = border::border_with_color(BorderStyle::Solid, Color::Red);
/// assert_eq!(prop.to_string(), "border: solid red;");
/// ```
pub fn border_with_color(style: BorderStyle, color: Color) -> Property {
    let value = BorderValue {
        width: None,
        style,
        color: Some(color),
    };

    Property::new("border", value)
}

/// Creates a CSS `border` property with width, style, and color.
///
/// The `border` property is a shorthand property that sets the border-width, border-style, and border-color.
/// This function sets all three properties.
///
/// ## Values
///
/// - `width`: The border width (px, em, rem, etc.)
/// - `style`: The border style (solid, dashed, dotted, etc.)
/// - `color`: The border color (named colors, RGB, RGBA, Hex, etc.)
///
/// # Arguments
///
/// * `width` - The border width to use
/// * `style` - The border style to use
/// * `color` - The border color to use
///
/// # Returns
///
/// A new `Property` instance representing the border property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::border;
/// use mew_css::values::{BorderStyle, Color, Size};
///
/// let prop = border::border_with_width_and_color(Size::Px(3), BorderStyle::Dashed, Color::Green);
/// assert_eq!(prop.to_string(), "border: 3px dashed green;");
///
/// let prop = border::border_with_width_and_color(Size::Px(4), BorderStyle::Double, Color::Rgb(50, 161, 206));
/// assert_eq!(prop.to_string(), "border: 4px double rgb(50, 161, 206);");
/// ```
pub fn border_with_width_and_color(width: Size, style: BorderStyle, color: Color) -> Property {
    let value = BorderValue {
        width: Some(width),
        style,
        color: Some(color),
    };

    Property::new("border", value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::{BorderStyle, Color, Size};

    #[test]
    fn test_border_style_only() {
        let prop = border(BorderStyle::Solid);
        assert_eq!(prop.to_string(), "border: solid;");

        let prop = border(BorderStyle::Dashed);
        assert_eq!(prop.to_string(), "border: dashed;");

        let prop = border(BorderStyle::Dotted);
        assert_eq!(prop.to_string(), "border: dotted;");

        let prop = border(BorderStyle::Double);
        assert_eq!(prop.to_string(), "border: double;");

        let prop = border(BorderStyle::Groove);
        assert_eq!(prop.to_string(), "border: groove;");

        let prop = border(BorderStyle::Ridge);
        assert_eq!(prop.to_string(), "border: ridge;");

        let prop = border(BorderStyle::Inset);
        assert_eq!(prop.to_string(), "border: inset;");

        let prop = border(BorderStyle::Outset);
        assert_eq!(prop.to_string(), "border: outset;");

        let prop = border(BorderStyle::None);
        assert_eq!(prop.to_string(), "border: none;");
    }

    #[test]
    fn test_border_with_width() {
        let prop = border_with_width(Size::Px(2), BorderStyle::Dotted);
        assert_eq!(prop.to_string(), "border: 2px dotted;");

        let prop = border_with_width(Size::Rem(1.5), BorderStyle::Solid);
        assert_eq!(prop.to_string(), "border: 1.5rem solid;");

        let prop = border_with_width(Size::Em(0.5), BorderStyle::Dashed);
        assert_eq!(prop.to_string(), "border: 0.5em dashed;");

        let prop = border_with_width(Size::Percent(100.0), BorderStyle::Double);
        assert_eq!(prop.to_string(), "border: 100% double;");
    }

    #[test]
    fn test_border_with_color() {
        let prop = border_with_color(BorderStyle::Outset, Color::Hex("#f33".to_string()));
        assert_eq!(prop.to_string(), "border: outset #f33;");

        let prop = border_with_color(BorderStyle::Solid, Color::Red);
        assert_eq!(prop.to_string(), "border: solid red;");

        let prop = border_with_color(BorderStyle::Dashed, Color::Rgb(255, 0, 0));
        assert_eq!(prop.to_string(), "border: dashed rgb(255, 0, 0);");

        let prop = border_with_color(BorderStyle::Dotted, Color::Rgba(0, 0, 255, 0.5));
        assert_eq!(prop.to_string(), "border: dotted rgba(0, 0, 255, 0.5);");
    }

    #[test]
    fn test_border_with_width_and_color() {
        let prop = border_with_width_and_color(Size::Px(3), BorderStyle::Dashed, Color::Green);
        assert_eq!(prop.to_string(), "border: 3px dashed green;");

        let prop = border_with_width_and_color(Size::Px(4), BorderStyle::Double, Color::Rgb(50, 161, 206));
        assert_eq!(prop.to_string(), "border: 4px double rgb(50, 161, 206);");

        let prop = border_with_width_and_color(Size::Rem(0.25), BorderStyle::Ridge, Color::Rgba(211, 220, 50, 0.6));
        assert_eq!(prop.to_string(), "border: 0.25rem ridge rgba(211, 220, 50, 0.6);");
    }
}
