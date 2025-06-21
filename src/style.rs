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
//! use mew_css::values::{Color, Size};
//!
//! let css = style()
//!     .color(Color::Blue)
//!     .font_size(Size::Px(16))
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

use crate::properties::{Property, color, size, display, font, border, position, layout, flex, grid, transition, size_ext};
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
/// use mew_css::values::{Color, Size};
///
/// let css = style()
///     .color(Color::Blue)
///     .font_size(Size::Px(16))
///     .apply();
/// ```
///
/// The resulting CSS string will be: `color: blue; font-size: 16px;`
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
    /// use mew_css::values::{Color, Size};
    ///
    /// let css = style()
    ///     .color(Color::Blue)
    ///     .font_size(Size::Px(16))
    ///     .apply();
    ///
    /// assert_eq!(css, "color: blue; font-size: 16px;");
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
    /// use mew_css::values::{Color, Size};
    ///
    /// let css = style()
    ///     .color(Color::Blue)
    ///     .font_size(Size::Px(16))
    ///     .build();
    ///
    /// assert_eq!(css, "color: blue; font-size: 16px;");
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

    // Color properties

    /// Sets the text color of an element.
    ///
    /// The `color` property specifies the color of text content and text decorations.
    /// It can be set using named colors, RGB/RGBA values, HSL/HSLA values, or hex codes.
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
    /// // Using a named color
    /// let css1 = style().color(Color::Red).apply();
    ///
    /// // Using RGB values
    /// let css2 = style().color(Color::Rgb(255, 0, 0)).apply();
    ///
    /// // Using RGBA values with transparency
    /// let css3 = style().color(Color::Rgba(255, 0, 0, 0.5)).apply();
    ///
    /// // Using a hex color
    /// let css4 = style().color(Color::Hex("#ff0000".to_string())).apply();
    /// ```
    pub fn color(&mut self, value: Color) -> &mut Self {
        self.add_property(color::color(value))
    }

    /// Sets the background color of an element.
    ///
    /// The `background-color` property sets the background color of an element.
    /// The background covers the element's content, padding, and border areas.
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
    /// // Using a named color
    /// let css1 = style().background_color(Color::LightGray).apply();
    ///
    /// // Using RGB values
    /// let css2 = style().background_color(Color::Rgb(240, 240, 240)).apply();
    ///
    /// // Using RGBA values with transparency
    /// let css3 = style().background_color(Color::Rgba(240, 240, 240, 0.5)).apply();
    /// ```
    pub fn background_color(&mut self, value: Color) -> &mut Self {
        self.add_property(color::background_color(value))
    }

    /// Set the border-color property
    pub fn border_color(&mut self, value: Color) -> &mut Self {
        self.add_property(color::border_color(value))
    }

    // Size properties

    /// Sets the width of an element.
    ///
    /// The `width` property sets the width of an element's content area.
    /// It can be specified in various units like pixels, percentages, or relative units.
    ///
    /// # Arguments
    ///
    /// * `value` - The width value to set
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mew_css::style;
    /// use mew_css::values::Size;
    ///
    /// // Fixed width in pixels
    /// let css1 = style().width(Size::Px(300)).apply();
    ///
    /// // Percentage width (relative to parent)
    /// let css2 = style().width(Size::Percent(100.0)).apply();
    ///
    /// // Auto width
    /// let css3 = style().width(Size::Auto).apply();
    ///
    /// // Viewport-relative width
    /// let css4 = style().width(Size::Vw(50.0)).apply();
    /// ```
    pub fn width(&mut self, value: Size) -> &mut Self {
        self.add_property(size::width(value))
    }

    /// Sets the height of an element.
    ///
    /// The `height` property sets the height of an element's content area.
    /// Like width, it can be specified in various units.
    ///
    /// # Arguments
    ///
    /// * `value` - The height value to set
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mew_css::style;
    /// use mew_css::values::Size;
    ///
    /// // Fixed height in pixels
    /// let css1 = style().height(Size::Px(200)).apply();
    ///
    /// // Percentage height (requires parent to have a defined height)
    /// let css2 = style().height(Size::Percent(100.0)).apply();
    ///
    /// // Viewport-relative height
    /// let css3 = style().height(Size::Vh(100.0)).apply();
    /// ```
    pub fn height(&mut self, value: Size) -> &mut Self {
        self.add_property(size::height(value))
    }

    /// Sets the margin around an element.
    ///
    /// The `margin` property creates space around an element, outside of any defined
    /// borders. It sets the margin on all four sides at once (top, right, bottom, left).
    ///
    /// # Arguments
    ///
    /// * `value` - The margin value to set
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mew_css::style;
    /// use mew_css::values::Size;
    ///
    /// // Fixed margin in pixels
    /// let css1 = style().margin(Size::Px(10)).apply();
    ///
    /// // Auto margin (useful for horizontal centering)
    /// let css2 = style().margin(Size::Auto).apply();
    ///
    /// // Relative margin using em
    /// let css3 = style().margin(Size::Em(1.5)).apply();
    /// ```
    ///
    /// For directional margins, use the specific methods:
    /// `margin_top()`, `margin_right()`, `margin_bottom()`, `margin_left()`
    pub fn margin(&mut self, value: Size) -> &mut Self {
        self.add_property(size::margin(value))
    }

    /// Set the margin-top property
    pub fn margin_top(&mut self, value: Size) -> &mut Self {
        self.add_property(size::margin_top(value))
    }

    /// Set the margin-right property
    pub fn margin_right(&mut self, value: Size) -> &mut Self {
        self.add_property(size::margin_right(value))
    }

    /// Set the margin-bottom property
    pub fn margin_bottom(&mut self, value: Size) -> &mut Self {
        self.add_property(size::margin_bottom(value))
    }

    /// Set the margin-left property
    pub fn margin_left(&mut self, value: Size) -> &mut Self {
        self.add_property(size::margin_left(value))
    }

    /// Set the padding property
    pub fn padding(&mut self, value: Size) -> &mut Self {
        self.add_property(size::padding(value))
    }

    /// Set the padding-top property
    pub fn padding_top(&mut self, value: Size) -> &mut Self {
        self.add_property(size::padding_top(value))
    }

    /// Set the padding-right property
    pub fn padding_right(&mut self, value: Size) -> &mut Self {
        self.add_property(size::padding_right(value))
    }

    /// Set the padding-bottom property
    pub fn padding_bottom(&mut self, value: Size) -> &mut Self {
        self.add_property(size::padding_bottom(value))
    }

    /// Set the padding-left property
    pub fn padding_left(&mut self, value: Size) -> &mut Self {
        self.add_property(size::padding_left(value))
    }

    /// Set the font-size property
    pub fn font_size(&mut self, value: Size) -> &mut Self {
        self.add_property(size::font_size(value))
    }

    /// Set the line-height property
    pub fn line_height(&mut self, value: Size) -> &mut Self {
        self.add_property(size::line_height(value))
    }

    /// Set the border-width property
    pub fn border_width(&mut self, value: Size) -> &mut Self {
        self.add_property(size::border_width(value))
    }

    // Display properties

    /// Sets how an element is displayed in the layout.
    ///
    /// The `display` property determines how an element is treated in the layout flow
    /// and how its children are laid out. This is one of the most important CSS properties
    /// for controlling layout.
    ///
    /// # Arguments
    ///
    /// * `value` - The display value to set
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mew_css::style;
    /// use mew_css::values::Display;
    ///
    /// // Block layout (element takes up full width)
    /// let css1 = style().display(Display::Block).apply();
    ///
    /// // Inline layout (element flows with text)
    /// let css2 = style().display(Display::Inline).apply();
    ///
    /// // Flexbox layout
    /// let css3 = style().display(Display::Flex).apply();
    ///
    /// // Grid layout
    /// let css4 = style().display(Display::Grid).apply();
    ///
    /// // Hide element
    /// let css5 = style().display(Display::None).apply();
    /// ```
    pub fn display(&mut self, value: Display) -> &mut Self {
        self.add_property(display::display(value))
    }

    /// Sets the positioning method for an element.
    ///
    /// The `position` property specifies how an element is positioned in the document.
    /// It works together with the `top`, `right`, `bottom`, and `left` properties to
    /// determine the final position of the element.
    ///
    /// # Arguments
    ///
    /// * `value` - The position value to set
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mew_css::style;
    /// use mew_css::values::{Position, Size};
    ///
    /// // Static positioning (default flow)
    /// let css1 = style().position(Position::Static).apply();
    ///
    /// // Relative positioning (offset from normal position)
    /// let css2 = style()
    ///     .position(Position::Relative)
    ///     .top(Size::Px(10))
    ///     .left(Size::Px(20))
    ///     .apply();
    ///
    /// // Absolute positioning (relative to nearest positioned ancestor)
    /// let css3 = style()
    ///     .position(Position::Absolute)
    ///     .top(Size::Px(0))
    ///     .right(Size::Px(0))
    ///     .apply();
    ///
    /// // Fixed positioning (relative to viewport)
    /// let css4 = style()
    ///     .position(Position::Fixed)
    ///     .bottom(Size::Px(20))
    ///     .right(Size::Px(20))
    ///     .apply();
    /// ```
    pub fn position(&mut self, value: Position) -> &mut Self {
        self.add_property(display::position(value))
    }

    /// Sets the direction of flex items within a flex container.
    ///
    /// The `flex-direction` property establishes the main axis of a flex container,
    /// defining the direction in which flex items are placed. This property only
    /// applies to elements with `display: flex`.
    ///
    /// # Arguments
    ///
    /// * `value` - The flex-direction value to set
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mew_css::style;
    /// use mew_css::values::{Display, FlexDirection};
    ///
    /// // Create a horizontal flex container (default)
    /// let css1 = style()
    ///     .display(Display::Flex)
    ///     .flex_direction(FlexDirection::Row)
    ///     .apply();
    ///
    /// // Create a vertical flex container
    /// let css2 = style()
    ///     .display(Display::Flex)
    ///     .flex_direction(FlexDirection::Column)
    ///     .apply();
    ///
    /// // Reverse the order of items
    /// let css3 = style()
    ///     .display(Display::Flex)
    ///     .flex_direction(FlexDirection::RowReverse)
    ///     .apply();
    /// ```
    pub fn flex_direction(&mut self, value: FlexDirection) -> &mut Self {
        self.add_property(display::flex_direction(value))
    }

    /// Set the justify-content property
    pub fn justify_content(&mut self, value: JustifyContent) -> &mut Self {
        self.add_property(display::justify_content(value))
    }

    /// Set the align-items property
    pub fn align_items(&mut self, value: AlignItems) -> &mut Self {
        self.add_property(display::align_items(value))
    }

    // Font properties

    /// Set the font-weight property
    pub fn font_weight(&mut self, value: FontWeight) -> &mut Self {
        self.add_property(font::font_weight(value))
    }

    /// Set the font-family property
    pub fn font_family(&mut self, value: &str) -> &mut Self {
        self.add_property(font::font_family(value))
    }

    /// Set the text-align property
    pub fn text_align(&mut self, value: TextAlign) -> &mut Self {
        self.add_property(font::text_align(value))
    }

    /// Set the font-size property with FontSize enum
    pub fn font_size_enum(&mut self, value: FontSize) -> &mut Self {
        self.add_property(font::font_size_enum(value))
    }

    /// Set the line-height property with LineHeight enum
    pub fn line_height_enum(&mut self, value: LineHeight) -> &mut Self {
        self.add_property(font::line_height_enum(value))
    }

    /// Set the text-decoration property
    pub fn text_decoration(&mut self, value: TextDecoration) -> &mut Self {
        self.add_property(font::text_decoration(value))
    }

    // Border properties

    /// Set the border-style property
    pub fn border_style(&mut self, value: BorderStyle) -> &mut Self {
        self.add_property(border::border_style(value))
    }

    /// Set the border-radius property
    pub fn border_radius(&mut self, value: Size) -> &mut Self {
        self.add_property(border::border_radius(value))
    }

    /// Set the border property (shorthand)
    pub fn border(&mut self, width: Size, style: BorderStyle, color: Color) -> &mut Self {
        self.add_property(border::border(width, style, color))
    }

    /// Set the border-top property (shorthand)
    pub fn border_top(&mut self, width: Size, style: BorderStyle, color: Color) -> &mut Self {
        self.add_property(border::border_top(width, style, color))
    }

    /// Set the border-right property (shorthand)
    pub fn border_right(&mut self, width: Size, style: BorderStyle, color: Color) -> &mut Self {
        self.add_property(border::border_right(width, style, color))
    }

    /// Set the border-bottom property (shorthand)
    pub fn border_bottom(&mut self, width: Size, style: BorderStyle, color: Color) -> &mut Self {
        self.add_property(border::border_bottom(width, style, color))
    }

    /// Set the border-left property (shorthand)
    pub fn border_left(&mut self, width: Size, style: BorderStyle, color: Color) -> &mut Self {
        self.add_property(border::border_left(width, style, color))
    }

    /// Set the box-shadow property
    pub fn box_shadow(&mut self, value: BoxShadow) -> &mut Self {
        self.add_property(border::box_shadow(value))
    }

    /// Set the box-shadow property to none
    pub fn box_shadow_none(&mut self) -> &mut Self {
        self.add_property(border::box_shadow_none())
    }

    // Position properties

    /// Set the top property
    pub fn top(&mut self, value: Size) -> &mut Self {
        self.add_property(position::top(value))
    }

    /// Set the right property
    pub fn right(&mut self, value: Size) -> &mut Self {
        self.add_property(position::right(value))
    }

    /// Set the bottom property
    pub fn bottom(&mut self, value: Size) -> &mut Self {
        self.add_property(position::bottom(value))
    }

    /// Set the left property
    pub fn left(&mut self, value: Size) -> &mut Self {
        self.add_property(position::left(value))
    }

    /// Set the z-index property
    pub fn z_index(&mut self, value: ZIndex) -> &mut Self {
        self.add_property(position::z_index(value))
    }

    // Layout properties

    /// Set the overflow property
    pub fn overflow(&mut self, value: Overflow) -> &mut Self {
        self.add_property(layout::overflow(value))
    }

    /// Set the overflow-x property
    pub fn overflow_x(&mut self, value: Overflow) -> &mut Self {
        self.add_property(layout::overflow_x(value))
    }

    /// Set the overflow-y property
    pub fn overflow_y(&mut self, value: Overflow) -> &mut Self {
        self.add_property(layout::overflow_y(value))
    }

    /// Set the visibility property
    pub fn visibility(&mut self, value: Visibility) -> &mut Self {
        self.add_property(layout::visibility(value))
    }

    /// Set the opacity property
    pub fn opacity(&mut self, value: f32) -> &mut Self {
        self.add_property(layout::opacity(value))
    }

    /// Set the cursor property
    pub fn cursor(&mut self, value: Cursor) -> &mut Self {
        self.add_property(layout::cursor(value))
    }

    // Flex properties

    /// Set the gap property
    pub fn gap(&mut self, value: Size) -> &mut Self {
        self.add_property(flex::gap(value))
    }

    /// Set the row-gap property
    pub fn row_gap(&mut self, value: Size) -> &mut Self {
        self.add_property(flex::row_gap(value))
    }

    /// Set the column-gap property
    pub fn column_gap(&mut self, value: Size) -> &mut Self {
        self.add_property(flex::column_gap(value))
    }

    // Grid properties

    /// Set the grid-template-columns property
    pub fn grid_template_columns(&mut self, value: &str) -> &mut Self {
        self.add_property(grid::grid_template_columns(value))
    }

    /// Set the grid-template-rows property
    pub fn grid_template_rows(&mut self, value: &str) -> &mut Self {
        self.add_property(grid::grid_template_rows(value))
    }

    // Transition properties

    /// Set the transition property
    pub fn transition(&mut self, value: Transition) -> &mut Self {
        self.add_property(transition::transition(value))
    }

    /// Set the transition property to none
    pub fn transition_none(&mut self) -> &mut Self {
        self.add_property(transition::transition_none())
    }

    /// Set the transition property to all
    pub fn transition_all(&mut self, duration: f32, timing_function: Option<&str>, delay: Option<f32>) -> &mut Self {
        self.add_property(transition::transition_all(duration, timing_function, delay))
    }

    // Size properties extension

    /// Set the max-width property
    pub fn max_width(&mut self, value: Size) -> &mut Self {
        self.add_property(size_ext::max_width(value))
    }

    /// Set the min-width property
    pub fn min_width(&mut self, value: Size) -> &mut Self {
        self.add_property(size_ext::min_width(value))
    }

    /// Set the max-height property
    pub fn max_height(&mut self, value: Size) -> &mut Self {
        self.add_property(size_ext::max_height(value))
    }

    /// Set the min-height property
    pub fn min_height(&mut self, value: Size) -> &mut Self {
        self.add_property(size_ext::min_height(value))
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
/// use mew_css::values::{Color, Size, Display};
///
/// let css = style()
///     .color(Color::Blue)
///     .background_color(Color::Rgb(240, 240, 240))
///     .font_size(Size::Px(18))
///     .display(Display::Block)
///     .apply();
///
/// assert_eq!(css, "color: blue; background-color: rgb(240, 240, 240); font-size: 18px; display: block;");
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
            .color(Color::White)
            .background_color(Color::Rgba(255, 0, 0, 0.5))
            .font_size(Size::Px(16))
            .display(Display::Flex)
            .apply();

        assert_eq!(
            css,
            "color: white; background-color: rgba(255, 0, 0, 0.5); font-size: 16px; display: flex;"
        );
    }

    #[test]
    fn test_empty_style() {
        let css = style().apply();
        assert_eq!(css, "");
    }

    #[test]
    fn test_multiple_properties() {
        let css = style()
            .width(Size::Percent(100.0))
            .height(Size::Px(200))
            .margin(Size::Auto)
            .padding(Size::Px(10))
            .apply();

        assert_eq!(
            css,
            "width: 100%; height: 200px; margin: auto; padding: 10px;"
        );
    }
}
