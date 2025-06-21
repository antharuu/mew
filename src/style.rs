//! # Style Builder Module
//!
//! This module provides a fluent API for building CSS styles with strong typing.
//! It forms the core of the Mew CSS library, allowing you to chain method calls
//! to build complex CSS styles in a type-safe manner.
//!
//! ## Design Philosophy
//!
//! The style builder follows a fluent interface pattern where each method returns
//! a mutable reference to self, allowing for method chaining. This creates a clean,
//! declarative API that mimics the structure of CSS itself while providing the
//! benefits of Rust's type system.
//!
//! ## Usage Patterns
//!
//! ### Basic Style Creation
//!
//! ```rust
//! use mew_css::style;
//! use mew_css::values::AlignContent;
//!
//! let css = style()
//!     .align_content(AlignContent::Center)
//!     .apply();
//! ```
//!
//! ### Custom Properties
//!
//! ```rust
//! use mew_css::style;
//!
//! let css = style()
//!     .custom_property("animation-name", "fade-in")
//!     .custom_property("animation-duration", "2s")
//!     .apply();
//! ```
//!
//! ### CSS Variables
//!
//! ```rust
//! use mew_css::style;
//!
//! let css = style()
//!     .set_var("primary-color", "#3366ff")
//!     .set_var("spacing", "1rem")
//!     .apply();
//! ```

use crate::properties::{Property, align_content, align_items, background_color, border};
use crate::values::*;
use std::fmt;

/// A CSS style builder that provides a fluent API for creating CSS styles.
///
/// The `Style` struct is the main entry point for building CSS styles. It maintains
/// an internal collection of CSS properties and provides methods for adding and
/// manipulating these properties. The fluent API design allows for chaining method
/// calls to create complex styles in a readable, declarative manner.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use mew_css::style;
/// use mew_css::values::AlignContent;
///
/// let css = style()
///     .align_content(AlignContent::Center)
///     .apply();
/// ```
///
/// The resulting CSS string will be: `align-content: center;`
#[derive(Debug, Default)]
pub struct Style {
    /// Collection of CSS properties that make up this style
    properties: Vec<Property>,
}

impl Style {
    /// Creates a new empty style with no properties.
    ///
    /// This is the starting point for building a CSS style. After creating a new style,
    /// you can chain method calls to add properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mew_css::style::Style;
    ///
    /// let style = Style::new();
    /// ```
    ///
    /// More commonly, you'll use the `style()` function:
    ///
    /// ```rust
    /// use mew_css::style;
    ///
    /// let style = style();
    /// ```
    pub fn new() -> Self {
        Self {
            properties: Vec::new(),
        }
    }

    /// Adds a property to the style and returns a mutable reference to self.
    ///
    /// This is a low-level method used by the property-specific methods. Most users
    /// won't need to call this directly, but it's useful for extending the library
    /// with custom properties.
    ///
    /// # Arguments
    ///
    /// * `property` - The CSS property to add
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mew_css::style;
    /// use mew_css::properties::Property;
    ///
    /// let css = style()
    ///     .add_property(Property::new("color", "blue"))
    ///     .add_property(Property::new("font-size", "16px"))
    ///     .apply();
    /// ```
    pub fn add_property(&mut self, property: Property) -> &mut Self {
        self.properties.push(property);
        self
    }

    /// Generates the final CSS string from all added properties.
    ///
    /// This method should be called after adding all desired properties to generate
    /// the CSS string. It formats each property as `name: value;` and joins them
    /// with spaces.
    ///
    /// # Returns
    ///
    /// A string containing the CSS representation of all properties
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mew_css::style;
    /// use mew_css::values::AlignContent;
    ///
    /// let css = style()
    ///     .align_content(AlignContent::Center)
    ///     .apply();
    ///
    /// assert_eq!(css, "align-content: center;");
    /// ```
    pub fn apply(&self) -> String {
        self.to_string()
    }

