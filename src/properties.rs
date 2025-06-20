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
    pub fn text_align(value: &str) -> Property {
        Property::new("text-align", value)
    }
}

/// Border properties
pub mod border {
    use super::*;

    /// Create a border-style property
    pub fn border_style(value: &str) -> Property {
        Property::new("border-style", value)
    }

    /// Create a border-radius property
    pub fn border_radius(value: Size) -> Property {
        Property::new("border-radius", value)
    }
}