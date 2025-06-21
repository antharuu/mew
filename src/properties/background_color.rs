//! # Background Color Property
//!
//! This module provides a function for creating the CSS `background-color` property.
//! The `background-color` property sets the background color of an element.
//!
//! ## Syntax
//!
//! ```css
//! /* Keyword values */
//! background-color: red;
//!
//! /* Hexadecimal value */
//! background-color: #bbff00;    /* Fully opaque */
//! background-color: #bf0;       /* Fully opaque (shorthand) */
//! background-color: #11ffee00;  /* Fully transparent */
//! background-color: #1fe0;      /* Fully transparent (shorthand) */
//! background-color: #11ffeeff;  /* Fully opaque */
//! background-color: #1fef;      /* Fully opaque (shorthand) */
//!
//! /* RGB value */
//! background-color: rgb(255, 255, 128);
//!
//! /* RGBA value */
//! background-color: rgba(117, 190, 218, 0);     /* 0.0 - transparent */
//! background-color: rgba(117, 190, 218, 0.5);   /* 0.5 - semi-transparent */
//! background-color: rgba(117, 190, 218, 1);     /* 1.0 - opaque */
//!
//! /* HSL value */
//! background-color: hsl(50, 33%, 25%);
//!
//! /* HSLA value */
//! background-color: hsla(50, 33%, 25%, 0.75);
//!
//! /* Special keyword values */
//! background-color: currentcolor;
//! background-color: transparent;
//!
//! /* Global values */
//! background-color: inherit;
//! background-color: initial;
//! background-color: unset;
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use mew_css::properties::background_color;
//! use mew_css::values::Color;
//!
//! let prop = background_color::background_color(Color::Red);
//! assert_eq!(prop.to_string(), "background-color: red;");
//!
//! let prop = background_color::background_color(Color::Rgb(255, 255, 128));
//! assert_eq!(prop.to_string(), "background-color: rgb(255, 255, 128);");
//! ```

use crate::properties::Property;
use crate::values::Color;

