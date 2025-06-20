//! CSS properties module
//!
//! This module defines the CSS properties that can be set on a style.

use crate::values::*;
use std::fmt;

/// A CSS property with a name and value
#[derive(Debug, Clone)]
pub struct Property {
    name: String,
    value: String,
}

impl Property {
    /// Create a new property with the given name and value
    pub fn new<T: fmt::Display>(name: &str, value: T) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {};", self.name, self.value)
    }
}

/// Color properties
pub mod color {
    use super::*;

    /// Create a color property
    pub fn color(value: Color) -> Property {
        Property::new("color", value)
    }

    /// Create a background-color property
    pub fn background_color(value: Color) -> Property {
        Property::new("background-color", value)
    }

    /// Create a border-color property
    pub fn border_color(value: Color) -> Property {
        Property::new("border-color", value)
    }
}

/// Size properties
pub mod size {
    use super::*;

    /// Create a width property
    pub fn width(value: Size) -> Property {
        Property::new("width", value)
    }

    /// Create a height property
    pub fn height(value: Size) -> Property {
        Property::new("height", value)
    }

    /// Create a margin property
    pub fn margin(value: Size) -> Property {
        Property::new("margin", value)
    }

    /// Create a margin-top property
    pub fn margin_top(value: Size) -> Property {
        Property::new("margin-top", value)
    }

    /// Create a margin-right property
    pub fn margin_right(value: Size) -> Property {
        Property::new("margin-right", value)
    }

    /// Create a margin-bottom property
    pub fn margin_bottom(value: Size) -> Property {
        Property::new("margin-bottom", value)
    }

    /// Create a margin-left property
    pub fn margin_left(value: Size) -> Property {
        Property::new("margin-left", value)
    }

    /// Create a padding property
    pub fn padding(value: Size) -> Property {
        Property::new("padding", value)
    }

    /// Create a padding-top property
    pub fn padding_top(value: Size) -> Property {
        Property::new("padding-top", value)
    }

    /// Create a padding-right property
    pub fn padding_right(value: Size) -> Property {
        Property::new("padding-right", value)
    }

    /// Create a padding-bottom property
    pub fn padding_bottom(value: Size) -> Property {
        Property::new("padding-bottom", value)
    }

    /// Create a padding-left property
    pub fn padding_left(value: Size) -> Property {
        Property::new("padding-left", value)
    }

    /// Create a font-size property
    pub fn font_size(value: Size) -> Property {
        Property::new("font-size", value)
    }

    /// Create a line-height property
    pub fn line_height(value: Size) -> Property {
        Property::new("line-height", value)
    }

    /// Create a border-width property
    pub fn border_width(value: Size) -> Property {
        Property::new("border-width", value)
    }
}

/// Display properties
pub mod display {
    use super::*;

    /// Create a display property
    pub fn display(value: Display) -> Property {
        Property::new("display", value)
    }

    /// Create a position property
    pub fn position(value: Position) -> Property {
        Property::new("position", value)
    }

    /// Create a flex-direction property
    pub fn flex_direction(value: FlexDirection) -> Property {
        Property::new("flex-direction", value)
    }

    /// Create a justify-content property
    pub fn justify_content(value: JustifyContent) -> Property {
        Property::new("justify-content", value)
    }

    /// Create an align-items property
    pub fn align_items(value: AlignItems) -> Property {
        Property::new("align-items", value)
    }
}

/// Font properties
pub mod font {
    use super::*;

    /// Create a font-weight property
    pub fn font_weight(value: FontWeight) -> Property {
        Property::new("font-weight", value)
    }

    /// Create a font-family property
    pub fn font_family(value: &str) -> Property {
        Property::new("font-family", value)
    }

    /// Create a text-align property
    pub fn text_align(value: TextAlign) -> Property {
        Property::new("text-align", value)
    }

    /// Create a font-size property with FontSize enum
    pub fn font_size_enum(value: FontSize) -> Property {
        Property::new("font-size", value)
    }

    /// Create a line-height property with LineHeight enum
    pub fn line_height_enum(value: LineHeight) -> Property {
        Property::new("line-height", value)
    }

    /// Create a text-decoration property
    pub fn text_decoration(value: TextDecoration) -> Property {
        Property::new("text-decoration", value)
    }
}

/// Border properties
pub mod border {
    use super::*;

