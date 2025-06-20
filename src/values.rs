//! CSS value types
//!
//! This module contains strongly-typed enums for CSS values.

use std::fmt;

/// Color values for CSS properties
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    /// Named CSS colors
    Black,
    White,
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
    Gray,
    Transparent,
    
    /// RGB color with values from 0-255
    Rgb(u8, u8, u8),
    
    /// RGBA color with values from 0-255 and alpha from 0.0-1.0
    Rgba(u8, u8, u8, f32),
    
    /// Hex color code
    Hex(String),
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Black => write!(f, "black"),
            Color::White => write!(f, "white"),
            Color::Red => write!(f, "red"),
            Color::Green => write!(f, "green"),
            Color::Blue => write!(f, "blue"),
            Color::Yellow => write!(f, "yellow"),
            Color::Purple => write!(f, "purple"),
            Color::Gray => write!(f, "gray"),
            Color::Transparent => write!(f, "transparent"),
            Color::Rgb(r, g, b) => write!(f, "rgb({}, {}, {})", r, g, b),
            Color::Rgba(r, g, b, a) => write!(f, "rgba({}, {}, {}, {})", r, g, b, a),
            Color::Hex(hex) => {
                let hex_str = if hex.starts_with('#') { hex.clone() } else { format!("#{}", hex) };
                write!(f, "{}", hex_str)
            }
        }
    }
}

/// Size values for CSS properties
#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    /// Pixel values
    Px(u32),
    /// Percentage values
    Percent(f32),
    /// Em values (relative to font size)
    Em(f32),
    /// Rem values (relative to root font size)
    Rem(f32),
    /// Viewport width percentage
    Vw(f32),
    /// Viewport height percentage
    Vh(f32),
    /// Auto value
    Auto,
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Size::Px(val) => write!(f, "{}px", val),
            Size::Percent(val) => write!(f, "{}%", val),
            Size::Em(val) => write!(f, "{}em", val),
            Size::Rem(val) => write!(f, "{}rem", val),
            Size::Vw(val) => write!(f, "{}vw", val),
            Size::Vh(val) => write!(f, "{}vh", val),
            Size::Auto => write!(f, "auto"),
        }
    }
}

/// Display property values
#[derive(Debug, Clone, PartialEq)]
pub enum Display {
    None,
    Block,
    Inline,
    InlineBlock,
    Flex,
    Grid,
    Table,
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Display::None => write!(f, "none"),
            Display::Block => write!(f, "block"),
            Display::Inline => write!(f, "inline"),
            Display::InlineBlock => write!(f, "inline-block"),
            Display::Flex => write!(f, "flex"),
            Display::Grid => write!(f, "grid"),
            Display::Table => write!(f, "table"),
        }
    }
}

/// Position property values
#[derive(Debug, Clone, PartialEq)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Position::Static => write!(f, "static"),
            Position::Relative => write!(f, "relative"),
            Position::Absolute => write!(f, "absolute"),
            Position::Fixed => write!(f, "fixed"),
            Position::Sticky => write!(f, "sticky"),
        }
    }
}

/// Flex direction values
#[derive(Debug, Clone, PartialEq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl fmt::Display for FlexDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlexDirection::Row => write!(f, "row"),
            FlexDirection::RowReverse => write!(f, "row-reverse"),
            FlexDirection::Column => write!(f, "column"),
            FlexDirection::ColumnReverse => write!(f, "column-reverse"),
        }
    }
}

/// Justify content values
#[derive(Debug, Clone, PartialEq)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl fmt::Display for JustifyContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JustifyContent::FlexStart => write!(f, "flex-start"),
            JustifyContent::FlexEnd => write!(f, "flex-end"),
            JustifyContent::Center => write!(f, "center"),
            JustifyContent::SpaceBetween => write!(f, "space-between"),
            JustifyContent::SpaceAround => write!(f, "space-around"),
            JustifyContent::SpaceEvenly => write!(f, "space-evenly"),
        }
    }
}

/// Align items values
#[derive(Debug, Clone, PartialEq)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

impl fmt::Display for AlignItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlignItems::FlexStart => write!(f, "flex-start"),
            AlignItems::FlexEnd => write!(f, "flex-end"),
            AlignItems::Center => write!(f, "center"),
            AlignItems::Baseline => write!(f, "baseline"),
            AlignItems::Stretch => write!(f, "stretch"),
        }
    }
}

/// Font weight values
#[derive(Debug, Clone, PartialEq)]
pub enum FontWeight {
    Normal,
    Bold,
    Bolder,
    Lighter,
    /// Numeric weight (100-900)
    Weight(u16),
}

impl fmt::Display for FontWeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FontWeight::Normal => write!(f, "normal"),
            FontWeight::Bold => write!(f, "bold"),
            FontWeight::Bolder => write!(f, "bolder"),
            FontWeight::Lighter => write!(f, "lighter"),
            FontWeight::Weight(w) => {
                // Validate weight is between 100-900 and a multiple of 100
                if *w >= 100 && *w <= 900 && w % 100 == 0 {
                    write!(f, "{}", w)
                } else {
                    write!(f, "400") // Default to normal if invalid
                }
            }
        }
    }
}