/// Creates a CSS `background-color` property.
///
/// The `background-color` property sets the background color of an element.
///
/// ## Values
///
/// - Named colors: `Red`, `Blue`, `Green`, etc.
/// - RGB: `Rgb(255, 0, 0)` for red
/// - RGBA: `Rgba(255, 0, 0, 0.5)` for semi-transparent red
/// - Hex: `Hex("#ff0000")` for red
/// - HSL: `Hsl(0, 100, 50)` for red
/// - HSLA: `Hsla(0, 100, 50, 0.5)` for semi-transparent red
/// - Special values: `Transparent`, `CurrentColor`
/// - Global values: `Inherit`
///
/// # Arguments
///
/// * `value` - The color value to use
///
/// # Returns
///
/// A new `Property` instance representing the background-color property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::background_color;
/// use mew_css::values::Color;
///
/// // Named color
/// let prop = background_color::background_color(Color::Red);
/// assert_eq!(prop.to_string(), "background-color: red;");
///
/// // RGB color
/// let prop = background_color::background_color(Color::Rgb(255, 0, 0));
/// assert_eq!(prop.to_string(), "background-color: rgb(255, 0, 0);");
///
/// // RGBA color (with transparency)
/// let prop = background_color::background_color(Color::Rgba(255, 0, 0, 0.5));
/// assert_eq!(prop.to_string(), "background-color: rgba(255, 0, 0, 0.5);");
///
/// // Hex color
/// let prop = background_color::background_color(Color::Hex("#ff0000".to_string()));
/// assert_eq!(prop.to_string(), "background-color: #ff0000;");
///
/// // HSL color
/// let prop = background_color::background_color(Color::Hsl(0, 100, 50));
/// assert_eq!(prop.to_string(), "background-color: hsl(0, 100%, 50%);");
///
/// // HSLA color (with transparency)
/// let prop = background_color::background_color(Color::Hsla(0, 100, 50, 0.5));
/// assert_eq!(prop.to_string(), "background-color: hsla(0, 100%, 50%, 0.5);");
///
/// // Special values
/// let prop = background_color::background_color(Color::Transparent);
/// assert_eq!(prop.to_string(), "background-color: transparent;");
///
/// let prop = background_color::background_color(Color::CurrentColor);
/// assert_eq!(prop.to_string(), "background-color: currentColor;");
///
/// // Global values
/// let prop = background_color::background_color(Color::Inherit);
/// assert_eq!(prop.to_string(), "background-color: inherit;");
/// ```
pub fn background_color(value: Color) -> Property {
    Property::new("background-color", value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::Color;

    #[test]
    fn test_named_colors() {
        let prop = background_color(Color::Red);
        assert_eq!(prop.to_string(), "background-color: red;");

        let prop = background_color(Color::Blue);
        assert_eq!(prop.to_string(), "background-color: blue;");

        let prop = background_color(Color::Green);
        assert_eq!(prop.to_string(), "background-color: green;");
    }

    #[test]
    fn test_rgb_colors() {
        let prop = background_color(Color::Rgb(255, 0, 0));
        assert_eq!(prop.to_string(), "background-color: rgb(255, 0, 0);");

        let prop = background_color(Color::Rgb(0, 255, 0));
        assert_eq!(prop.to_string(), "background-color: rgb(0, 255, 0);");

        let prop = background_color(Color::Rgb(0, 0, 255));
        assert_eq!(prop.to_string(), "background-color: rgb(0, 0, 255);");
    }

    #[test]
    fn test_rgba_colors() {
        let prop = background_color(Color::Rgba(255, 0, 0, 1.0));
        assert_eq!(prop.to_string(), "background-color: rgba(255, 0, 0, 1);");

        let prop = background_color(Color::Rgba(0, 255, 0, 0.5));
        assert_eq!(prop.to_string(), "background-color: rgba(0, 255, 0, 0.5);");

        let prop = background_color(Color::Rgba(0, 0, 255, 0.0));
        assert_eq!(prop.to_string(), "background-color: rgba(0, 0, 255, 0);");
    }

    #[test]
    fn test_hex_colors() {
        let prop = background_color(Color::Hex("#ff0000".to_string()));
        assert_eq!(prop.to_string(), "background-color: #ff0000;");

        let prop = background_color(Color::Hex("#00ff00".to_string()));
        assert_eq!(prop.to_string(), "background-color: #00ff00;");

        let prop = background_color(Color::Hex("#0000ff".to_string()));
        assert_eq!(prop.to_string(), "background-color: #0000ff;");
    }

    #[test]
    fn test_hsl_colors() {
        let prop = background_color(Color::Hsl(0, 100, 50));
        assert_eq!(prop.to_string(), "background-color: hsl(0, 100%, 50%);");

        let prop = background_color(Color::Hsl(120, 100, 50));
        assert_eq!(prop.to_string(), "background-color: hsl(120, 100%, 50%);");

        let prop = background_color(Color::Hsl(240, 100, 50));
        assert_eq!(prop.to_string(), "background-color: hsl(240, 100%, 50%);");
    }

    #[test]
    fn test_hsla_colors() {
        let prop = background_color(Color::Hsla(0, 100, 50, 1.0));
        assert_eq!(prop.to_string(), "background-color: hsla(0, 100%, 50%, 1);");

        let prop = background_color(Color::Hsla(120, 100, 50, 0.5));
        assert_eq!(prop.to_string(), "background-color: hsla(120, 100%, 50%, 0.5);");

        let prop = background_color(Color::Hsla(240, 100, 50, 0.0));
        assert_eq!(prop.to_string(), "background-color: hsla(240, 100%, 50%, 0);");
    }

    #[test]
    fn test_special_values() {
        let prop = background_color(Color::Transparent);
        assert_eq!(prop.to_string(), "background-color: transparent;");

        let prop = background_color(Color::CurrentColor);
        assert_eq!(prop.to_string(), "background-color: currentColor;");
    }

    #[test]
    fn test_global_values() {
        let prop = background_color(Color::Inherit);
        assert_eq!(prop.to_string(), "background-color: inherit;");
    }
}