    /// Alias for `apply()` that generates the CSS string.
    ///
    /// This method provides an alternative name that might be more intuitive in some contexts.
    ///
    /// # Returns
    ///
    /// A string containing the CSS representation of all properties
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mew_css::style;
    /// use mew_css::values::AlignContent;
    ///
    /// let css = style()
    ///     .align_content(AlignContent::Center)
    ///     .build();
    ///
    /// assert_eq!(css, "align-content: center;");
    /// ```
    pub fn build(&self) -> String {
        self.apply()
    }

    /// Adds a custom property with the given name and value.
    ///
    /// This method allows you to add any CSS property, including those not explicitly
    /// supported by the library. It's useful for experimental properties, vendor-prefixed
    /// properties, or any other property not covered by the built-in methods.
    ///
    /// # Arguments
    ///
    /// * `name` - The CSS property name
    /// * `value` - The property value, which can be any type that implements `Display`
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mew_css::style;
    ///
    /// let css = style()
    ///     .custom_property("animation-name", "fade-in")
    ///     .custom_property("animation-duration", "2s")
    ///     .apply();
    ///
    /// assert_eq!(css, "animation-name: fade-in; animation-duration: 2s;");
    /// ```
    pub fn custom_property<T: fmt::Display>(&mut self, name: &str, value: T) -> &mut Self {
        self.add_property(Property::new(name, value))
    }

    /// Defines a CSS custom property (CSS variable).
    ///
    /// This method adds a CSS variable definition to the style. CSS variables are defined
    /// with the `--` prefix and can be referenced using the `var()` function.
    ///
    /// # Arguments
    ///
    /// * `name` - The variable name (with or without the `--` prefix)
    /// * `value` - The variable value, which can be any type that implements `Display`
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mew_css::style;
    ///
    /// let css = style()
    ///     .set_var("primary-color", "#3366ff")
    ///     .set_var("spacing", "1rem")
    ///     .apply();
    ///
    /// assert_eq!(css, "--primary-color: #3366ff; --spacing: 1rem;");
    /// ```
    ///
    /// The `--` prefix is added automatically if not present:
    ///
    /// ```rust
    /// use mew_css::style;
    ///
    /// let css = style()
    ///     .set_var("--primary-color", "#3366ff") // With prefix
    ///     .set_var("spacing", "1rem")            // Without prefix
    ///     .apply();
    ///
    /// assert_eq!(css, "--primary-color: #3366ff; --spacing: 1rem;");
    /// ```
    pub fn set_var<T: fmt::Display>(&mut self, name: &str, value: T) -> &mut Self {
        let var_name = if name.trim().starts_with("--") {
            name.trim().to_string()
        } else {
            format!("--{}", name.trim())
        };
        self.custom_property(&var_name, value)
    }

    /// Sets the align-content property for flex and grid containers.
    ///
    /// The `align-content` property aligns a flex container's lines within the flex container
    /// when there is extra space in the cross-axis.
    ///
    /// # Arguments
    ///
    /// * `value` - The align-content value to set
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mew_css::style;
    /// use mew_css::values::AlignContent;
    ///
    /// let css = style().align_content(AlignContent::Center).apply();
    /// assert_eq!(css, "align-content: center;");
    ///
    /// let css = style().align_content(AlignContent::SpaceBetween).apply();
    /// assert_eq!(css, "align-content: space-between;");
    /// ```
    pub fn align_content(&mut self, value: AlignContent) -> &mut Self {
        self.add_property(align_content::align_content(value))
    }

    /// Sets the align-items property for flex and grid containers.
    ///
    /// The `align-items` property sets the align-self value on all direct children as a group.
    /// In flexbox, it controls the alignment of items on the cross axis.
    /// In grid layout, it controls the alignment of items on the block axis within their grid areas.
    ///
    /// # Arguments
    ///
    /// * `value` - The align-items value to set
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mew_css::style;
    /// use mew_css::values::AlignItems;
    ///
    /// let css = style().align_items(AlignItems::Center).apply();
    /// assert_eq!(css, "align-items: center;");
    ///
    /// let css = style().align_items(AlignItems::Stretch).apply();
    /// assert_eq!(css, "align-items: stretch;");
    /// ```
    pub fn align_items(&mut self, value: AlignItems) -> &mut Self {
        self.add_property(align_items::align_items(value))
    }

