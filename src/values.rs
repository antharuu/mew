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

    /// HSL color
    Hsl(u16, u8, u8),

    /// HSLA color
    Hsla(u16, u8, u8, f32),

    /// Current color
    CurrentColor,

    /// Inherit color
    Inherit,
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
            },
            Color::Hsl(h, s, l) => write!(f, "hsl({}, {}%, {}%)", h, s, l),
            Color::Hsla(h, s, l, a) => write!(f, "hsla({}, {}%, {}%, {})", h, s, l, a),
            Color::CurrentColor => write!(f, "currentColor"),
            Color::Inherit => write!(f, "inherit"),
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

/// Text align values
#[derive(Debug, Clone, PartialEq)]
pub enum TextAlign {
    Left,
    Right,
    Center,
    Justify,
    Start,
    End,
}

impl fmt::Display for TextAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextAlign::Left => write!(f, "left"),
            TextAlign::Right => write!(f, "right"),
            TextAlign::Center => write!(f, "center"),
            TextAlign::Justify => write!(f, "justify"),
            TextAlign::Start => write!(f, "start"),
            TextAlign::End => write!(f, "end"),
        }
    }
}

/// Text decoration values
#[derive(Debug, Clone, PartialEq)]
pub enum TextDecoration {
    None,
    Underline,
    Overline,
    LineThrough,
}

impl fmt::Display for TextDecoration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextDecoration::None => write!(f, "none"),
            TextDecoration::Underline => write!(f, "underline"),
            TextDecoration::Overline => write!(f, "overline"),
            TextDecoration::LineThrough => write!(f, "line-through"),
        }
    }
}

/// Overflow values
#[derive(Debug, Clone, PartialEq)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
    Auto,
    Clip,
}

impl fmt::Display for Overflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Overflow::Visible => write!(f, "visible"),
            Overflow::Hidden => write!(f, "hidden"),
            Overflow::Scroll => write!(f, "scroll"),
            Overflow::Auto => write!(f, "auto"),
            Overflow::Clip => write!(f, "clip"),
        }
    }
}

/// Cursor values
#[derive(Debug, Clone, PartialEq)]
pub enum Cursor {
    Default,
    Pointer,
    Text,
    NotAllowed,
    Wait,
    Move,
    Grab,
    ZoomIn,
    ZoomOut,
}

impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cursor::Default => write!(f, "default"),
            Cursor::Pointer => write!(f, "pointer"),
            Cursor::Text => write!(f, "text"),
            Cursor::NotAllowed => write!(f, "not-allowed"),
            Cursor::Wait => write!(f, "wait"),
            Cursor::Move => write!(f, "move"),
            Cursor::Grab => write!(f, "grab"),
            Cursor::ZoomIn => write!(f, "zoom-in"),
            Cursor::ZoomOut => write!(f, "zoom-out"),
        }
    }
}

/// Visibility values
#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Visibility::Visible => write!(f, "visible"),
            Visibility::Hidden => write!(f, "hidden"),
            Visibility::Collapse => write!(f, "collapse"),
        }
    }
}

/// Border style values
#[derive(Debug, Clone, PartialEq)]
pub enum BorderStyle {
    None,
    Solid,
    Dashed,
    Dotted,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

impl fmt::Display for BorderStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BorderStyle::None => write!(f, "none"),
            BorderStyle::Solid => write!(f, "solid"),
            BorderStyle::Dashed => write!(f, "dashed"),
            BorderStyle::Dotted => write!(f, "dotted"),
            BorderStyle::Double => write!(f, "double"),
            BorderStyle::Groove => write!(f, "groove"),
            BorderStyle::Ridge => write!(f, "ridge"),
            BorderStyle::Inset => write!(f, "inset"),
            BorderStyle::Outset => write!(f, "outset"),
        }
    }
}

/// Font size values
#[derive(Debug, Clone, PartialEq)]
pub enum FontSize {
    /// Pixel values
    Px(u32),
    /// Percentage values
    Percent(f32),
    /// Em values (relative to font size)
    Em(f32),
    /// Rem values (relative to root font size)
    Rem(f32),
    /// Smaller than parent
    Smaller,
    /// Larger than parent
    Larger,
    /// Absolute size keywords
    XxSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XxLarge,
    /// Calculated value
    Calc(String),
}

impl fmt::Display for FontSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FontSize::Px(val) => write!(f, "{}px", val),
            FontSize::Percent(val) => write!(f, "{}%", val),
            FontSize::Em(val) => write!(f, "{}em", val),
            FontSize::Rem(val) => write!(f, "{}rem", val),
            FontSize::Smaller => write!(f, "smaller"),
            FontSize::Larger => write!(f, "larger"),
            FontSize::XxSmall => write!(f, "xx-small"),
            FontSize::XSmall => write!(f, "x-small"),
            FontSize::Small => write!(f, "small"),
            FontSize::Medium => write!(f, "medium"),
            FontSize::Large => write!(f, "large"),
            FontSize::XLarge => write!(f, "x-large"),
            FontSize::XxLarge => write!(f, "xx-large"),
            FontSize::Calc(expr) => write!(f, "calc({})", expr),
        }
    }
}

/// Line height values
#[derive(Debug, Clone, PartialEq)]
pub enum LineHeight {
    /// Normal line height
    Normal,
    /// Number (unitless) value
    Number(f32),
    /// Length value
    Length(Size),
    /// Percentage value
    Percent(f32),
    /// Calculated value
    Calc(String),
}

impl fmt::Display for LineHeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LineHeight::Normal => write!(f, "normal"),
            LineHeight::Number(val) => write!(f, "{}", val),
            LineHeight::Length(size) => write!(f, "{}", size),
            LineHeight::Percent(val) => write!(f, "{}%", val),
            LineHeight::Calc(expr) => write!(f, "calc({})", expr),
        }
    }
}

/// Box shadow values
#[derive(Debug, Clone, PartialEq)]
pub struct BoxShadow {
    pub h_offset: Size,
    pub v_offset: Size,
    pub blur: Option<Size>,
    pub spread: Option<Size>,
    pub color: Option<Color>,
    pub inset: bool,
}

impl fmt::Display for BoxShadow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.inset {
            write!(f, "inset ")?;
        }

        write!(f, "{} {}", self.h_offset, self.v_offset)?;

        if let Some(blur) = &self.blur {
            write!(f, " {}", blur)?;
        }

        if let Some(spread) = &self.spread {
            write!(f, " {}", spread)?;
        }

        if let Some(color) = &self.color {
            write!(f, " {}", color)?;
        }

        Ok(())
    }
}

/// Transition values
#[derive(Debug, Clone, PartialEq)]
pub struct Transition {
    pub property: String,
    pub duration: f32,
    pub timing_function: Option<String>,
    pub delay: Option<f32>,
}

impl fmt::Display for Transition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}s", self.property, self.duration)?;

        if let Some(timing) = &self.timing_function {
            write!(f, " {}", timing)?;
        }

        if let Some(delay) = self.delay {
            write!(f, " {}s", delay)?;
        }

        Ok(())
    }
}

/// Z-index values
#[derive(Debug, Clone, PartialEq)]
pub enum ZIndex {
    Auto,
    Index(i32),
}

impl fmt::Display for ZIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZIndex::Auto => write!(f, "auto"),
            ZIndex::Index(val) => write!(f, "{}", val),
        }
    }
}
