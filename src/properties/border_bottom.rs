//! # Border Bottom Property
//!
//! This module provides a function for creating the CSS `border-bottom` property.
//! The `border-bottom` property is a shorthand property that sets the border-bottom-width, border-bottom-style, and border-bottom-color.
//!
//! ## Syntax
//!
//! ```css
//! /* style */
//! border-bottom: solid;
//!
//! /* width | style */
//! border-bottom: 2px dotted;
//!
//! /* style | color */
//! border-bottom: outset #f33;
//!
//! /* width | style | color */
//! border-bottom: medium dashed green;
//!
//! /* Global values */
//! border-bottom: inherit;
//! border-bottom: initial;
//! border-bottom: unset;
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use mew_css::properties::border_bottom;
//! use mew_css::values::{BorderStyle, Color, Size};
//!
//! // Style only
//! let prop = border_bottom::border_bottom(BorderStyle::Solid);
//! assert_eq!(prop.to_string(), "border-bottom: solid;");
//!
//! // Width and style
//! let prop = border_bottom::border_bottom_with_width(Size::Px(2), BorderStyle::Dotted);
//! assert_eq!(prop.to_string(), "border-bottom: 2px dotted;");
//!
//! // Style and color
//! let prop = border_bottom::border_bottom_with_color(BorderStyle::Outset, Color::Hex("#f33".to_string()));
//! assert_eq!(prop.to_string(), "border-bottom: outset #f33;");
//!
//! // Width, style, and color
//! let prop = border_bottom::border_bottom_with_width_and_color(Size::Px(3), BorderStyle::Dashed, Color::Green);
//! assert_eq!(prop.to_string(), "border-bottom: 3px dashed green;");
//! ```

use crate::properties::Property;
use crate::values::{BorderStyle, Color, Size};
use std::fmt;

/// A struct to represent border-bottom property values
struct BorderBottomValue {
    width: Option<Size>,
    style: BorderStyle,
    color: Option<Color>,
}

