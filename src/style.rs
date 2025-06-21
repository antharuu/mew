//! Style builder module
//!
//! This module provides a fluent API for building CSS styles.

use crate::properties::{Property, color, size, display, font, border, position, layout, flex, grid, transition, size_ext};
use crate::values::*;
use std::fmt;

/// A CSS style builder
#[derive(Debug, Default)]
pub struct Style {
    properties: Vec<Property>,
}

impl Style {
    /// Create a new empty style
    pub fn new() -> Self {
        Self {
            properties: Vec::new(),
        }
    }

    /// Add a property to the style
    pub fn add_property(&mut self, property: Property) -> &mut Self {
        self.properties.push(property);
        self
    }

    /// Generate the CSS string
    pub fn apply(&self) -> String {
        self.to_string()
    }

    /// Alias for apply
    pub fn build(&self) -> String {
        self.apply()
    }

    /// Add a custom property. This allows using raw property names and values
    /// such as CSS variables.
    pub fn custom_property<T: fmt::Display>(&mut self, name: &str, value: T) -> &mut Self {
        self.add_property(Property::new(name, value))
    }

    /// Define a CSS variable (custom property).
    pub fn set_var<T: fmt::Display>(&mut self, name: &str, value: T) -> &mut Self {
        let var_name = if name.trim().starts_with("--") {
            name.trim().to_string()
        } else {
            format!("--{}", name.trim())
        };
        self.custom_property(&var_name, value)
    }

    // Color properties

    /// Set the color property
    pub fn color(&mut self, value: Color) -> &mut Self {
        self.add_property(color::color(value))
    }

    /// Set the background-color property
    pub fn background_color(&mut self, value: Color) -> &mut Self {
        self.add_property(color::background_color(value))
    }

    /// Set the border-color property
    pub fn border_color(&mut self, value: Color) -> &mut Self {
        self.add_property(color::border_color(value))
    }

    // Size properties

    /// Set the width property
    pub fn width(&mut self, value: Size) -> &mut Self {
        self.add_property(size::width(value))
    }

    /// Set the height property
    pub fn height(&mut self, value: Size) -> &mut Self {
        self.add_property(size::height(value))
    }

    /// Set the margin property
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

    /// Set the display property
    pub fn display(&mut self, value: Display) -> &mut Self {
        self.add_property(display::display(value))
    }

    /// Set the position property
    pub fn position(&mut self, value: Position) -> &mut Self {
        self.add_property(display::position(value))
    }

    /// Set the flex-direction property
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

/// Create a new style builder
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