    /// Sets the background-color property of an element.
    ///
    /// The `background-color` property sets the background color of an element.
    ///
    /// # Arguments
    ///
    /// * `value` - The color value to set
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mew_css::style;
    /// use mew_css::values::Color;
    ///
    /// // Named color
    /// let css = style().background_color(Color::Red).apply();
    /// assert_eq!(css, "background-color: red;");
    ///
    /// // RGB color
    /// let css = style().background_color(Color::Rgb(255, 0, 0)).apply();
    /// assert_eq!(css, "background-color: rgb(255, 0, 0);");
    ///
    /// // RGBA color (with transparency)
    /// let css = style().background_color(Color::Rgba(255, 0, 0, 0.5)).apply();
    /// assert_eq!(css, "background-color: rgba(255, 0, 0, 0.5);");
    /// ```
    pub fn background_color(&mut self, value: Color) -> &mut Self {
        self.add_property(background_color::background_color(value))
    }

    /// Sets the border property of an element with only style.
    ///
    /// The `border` property is a shorthand property that sets the border-width, border-style, and border-color.
    /// This method sets only the border style.
    ///
    /// # Arguments
    ///
    /// * `style` - The border style to use
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mew_css::style;
    /// use mew_css::values::BorderStyle;
    ///
    /// let css = style().border(BorderStyle::Solid).apply();
    /// assert_eq!(css, "border: solid;");
    ///
    /// let css = style().border(BorderStyle::Dashed).apply();
    /// assert_eq!(css, "border: dashed;");
    /// ```
    pub fn border(&mut self, style: BorderStyle) -> &mut Self {
        self.add_property(border::border(style))
    }

    /// Sets the border property of an element with width and style.
    ///
    /// The `border` property is a shorthand property that sets the border-width, border-style, and border-color.
    /// This method sets the border width and style.
    ///
    /// # Arguments
    ///
    /// * `width` - The border width to use
    /// * `style` - The border style to use
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mew_css::style;
    /// use mew_css::values::{BorderStyle, Size};
    ///
    /// let css = style().border_with_width(Size::Px(2), BorderStyle::Dotted).apply();
    /// assert_eq!(css, "border: 2px dotted;");
    ///
    /// let css = style().border_with_width(Size::Rem(1.5), BorderStyle::Solid).apply();
    /// assert_eq!(css, "border: 1.5rem solid;");
    /// ```
    pub fn border_with_width(&mut self, width: Size, style: BorderStyle) -> &mut Self {
        self.add_property(border::border_with_width(width, style))
    }

    /// Sets the border property of an element with style and color.
    ///
    /// The `border` property is a shorthand property that sets the border-width, border-style, and border-color.
    /// This method sets the border style and color.
    ///
    /// # Arguments
    ///
    /// * `style` - The border style to use
    /// * `color` - The border color to use
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mew_css::style;
    /// use mew_css::values::{BorderStyle, Color};
    ///
    /// let css = style().border_with_color(BorderStyle::Outset, Color::Hex("#f33".to_string())).apply();
    /// assert_eq!(css, "border: outset #f33;");
    ///
    /// let css = style().border_with_color(BorderStyle::Solid, Color::Red).apply();
    /// assert_eq!(css, "border: solid red;");
    /// ```
    pub fn border_with_color(&mut self, style: BorderStyle, color: Color) -> &mut Self {
        self.add_property(border::border_with_color(style, color))
    }