impl fmt::Display for BorderBottomValue {
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

/// Creates a CSS `border-bottom` property with only style.
///
/// The `border-bottom` property is a shorthand property that sets the border-bottom-width, border-bottom-style, and border-bottom-color.
/// This function sets only the border-bottom style.
///
/// ## Values
///
/// - `style`: The border-bottom style (solid, dashed, dotted, etc.)
///
/// # Arguments
///
/// * `style` - The border-bottom style to use
///
/// # Returns
///
/// A new `Property` instance representing the border-bottom property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::border_bottom;
/// use mew_css::values::BorderStyle;
///
/// let prop = border_bottom::border_bottom(BorderStyle::Solid);
/// assert_eq!(prop.to_string(), "border-bottom: solid;");
///
/// let prop = border_bottom::border_bottom(BorderStyle::Dashed);
/// assert_eq!(prop.to_string(), "border-bottom: dashed;");
/// ```
pub fn border_bottom(style: BorderStyle) -> Property {
    let value = BorderBottomValue {
        width: None,
        style,
        color: None,
    };

    Property::new("border-bottom", value)
}

/// Creates a CSS `border-bottom` property with style and width.
///
/// The `border-bottom` property is a shorthand property that sets the border-bottom-width, border-bottom-style, and border-bottom-color.
/// This function sets the border-bottom width and style.
///
/// ## Values
///
/// - `width`: The border-bottom width (px, em, rem, etc.)
/// - `style`: The border-bottom style (solid, dashed, dotted, etc.)
///
/// # Arguments
///
/// * `width` - The border-bottom width to use
/// * `style` - The border-bottom style to use
///
/// # Returns
///
/// A new `Property` instance representing the border-bottom property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::border_bottom;
/// use mew_css::values::{BorderStyle, Size};
///
/// let prop = border_bottom::border_bottom_with_width(Size::Px(2), BorderStyle::Dotted);
/// assert_eq!(prop.to_string(), "border-bottom: 2px dotted;");
///
/// let prop = border_bottom::border_bottom_with_width(Size::Rem(1.5), BorderStyle::Solid);
/// assert_eq!(prop.to_string(), "border-bottom: 1.5rem solid;");
/// ```
pub fn border_bottom_with_width(width: Size, style: BorderStyle) -> Property {
    let value = BorderBottomValue {
        width: Some(width),
        style,
        color: None,
    };

    Property::new("border-bottom", value)
}

/// Creates a CSS `border-bottom` property with style and color.
///
/// The `border-bottom` property is a shorthand property that sets the border-bottom-width, border-bottom-style, and border-bottom-color.
/// This function sets the border-bottom style and color.
///
/// ## Values
///
/// - `style`: The border-bottom style (solid, dashed, dotted, etc.)
/// - `color`: The border-bottom color (named colors, RGB, RGBA, Hex, etc.)
///
/// # Arguments
///
/// * `style` - The border-bottom style to use
/// * `color` - The border-bottom color to use
///
/// # Returns
///
/// A new `Property` instance representing the border-bottom property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::border_bottom;
/// use mew_css::values::{BorderStyle, Color};
///
/// let prop = border_bottom::border_bottom_with_color(BorderStyle::Outset, Color::Hex("#f33".to_string()));
/// assert_eq!(prop.to_string(), "border-bottom: outset #f33;");
///
/// let prop = border_bottom::border_bottom_with_color(BorderStyle::Solid, Color::Red);
/// assert_eq!(prop.to_string(), "border-bottom: solid red;");
/// ```
pub fn border_bottom_with_color(style: BorderStyle, color: Color) -> Property {
    let value = BorderBottomValue {
        width: None,
        style,
        color: Some(color),
    };

    Property::new("border-bottom", value)
}

/// Creates a CSS `border-bottom` property with width, style, and color.
///
/// The `border-bottom` property is a shorthand property that sets the border-bottom-width, border-bottom-style, and border-bottom-color.
/// This function sets all three properties.
///
/// ## Values
///
/// - `width`: The border-bottom width (px, em, rem, etc.)
/// - `style`: The border-bottom style (solid, dashed, dotted, etc.)
/// - `color`: The border-bottom color (named colors, RGB, RGBA, Hex, etc.)
///
/// # Arguments
///
/// * `width` - The border-bottom width to use
/// * `style` - The border-bottom style to use
/// * `color` - The border-bottom color to use
///
/// # Returns
///
/// A new `Property` instance representing the border-bottom property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::border_bottom;
/// use mew_css::values::{BorderStyle, Color, Size};
///
/// let prop = border_bottom::border_bottom_with_width_and_color(Size::Px(3), BorderStyle::Dashed, Color::Green);
/// assert_eq!(prop.to_string(), "border-bottom: 3px dashed green;");
///
/// let prop = border_bottom::border_bottom_with_width_and_color(Size::Px(4), BorderStyle::Double, Color::Rgb(50, 161, 206));
/// assert_eq!(prop.to_string(), "border-bottom: 4px double rgb(50, 161, 206);");
/// ```
pub fn border_bottom_with_width_and_color(width: Size, style: BorderStyle, color: Color) -> Property {
    let value = BorderBottomValue {
        width: Some(width),
        style,
        color: Some(color),
    };

    Property::new("border-bottom", value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::{BorderStyle, Color, Size};

    #[test]
    fn test_border_bottom_style_only() {
        let prop = border_bottom(BorderStyle::Solid);
        assert_eq!(prop.to_string(), "border-bottom: solid;");

        let prop = border_bottom(BorderStyle::Dashed);
        assert_eq!(prop.to_string(), "border-bottom: dashed;");

        let prop = border_bottom(BorderStyle::Dotted);
        assert_eq!(prop.to_string(), "border-bottom: dotted;");

        let prop = border_bottom(BorderStyle::Double);
        assert_eq!(prop.to_string(), "border-bottom: double;");

        let prop = border_bottom(BorderStyle::Groove);
        assert_eq!(prop.to_string(), "border-bottom: groove;");

        let prop = border_bottom(BorderStyle::Ridge);
        assert_eq!(prop.to_string(), "border-bottom: ridge;");

        let prop = border_bottom(BorderStyle::Inset);
        assert_eq!(prop.to_string(), "border-bottom: inset;");

        let prop = border_bottom(BorderStyle::Outset);
        assert_eq!(prop.to_string(), "border-bottom: outset;");

        let prop = border_bottom(BorderStyle::None);
        assert_eq!(prop.to_string(), "border-bottom: none;");
    }

    #[test]
    fn test_border_bottom_with_width() {
        let prop = border_bottom_with_width(Size::Px(2), BorderStyle::Dotted);
        assert_eq!(prop.to_string(), "border-bottom: 2px dotted;");

        let prop = border_bottom_with_width(Size::Rem(1.5), BorderStyle::Solid);
        assert_eq!(prop.to_string(), "border-bottom: 1.5rem solid;");

        let prop = border_bottom_with_width(Size::Em(0.5), BorderStyle::Dashed);
        assert_eq!(prop.to_string(), "border-bottom: 0.5em dashed;");

        let prop = border_bottom_with_width(Size::Percent(100.0), BorderStyle::Double);
        assert_eq!(prop.to_string(), "border-bottom: 100% double;");
    }

    #[test]
    fn test_border_bottom_with_color() {
        let prop = border_bottom_with_color(BorderStyle::Outset, Color::Hex("#f33".to_string()));
        assert_eq!(prop.to_string(), "border-bottom: outset #f33;");

        let prop = border_bottom_with_color(BorderStyle::Solid, Color::Red);
        assert_eq!(prop.to_string(), "border-bottom: solid red;");

        let prop = border_bottom_with_color(BorderStyle::Dashed, Color::Rgb(255, 0, 0));
        assert_eq!(prop.to_string(), "border-bottom: dashed rgb(255, 0, 0);");

        let prop = border_bottom_with_color(BorderStyle::Dotted, Color::Rgba(0, 0, 255, 0.5));
        assert_eq!(prop.to_string(), "border-bottom: dotted rgba(0, 0, 255, 0.5);");
    }

    #[test]
    fn test_border_bottom_with_width_and_color() {
        let prop = border_bottom_with_width_and_color(Size::Px(3), BorderStyle::Dashed, Color::Green);
        assert_eq!(prop.to_string(), "border-bottom: 3px dashed green;");

        let prop = border_bottom_with_width_and_color(Size::Px(4), BorderStyle::Double, Color::Rgb(50, 161, 206));
        assert_eq!(prop.to_string(), "border-bottom: 4px double rgb(50, 161, 206);");

        let prop = border_bottom_with_width_and_color(Size::Rem(0.25), BorderStyle::Ridge, Color::Rgba(211, 220, 50, 0.6));
        assert_eq!(prop.to_string(), "border-bottom: 0.25rem ridge rgba(211, 220, 50, 0.6);");
    }
}