    /// Create a border-style property
    pub fn border_style(value: BorderStyle) -> Property {
        Property::new("border-style", value)
    }

    /// Create a border-radius property
    pub fn border_radius(value: Size) -> Property {
        Property::new("border-radius", value)
    }

    /// Create a border property (shorthand)
    pub fn border(width: Size, style: BorderStyle, color: Color) -> Property {
        Property::new("border", format!("{} {} {}", width, style, color))
    }

    /// Create a box-shadow property
    pub fn box_shadow(value: BoxShadow) -> Property {
        Property::new("box-shadow", value)
    }

    /// Create a box-shadow property with none value
    pub fn box_shadow_none() -> Property {
        Property::new("box-shadow", "none")
    }
}

/// Position properties
pub mod position {
    use super::*;

    /// Create a top property
    pub fn top(value: Size) -> Property {
        Property::new("top", value)
    }

    /// Create a right property
    pub fn right(value: Size) -> Property {
        Property::new("right", value)
    }

    /// Create a bottom property
    pub fn bottom(value: Size) -> Property {
        Property::new("bottom", value)
    }

    /// Create a left property
    pub fn left(value: Size) -> Property {
        Property::new("left", value)
    }

    /// Create a z-index property
    pub fn z_index(value: ZIndex) -> Property {
        Property::new("z-index", value)
    }
}

/// Layout properties
pub mod layout {
    use super::*;

    /// Create an overflow property
    pub fn overflow(value: Overflow) -> Property {
        Property::new("overflow", value)
    }

    /// Create an overflow-x property
    pub fn overflow_x(value: Overflow) -> Property {
        Property::new("overflow-x", value)
    }

    /// Create an overflow-y property
    pub fn overflow_y(value: Overflow) -> Property {
        Property::new("overflow-y", value)
    }

    /// Create a visibility property
    pub fn visibility(value: Visibility) -> Property {
        Property::new("visibility", value)
    }

    /// Create an opacity property
    pub fn opacity(value: f32) -> Property {
        // Ensure opacity is between 0 and 1
        let clamped = value.max(0.0).min(1.0);
        Property::new("opacity", clamped)
    }

    /// Create a cursor property
    pub fn cursor(value: Cursor) -> Property {
        Property::new("cursor", value)
    }
}

/// Flex properties
pub mod flex {
    use super::*;

    /// Create a gap property
    pub fn gap(value: Size) -> Property {
        Property::new("gap", value)
    }

    /// Create a row-gap property
    pub fn row_gap(value: Size) -> Property {
        Property::new("row-gap", value)
    }

    /// Create a column-gap property
    pub fn column_gap(value: Size) -> Property {
        Property::new("column-gap", value)
    }
}

/// Grid properties
pub mod grid {
    use super::*;

    /// Create a grid-template-columns property
    pub fn grid_template_columns(value: &str) -> Property {
        Property::new("grid-template-columns", value)
    }

    /// Create a grid-template-rows property
    pub fn grid_template_rows(value: &str) -> Property {
        Property::new("grid-template-rows", value)
    }
}

/// Transition properties
pub mod transition {
    use super::*;

    /// Create a transition property
    pub fn transition(value: Transition) -> Property {
        Property::new("transition", value)
    }

    /// Create a transition property with none value
    pub fn transition_none() -> Property {
        Property::new("transition", "none")
    }

    /// Create a transition property with all value
    pub fn transition_all(duration: f32, timing_function: Option<&str>, delay: Option<f32>) -> Property {
        let mut value = format!("all {}s", duration);

        if let Some(timing) = timing_function {
            value.push_str(&format!(" {}", timing));
        }

        if let Some(delay_val) = delay {
            value.push_str(&format!(" {}s", delay_val));
        }

        Property::new("transition", value)
    }
}

/// Size properties extension
pub mod size_ext {
    use super::*;

    /// Create a max-width property
    pub fn max_width(value: Size) -> Property {
        Property::new("max-width", value)
    }

    /// Create a min-width property
    pub fn min_width(value: Size) -> Property {
        Property::new("min-width", value)
    }

    /// Create a max-height property
    pub fn max_height(value: Size) -> Property {
        Property::new("max-height", value)
    }

    /// Create a min-height property
    pub fn min_height(value: Size) -> Property {
        Property::new("min-height", value)
    }
}