    /// Sets the border property of an element with width, style, and color.
    ///
    /// The `border` property is a shorthand property that sets the border-width, border-style, and border-color.
    /// This method sets all three properties.
    ///
    /// # Arguments
    ///
    /// * `width` - The border width to use
    /// * `style` - The border style to use
    /// * `color` - The border color to use
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mew_css::style;
    /// use mew_css::values::{BorderStyle, Color, Size};
    ///
    /// let css = style().border_with_width_and_color(Size::Px(3), BorderStyle::Dashed, Color::Green).apply();
    /// assert_eq!(css, "border: 3px dashed green;");
    ///
    /// let css = style().border_with_width_and_color(Size::Px(4), BorderStyle::Double, Color::Rgb(50, 161, 206)).apply();
    /// assert_eq!(css, "border: 4px double rgb(50, 161, 206);");
    /// ```
    pub fn border_with_width_and_color(&mut self, width: Size, style: BorderStyle, color: Color) -> &mut Self {
        self.add_property(border::border_with_width_and_color(width, style, color))
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, property) in self.properties.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", property)?;
        }
        Ok(())
    }
}

/// Creates a new style builder for constructing CSS styles.
///
/// This function is the main entry point for the Mew CSS library. It returns a new
/// `Style` instance that you can use to build CSS styles with a fluent API.
///
/// # Returns
///
/// A new, empty `Style` instance
///
/// # Examples
///
/// ```rust
/// use mew_css::style;
/// use mew_css::values::AlignContent;
///
/// let css = style()
///     .align_content(AlignContent::Center)
///     .apply();
///
/// assert_eq!(css, "align-content: center;");
/// ```
pub fn style() -> Style {
    Style::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_builder() {
        let css = style()
            .align_content(AlignContent::Center)
            .apply();

        assert_eq!(
            css,
            "align-content: center;"
        );
    }

    #[test]
    fn test_empty_style() {
        let css = style().apply();
        assert_eq!(css, "");
    }

    #[test]
    fn test_custom_property() {
        let css = style()
            .custom_property("animation-name", "fade-in")
            .custom_property("animation-duration", "2s")
            .apply();

        assert_eq!(
            css,
            "animation-name: fade-in; animation-duration: 2s;"
        );
    }

    #[test]
    fn test_align_items() {
        let css = style()
            .align_items(AlignItems::Center)
            .apply();

        assert_eq!(
            css,
            "align-items: center;"
        );

        let css = style()
            .align_items(AlignItems::Stretch)
            .apply();

        assert_eq!(
            css,
            "align-items: stretch;"
        );

        let css = style()
            .align_items(AlignItems::FlexStart)
            .apply();

        assert_eq!(
            css,
            "align-items: flex-start;"
        );
    }

    #[test]
    fn test_background_color() {
        // Named color
        let css = style()
            .background_color(Color::Red)
            .apply();

        assert_eq!(
            css,
            "background-color: red;"
        );

        // RGB color
        let css = style()
            .background_color(Color::Rgb(255, 0, 0))
            .apply();

        assert_eq!(
            css,
            "background-color: rgb(255, 0, 0);"
        );

        // RGBA color
        let css = style()
            .background_color(Color::Rgba(255, 0, 0, 0.5))
            .apply();

        assert_eq!(
            css,
            "background-color: rgba(255, 0, 0, 0.5);"
        );

        // Hex color
        let css = style()
            .background_color(Color::Hex("#ff0000".to_string()))
            .apply();

        assert_eq!(
            css,
            "background-color: #ff0000;"
        );
    }

    #[test]
    fn test_border() {
        // Style only
        let css = style()
            .border(BorderStyle::Solid)
            .apply();

        assert_eq!(
            css,
            "border: solid;"
        );

        // Width and style
        let css = style()
            .border_with_width(Size::Px(2), BorderStyle::Dotted)
            .apply();

        assert_eq!(
            css,
            "border: 2px dotted;"
        );

        // Style and color
        let css = style()
            .border_with_color(BorderStyle::Outset, Color::Hex("#f33".to_string()))
            .apply();

        assert_eq!(
            css,
            "border: outset #f33;"
        );

        // Width, style, and color
        let css = style()
            .border_with_width_and_color(Size::Px(3), BorderStyle::Dashed, Color::Green)
            .apply();

        assert_eq!(
            css,
            "border: 3px dashed green;"
        );
    }
}
