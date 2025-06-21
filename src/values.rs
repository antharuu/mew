//! # CSS Value Types
//!
//! This module provides strongly-typed representations of CSS values used throughout
//! the Mew CSS library. Each CSS value type is represented as an enum or struct that
//! implements the `Display` trait for conversion to CSS string representation.
//!
//! ## Design Philosophy
//!
//! The value types in this module are designed to:
//!
//! 1. Provide type safety by restricting values to valid CSS options
//! 2. Enable IDE autocompletion for available values
//! 3. Handle the correct string formatting for CSS output
//! 4. Support CSS variables through the `Var` variant in each enum
//!
//! ## Common Value Types
//!
//! - `Color`: CSS color values (named colors, RGB, RGBA, HSL, HSLA, hex)
//! - `Size`: CSS size values (px, %, em, rem, vw, vh, auto)
//! - `Display`: CSS display property values
//! - `Position`: CSS position property values
//! - `FontWeight`: CSS font-weight property values
//! - `BorderStyle`: CSS border-style property values
//!
//! ## Usage Example
//!
//! ```rust
//! use mew_css::style;
//! use mew_css::values::{Color, Size, Display, AlignContent};
//!
//! let css = style()
//!     .align_content(AlignContent::Center)
//!     .apply();
//! ```

use std::fmt;

/// Represents CSS color values with various formats and named colors.
///
/// The `Color` enum provides a type-safe way to specify colors in CSS. It supports:
/// - Standard named colors (e.g., `Blue`, `Red`, `White`)
/// - RGB and RGBA values
/// - Hexadecimal color codes
/// - HSL and HSLA values
/// - Special values like `Transparent` and `CurrentColor`
/// - CSS variables through the `Var` variant
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    /// Alice Blue (#F0F8FF)
    AliceBlue,
    /// Antique White (#FAEBD7)
    AntiqueWhite,
    /// Aqua (#00FFFF)
    Aqua,
    Aquamarine,
    Azure,
    Beige,
    Bisque,
    Black,
    BlanchedAlmond,
    Blue,
    BlueViolet,
    Brown,
    BurlyWood,
    CadetBlue,
    Chartreuse,
    Chocolate,
    Coral,
    CornflowerBlue,
    Cornsilk,
    Crimson,
    Cyan,
    DarkBlue,
    DarkCyan,
    DarkGoldenRod,
    DarkGray,
    DarkGrey,
    DarkGreen,
    DarkKhaki,
    DarkMagenta,
    DarkOliveGreen,
    DarkOrange,
    DarkOrchid,
    DarkRed,
    DarkSalmon,
    DarkSeaGreen,
    DarkSlateBlue,
    DarkSlateGray,
    DarkSlateGrey,
    DarkTurquoise,
    DarkViolet,
    DeepPink,
    DeepSkyBlue,
    DimGray,
    DimGrey,
    DodgerBlue,
    FireBrick,
    FloralWhite,
    ForestGreen,
    Fuchsia,
    Gainsboro,
    GhostWhite,
    Gold,
    GoldenRod,
    Gray,
    Grey,
    Green,
    GreenYellow,
    HoneyDew,
    HotPink,
    IndianRed,
    Indigo,
    Ivory,
    Khaki,
    Lavender,
    LavenderBlush,
    LawnGreen,
    LemonChiffon,
    LightBlue,
    LightCoral,
    LightCyan,
    LightGoldenRodYellow,
    LightGray,
    LightGrey,
    LightGreen,
    LightPink,
    LightSalmon,
    LightSeaGreen,
    LightSkyBlue,
    LightSlateGray,
    LightSlateGrey,
    LightSteelBlue,
    LightYellow,
    Lime,
    LimeGreen,
    Linen,
    Magenta,
    Maroon,
    MediumAquaMarine,
    MediumBlue,
    MediumOrchid,
    MediumPurple,
    MediumSeaGreen,
    MediumSlateBlue,
    MediumSpringGreen,
    MediumTurquoise,
    MediumVioletRed,
    MidnightBlue,
    MintCream,
    MistyRose,
    Moccasin,
    NavajoWhite,
    Navy,
    OldLace,
    Olive,
    OliveDrab,
    Orange,
    OrangeRed,
    Orchid,
    PaleGoldenRod,
    PaleGreen,
    PaleTurquoise,
    PaleVioletRed,
    PapayaWhip,
    PeachPuff,
    Peru,
    Pink,
    Plum,
    PowderBlue,
    Purple,
    RebeccaPurple,
    Red,
    RosyBrown,
    RoyalBlue,
    SaddleBrown,
    Salmon,
    SandyBrown,
    SeaGreen,
    SeaShell,
    Sienna,
    Silver,
    SkyBlue,
    SlateBlue,
    SlateGray,
    SlateGrey,
    Snow,
    SpringGreen,
    SteelBlue,
    Tan,
    Teal,
    Thistle,
    Tomato,
    Turquoise,
    Violet,
    Wheat,
    White,
    WhiteSmoke,
    Yellow,
    YellowGreen,

    /// Special color values
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

    /// CSS variable
    Var(crate::variable::CssVar),
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::AliceBlue => write!(f, "aliceblue"),
            Color::AntiqueWhite => write!(f, "antiquewhite"),
            Color::Aqua => write!(f, "aqua"),
            Color::Aquamarine => write!(f, "aquamarine"),
            Color::Azure => write!(f, "azure"),
            Color::Beige => write!(f, "beige"),
            Color::Bisque => write!(f, "bisque"),
            Color::Black => write!(f, "black"),
            Color::BlanchedAlmond => write!(f, "blanchedalmond"),
            Color::Blue => write!(f, "blue"),
            Color::BlueViolet => write!(f, "blueviolet"),
            Color::Brown => write!(f, "brown"),
            Color::BurlyWood => write!(f, "burlywood"),
            Color::CadetBlue => write!(f, "cadetblue"),
            Color::Chartreuse => write!(f, "chartreuse"),
            Color::Chocolate => write!(f, "chocolate"),
            Color::Coral => write!(f, "coral"),
            Color::CornflowerBlue => write!(f, "cornflowerblue"),
            Color::Cornsilk => write!(f, "cornsilk"),
            Color::Crimson => write!(f, "crimson"),
            Color::Cyan => write!(f, "cyan"),
            Color::DarkBlue => write!(f, "darkblue"),
            Color::DarkCyan => write!(f, "darkcyan"),
            Color::DarkGoldenRod => write!(f, "darkgoldenrod"),
            Color::DarkGray => write!(f, "darkgray"),
            Color::DarkGrey => write!(f, "darkgrey"),
            Color::DarkGreen => write!(f, "darkgreen"),
            Color::DarkKhaki => write!(f, "darkkhaki"),
            Color::DarkMagenta => write!(f, "darkmagenta"),
            Color::DarkOliveGreen => write!(f, "darkolivegreen"),
            Color::DarkOrange => write!(f, "darkorange"),
            Color::DarkOrchid => write!(f, "darkorchid"),
            Color::DarkRed => write!(f, "darkred"),
            Color::DarkSalmon => write!(f, "darksalmon"),
            Color::DarkSeaGreen => write!(f, "darkseagreen"),
            Color::DarkSlateBlue => write!(f, "darkslateblue"),
            Color::DarkSlateGray => write!(f, "darkslategray"),
            Color::DarkSlateGrey => write!(f, "darkslategrey"),
            Color::DarkTurquoise => write!(f, "darkturquoise"),
            Color::DarkViolet => write!(f, "darkviolet"),
            Color::DeepPink => write!(f, "deeppink"),
            Color::DeepSkyBlue => write!(f, "deepskyblue"),
            Color::DimGray => write!(f, "dimgray"),
            Color::DimGrey => write!(f, "dimgrey"),
            Color::DodgerBlue => write!(f, "dodgerblue"),
            Color::FireBrick => write!(f, "firebrick"),
            Color::FloralWhite => write!(f, "floralwhite"),
            Color::ForestGreen => write!(f, "forestgreen"),
            Color::Fuchsia => write!(f, "fuchsia"),
            Color::Gainsboro => write!(f, "gainsboro"),
            Color::GhostWhite => write!(f, "ghostwhite"),
            Color::Gold => write!(f, "gold"),
            Color::GoldenRod => write!(f, "goldenrod"),
            Color::Gray => write!(f, "gray"),
            Color::Grey => write!(f, "grey"),
            Color::Green => write!(f, "green"),
            Color::GreenYellow => write!(f, "greenyellow"),
            Color::HoneyDew => write!(f, "honeydew"),
            Color::HotPink => write!(f, "hotpink"),
            Color::IndianRed => write!(f, "indianred"),
            Color::Indigo => write!(f, "indigo"),
            Color::Ivory => write!(f, "ivory"),
            Color::Khaki => write!(f, "khaki"),
            Color::Lavender => write!(f, "lavender"),
            Color::LavenderBlush => write!(f, "lavenderblush"),
            Color::LawnGreen => write!(f, "lawngreen"),
            Color::LemonChiffon => write!(f, "lemonchiffon"),
            Color::LightBlue => write!(f, "lightblue"),
            Color::LightCoral => write!(f, "lightcoral"),
            Color::LightCyan => write!(f, "lightcyan"),
            Color::LightGoldenRodYellow => write!(f, "lightgoldenrodyellow"),
            Color::LightGray => write!(f, "lightgray"),
            Color::LightGrey => write!(f, "lightgrey"),
            Color::LightGreen => write!(f, "lightgreen"),
            Color::LightPink => write!(f, "lightpink"),
            Color::LightSalmon => write!(f, "lightsalmon"),
            Color::LightSeaGreen => write!(f, "lightseagreen"),
            Color::LightSkyBlue => write!(f, "lightskyblue"),
            Color::LightSlateGray => write!(f, "lightslategray"),
            Color::LightSlateGrey => write!(f, "lightslategrey"),
            Color::LightSteelBlue => write!(f, "lightsteelblue"),
            Color::LightYellow => write!(f, "lightyellow"),
            Color::Lime => write!(f, "lime"),
            Color::LimeGreen => write!(f, "limegreen"),
            Color::Linen => write!(f, "linen"),
            Color::Magenta => write!(f, "magenta"),
            Color::Maroon => write!(f, "maroon"),
            Color::MediumAquaMarine => write!(f, "mediumaquamarine"),
            Color::MediumBlue => write!(f, "mediumblue"),
            Color::MediumOrchid => write!(f, "mediumorchid"),
            Color::MediumPurple => write!(f, "mediumpurple"),
            Color::MediumSeaGreen => write!(f, "mediumseagreen"),
            Color::MediumSlateBlue => write!(f, "mediumslateblue"),
            Color::MediumSpringGreen => write!(f, "mediumspringgreen"),
            Color::MediumTurquoise => write!(f, "mediumturquoise"),
            Color::MediumVioletRed => write!(f, "mediumvioletred"),
            Color::MidnightBlue => write!(f, "midnightblue"),
            Color::MintCream => write!(f, "mintcream"),
            Color::MistyRose => write!(f, "mistyrose"),
            Color::Moccasin => write!(f, "moccasin"),
            Color::NavajoWhite => write!(f, "navajowhite"),
            Color::Navy => write!(f, "navy"),
            Color::OldLace => write!(f, "oldlace"),
            Color::Olive => write!(f, "olive"),
            Color::OliveDrab => write!(f, "olivedrab"),
            Color::Orange => write!(f, "orange"),
            Color::OrangeRed => write!(f, "orangered"),
            Color::Orchid => write!(f, "orchid"),
            Color::PaleGoldenRod => write!(f, "palegoldenrod"),
            Color::PaleGreen => write!(f, "palegreen"),
            Color::PaleTurquoise => write!(f, "paleturquoise"),
            Color::PaleVioletRed => write!(f, "palevioletred"),
            Color::PapayaWhip => write!(f, "papayawhip"),
            Color::PeachPuff => write!(f, "peachpuff"),
            Color::Peru => write!(f, "peru"),
            Color::Pink => write!(f, "pink"),
            Color::Plum => write!(f, "plum"),
            Color::PowderBlue => write!(f, "powderblue"),
            Color::Purple => write!(f, "purple"),
            Color::RebeccaPurple => write!(f, "rebeccapurple"),
            Color::Red => write!(f, "red"),
            Color::RosyBrown => write!(f, "rosybrown"),
            Color::RoyalBlue => write!(f, "royalblue"),
            Color::SaddleBrown => write!(f, "saddlebrown"),
            Color::Salmon => write!(f, "salmon"),
            Color::SandyBrown => write!(f, "sandybrown"),
            Color::SeaGreen => write!(f, "seagreen"),
            Color::SeaShell => write!(f, "seashell"),
            Color::Sienna => write!(f, "sienna"),
            Color::Silver => write!(f, "silver"),
            Color::SkyBlue => write!(f, "skyblue"),
            Color::SlateBlue => write!(f, "slateblue"),
            Color::SlateGray => write!(f, "slategray"),
            Color::SlateGrey => write!(f, "slategrey"),
            Color::Snow => write!(f, "snow"),
            Color::SpringGreen => write!(f, "springgreen"),
            Color::SteelBlue => write!(f, "steelblue"),
            Color::Tan => write!(f, "tan"),
            Color::Teal => write!(f, "teal"),
            Color::Thistle => write!(f, "thistle"),
            Color::Tomato => write!(f, "tomato"),
            Color::Turquoise => write!(f, "turquoise"),
            Color::Violet => write!(f, "violet"),
            Color::Wheat => write!(f, "wheat"),
            Color::White => write!(f, "white"),
            Color::WhiteSmoke => write!(f, "whitesmoke"),
            Color::Yellow => write!(f, "yellow"),
            Color::YellowGreen => write!(f, "yellowgreen"),
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
            Color::Var(var) => write!(f, "{}", var),
        }
    }
}

/// Represents size and length values for CSS properties.
///
/// The `Size` enum provides a type-safe way to specify sizes in CSS. It supports
/// various unit types including absolute units (like pixels), relative units
/// (like percentages, em, rem), viewport-relative units, and special values like
/// `Auto` and `Zero`.
///
/// This enum is used for properties like width, height, margin, padding, font-size,
/// and any other CSS property that accepts a size or length value.
#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    /// Zero value (equivalent to 0px)
    Zero,

    /// Pixel values - absolute length in pixels
    Px(u32),
    /// Percentage values - relative to parent element
    Percent(f32),
    /// Em values - relative to the element's font size
    Em(f32),
    /// Rem values - relative to the root element's font size
    Rem(f32),
    /// Viewport width percentage - relative to viewport width
    Vw(f32),
    /// Viewport height percentage - relative to viewport height
    Vh(f32),
    /// Auto value - browser determines the appropriate size
    Auto,
    /// CSS variable reference
    Var(crate::variable::CssVar),
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Size::Zero => write!(f, "0"),
            Size::Px(val) => write!(f, "{}px", val),
            Size::Percent(val) => write!(f, "{}%", val),
            Size::Em(val) => write!(f, "{}em", val),
            Size::Rem(val) => write!(f, "{}rem", val),
            Size::Vw(val) => write!(f, "{}vw", val),
            Size::Vh(val) => write!(f, "{}vh", val),
            Size::Auto => write!(f, "auto"),
            Size::Var(var) => write!(f, "{}", var),
        }
    }
}

/// Represents CSS display property values that control how elements are rendered.
///
/// The `display` property is one of the most important CSS properties for controlling layout.
/// It determines how an element is treated in the layout flow and how its children are laid out.
#[derive(Debug, Clone, PartialEq)]
pub enum Display {
    /// Removes the element from the document flow (element is not displayed)
    None,
    /// Element generates a block-level box (takes up full width available)
    Block,
    /// Element generates an inline box (flows with text)
    Inline,
    /// Element generates a block-level box that flows with text
    InlineBlock,
    /// Element becomes a flex container (enables flexbox layout)
    Flex,
    /// Element becomes a grid container (enables grid layout)
    Grid,
    /// Element is displayed as a table
    Table,
    /// CSS variable reference
    Var(crate::variable::CssVar),
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
            Display::Var(var) => write!(f, "{}", var),
        }
    }
}

/// Represents CSS position property values that control element positioning.
///
/// The `position` property specifies how an element is positioned in the document.
/// It works together with the `top`, `right`, `bottom`, and `left` properties to
/// determine the final position of the element.
#[derive(Debug, Clone, PartialEq)]
pub enum Position {
    /// Default positioning in the normal document flow
    Static,
    /// Positioned relative to its normal position
    Relative,
    /// Positioned relative to the nearest positioned ancestor
    Absolute,
    /// Positioned relative to the viewport
    Fixed,
    /// Positioned based on scroll position (hybrid of relative and fixed)
    Sticky,
    /// CSS variable reference
    Var(crate::variable::CssVar),
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Position::Static => write!(f, "static"),
            Position::Relative => write!(f, "relative"),
            Position::Absolute => write!(f, "absolute"),
            Position::Fixed => write!(f, "fixed"),
            Position::Sticky => write!(f, "sticky"),
            Position::Var(var) => write!(f, "{}", var),
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
    /// CSS variable
    Var(crate::variable::CssVar),
}

impl fmt::Display for FlexDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlexDirection::Row => write!(f, "row"),
            FlexDirection::RowReverse => write!(f, "row-reverse"),
            FlexDirection::Column => write!(f, "column"),
            FlexDirection::ColumnReverse => write!(f, "column-reverse"),
            FlexDirection::Var(var) => write!(f, "{}", var),
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
    /// CSS variable
    Var(crate::variable::CssVar),
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
            JustifyContent::Var(var) => write!(f, "{}", var),
        }
    }
}

/// Align items values for flex and grid containers
///
/// The CSS align-items property sets the align-self value on all direct children as a group.
/// In flexbox, it controls the alignment of items on the cross axis.
/// In grid layout, it controls the alignment of items on the block axis within their grid areas.
#[derive(Debug, Clone, PartialEq)]
pub enum AlignItems {
    /// The effect of this keyword is dependent of the layout mode
    Normal,
    /// If the cross-size of an item is larger than the flex container, it will overflow equally in both directions
    Center,
    /// The items are packed flush to each other toward the start edge of the alignment container
    Start,
    /// The items are packed flush to each other toward the end edge of the alignment container
    End,
    /// Used in flex layout, aligns the flex items flush against the flex container's cross-start side
    FlexStart,
    /// Used in flex layout, aligns the flex items flush against the flex container's cross-end side
    FlexEnd,
    /// The items are packed flush to the edge of the alignment container's start side of the item
    SelfStart,
    /// The items are packed flush to the edge of the alignment container's end side of the item
    SelfEnd,
    /// All flex items are aligned such that their baselines align
    Baseline,
    /// Same as baseline
    FirstBaseline,
    /// Aligns the items to their last baseline
    LastBaseline,
    /// Auto-sized items will be equally enlarged to fill the container
    Stretch,
    /// Aligns the items to the center of the associated anchor element in the block direction
    AnchorCenter,
    /// If the chosen keyword means that the item overflows the alignment container causing data loss,
    /// the item is instead aligned as if the alignment mode were start
    SafeCenter,
    /// Regardless of the relative sizes of the item and alignment container and whether overflow
    /// which causes data loss might happen, the given alignment value is honored
    UnsafeCenter,
    /// Inherits the value from its parent element
    Inherit,
    /// Sets the property to its default value
    Initial,
    /// Reverts the property to the value established by the user-agent stylesheet
    Revert,
    /// Reverts to the value established for the element in a previous cascade layer
    RevertLayer,
    /// Resets the property to its inherited value if it inherits, or to its initial value if not
    Unset,
    /// CSS variable
    Var(crate::variable::CssVar),
}

impl fmt::Display for AlignItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlignItems::Normal => write!(f, "normal"),
            AlignItems::Center => write!(f, "center"),
            AlignItems::Start => write!(f, "start"),
            AlignItems::End => write!(f, "end"),
            AlignItems::FlexStart => write!(f, "flex-start"),
            AlignItems::FlexEnd => write!(f, "flex-end"),
            AlignItems::SelfStart => write!(f, "self-start"),
            AlignItems::SelfEnd => write!(f, "self-end"),
            AlignItems::Baseline => write!(f, "baseline"),
            AlignItems::FirstBaseline => write!(f, "first baseline"),
            AlignItems::LastBaseline => write!(f, "last baseline"),
            AlignItems::Stretch => write!(f, "stretch"),
            AlignItems::AnchorCenter => write!(f, "anchor-center"),
            AlignItems::SafeCenter => write!(f, "safe center"),
            AlignItems::UnsafeCenter => write!(f, "unsafe center"),
            AlignItems::Inherit => write!(f, "inherit"),
            AlignItems::Initial => write!(f, "initial"),
            AlignItems::Revert => write!(f, "revert"),
            AlignItems::RevertLayer => write!(f, "revert-layer"),
            AlignItems::Unset => write!(f, "unset"),
            AlignItems::Var(var) => write!(f, "{}", var),
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
    /// CSS variable
    Var(crate::variable::CssVar),
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
            },
            FontWeight::Var(var) => write!(f, "{}", var),
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
    /// CSS variable
    Var(crate::variable::CssVar),
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
            TextAlign::Var(var) => write!(f, "{}", var),
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
    /// CSS variable
    Var(crate::variable::CssVar),
}

impl fmt::Display for TextDecoration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextDecoration::None => write!(f, "none"),
            TextDecoration::Underline => write!(f, "underline"),
            TextDecoration::Overline => write!(f, "overline"),
            TextDecoration::LineThrough => write!(f, "line-through"),
            TextDecoration::Var(var) => write!(f, "{}", var),
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
    /// CSS variable
    Var(crate::variable::CssVar),
}

impl fmt::Display for Overflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Overflow::Visible => write!(f, "visible"),
            Overflow::Hidden => write!(f, "hidden"),
            Overflow::Scroll => write!(f, "scroll"),
            Overflow::Auto => write!(f, "auto"),
            Overflow::Clip => write!(f, "clip"),
            Overflow::Var(var) => write!(f, "{}", var),
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
    /// CSS variable
    Var(crate::variable::CssVar),
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
            Cursor::Var(var) => write!(f, "{}", var),
        }
    }
}

/// Visibility values
#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
    /// CSS variable
    Var(crate::variable::CssVar),
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Visibility::Visible => write!(f, "visible"),
            Visibility::Hidden => write!(f, "hidden"),
            Visibility::Collapse => write!(f, "collapse"),
            Visibility::Var(var) => write!(f, "{}", var),
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
    /// CSS variable
    Var(crate::variable::CssVar),
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
            BorderStyle::Var(var) => write!(f, "{}", var),
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
    /// CSS variable
    Var(crate::variable::CssVar),
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
            FontSize::Var(var) => write!(f, "{}", var),
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
    /// CSS variable
    Var(crate::variable::CssVar),
}

impl fmt::Display for LineHeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LineHeight::Normal => write!(f, "normal"),
            LineHeight::Number(val) => write!(f, "{}", val),
            LineHeight::Length(size) => write!(f, "{}", size),
            LineHeight::Percent(val) => write!(f, "{}%", val),
            LineHeight::Calc(expr) => write!(f, "calc({})", expr),
            LineHeight::Var(var) => write!(f, "{}", var),
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
    /// CSS variable
    Var(crate::variable::CssVar),
}

impl fmt::Display for ZIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZIndex::Auto => write!(f, "auto"),
            ZIndex::Index(val) => write!(f, "{}", val),
            ZIndex::Var(var) => write!(f, "{}", var),
        }
    }
}

// Implement From<CssVar> for Color to allow automatic conversion
impl From<crate::variable::CssVar> for Color {
    fn from(var: crate::variable::CssVar) -> Self {
        Color::Var(var)
    }
}

// Implement From<CssVar> for Size to allow automatic conversion
impl From<crate::variable::CssVar> for Size {
    fn from(var: crate::variable::CssVar) -> Self {
        Size::Var(var)
    }
}

// Implement From<CssVar> for Display to allow automatic conversion
impl From<crate::variable::CssVar> for Display {
    fn from(var: crate::variable::CssVar) -> Self {
        Display::Var(var)
    }
}

// Implement From<CssVar> for Position to allow automatic conversion
impl From<crate::variable::CssVar> for Position {
    fn from(var: crate::variable::CssVar) -> Self {
        Position::Var(var)
    }
}

// Implement From<CssVar> for FlexDirection to allow automatic conversion
impl From<crate::variable::CssVar> for FlexDirection {
    fn from(var: crate::variable::CssVar) -> Self {
        FlexDirection::Var(var)
    }
}

// Implement From<CssVar> for JustifyContent to allow automatic conversion
impl From<crate::variable::CssVar> for JustifyContent {
    fn from(var: crate::variable::CssVar) -> Self {
        JustifyContent::Var(var)
    }
}

// Implement From<CssVar> for AlignItems to allow automatic conversion
impl From<crate::variable::CssVar> for AlignItems {
    fn from(var: crate::variable::CssVar) -> Self {
        AlignItems::Var(var)
    }
}

// Implement From<CssVar> for FontWeight to allow automatic conversion
impl From<crate::variable::CssVar> for FontWeight {
    fn from(var: crate::variable::CssVar) -> Self {
        FontWeight::Var(var)
    }
}

// Implement From<CssVar> for TextAlign to allow automatic conversion
impl From<crate::variable::CssVar> for TextAlign {
    fn from(var: crate::variable::CssVar) -> Self {
        TextAlign::Var(var)
    }
}

// Implement From<CssVar> for TextDecoration to allow automatic conversion
impl From<crate::variable::CssVar> for TextDecoration {
    fn from(var: crate::variable::CssVar) -> Self {
        TextDecoration::Var(var)
    }
}

// Implement From<CssVar> for Overflow to allow automatic conversion
impl From<crate::variable::CssVar> for Overflow {
    fn from(var: crate::variable::CssVar) -> Self {
        Overflow::Var(var)
    }
}

// Implement From<CssVar> for Cursor to allow automatic conversion
impl From<crate::variable::CssVar> for Cursor {
    fn from(var: crate::variable::CssVar) -> Self {
        Cursor::Var(var)
    }
}

// Implement From<CssVar> for Visibility to allow automatic conversion
impl From<crate::variable::CssVar> for Visibility {
    fn from(var: crate::variable::CssVar) -> Self {
        Visibility::Var(var)
    }
}

// Implement From<CssVar> for BorderStyle to allow automatic conversion
impl From<crate::variable::CssVar> for BorderStyle {
    fn from(var: crate::variable::CssVar) -> Self {
        BorderStyle::Var(var)
    }
}

// Implement From<CssVar> for ZIndex to allow automatic conversion
impl From<crate::variable::CssVar> for ZIndex {
    fn from(var: crate::variable::CssVar) -> Self {
        ZIndex::Var(var)
    }
}

// Implement From<CssVar> for FontSize to allow automatic conversion
impl From<crate::variable::CssVar> for FontSize {
    fn from(var: crate::variable::CssVar) -> Self {
        FontSize::Var(var)
    }
}

/// Align content values for flex and grid containers
///
/// The CSS align-content property sets the distribution of space between and around content items
/// along a flexbox's cross axis, or a grid or block-level element's block axis.
#[derive(Debug, Clone, PartialEq)]
pub enum AlignContent {
    /// The items are packed in their default position as if no align-content value was set.
    Normal,
    /// The items are packed flush to each other against the start edge of the alignment container in the cross axis.
    Start,
    /// The items are packed flush to each other in the center of the alignment container along the cross axis.
    Center,
    /// The items are packed flush to each other against the end edge of the alignment container in the cross axis.
    End,
    /// The items are packed flush to each other against the edge of the alignment container depending on the flex container's cross-start side.
    /// This only applies to flex layout items. For items that are not children of a flex container, this value is treated like start.
    FlexStart,
    /// The items are packed flush to each other against the edge of the alignment container depending on the flex container's cross-end side.
    /// This only applies to flex layout items. For items that are not children of a flex container, this value is treated like end.
    FlexEnd,
    /// Specifies participation in first-baseline alignment: aligns the alignment baseline of the box's first baseline set with the corresponding baseline in the shared first baseline set of all the boxes in its baseline-sharing group.
    Baseline,
    /// Specifies participation in first-baseline alignment: aligns the alignment baseline of the box's first baseline set with the corresponding baseline in the shared first baseline set of all the boxes in its baseline-sharing group.
    FirstBaseline,
    /// Specifies participation in last-baseline alignment: aligns the alignment baseline of the box's last baseline set with the corresponding baseline in the shared last baseline set of all the boxes in its baseline-sharing group.
    LastBaseline,
    /// The items are evenly distributed within the alignment container along the cross axis. The spacing between each pair of adjacent items is the same. The first item is flush with the start edge of the alignment container in the cross axis, and the last item is flush with the end edge of the alignment container in the cross axis.
    SpaceBetween,
    /// The items are evenly distributed within the alignment container along the cross axis. The spacing between each pair of adjacent items is the same. The empty space before the first and after the last item equals half of the space between each pair of adjacent items.
    SpaceAround,
    /// The items are evenly distributed within the alignment container along the cross axis. The spacing between each pair of adjacent items, the start edge and the first item, and the end edge and the last item, are all exactly the same.
    SpaceEvenly,
    /// If the combined size of the items along the cross axis is less than the size of the alignment container, any auto-sized items have their size increased equally (not proportionally), while still respecting the constraints imposed by max-height/max-width (or equivalent functionality), so that the combined size exactly fills the alignment container along the cross axis.
    Stretch,
    /// Used alongside an alignment keyword. If the chosen keyword means that the item overflows the alignment container causing data loss, the item is instead aligned as if the alignment mode were start.
    SafeCenter,
    /// Used alongside an alignment keyword. Regardless of the relative sizes of the item and alignment container and whether overflow which causes data loss might happen, the given alignment value is honored.
    UnsafeCenter,
    /// Inherits the value from its parent element.
    Inherit,
    /// Sets the property to its default value.
    Initial,
    /// Reverts the property to the value established by the user-agent stylesheet (or by user styles, if any exist).
    Revert,
    /// Reverts the property to the value established for the current cascade layer (if any).
    RevertLayer,
    /// Resets the property to its inherited value if it inherits, or to its initial value if not.
    Unset,
    /// CSS variable
    Var(crate::variable::CssVar),
}

impl fmt::Display for AlignContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlignContent::Normal => write!(f, "normal"),
            AlignContent::Start => write!(f, "start"),
            AlignContent::Center => write!(f, "center"),
            AlignContent::End => write!(f, "end"),
            AlignContent::FlexStart => write!(f, "flex-start"),
            AlignContent::FlexEnd => write!(f, "flex-end"),
            AlignContent::Baseline => write!(f, "baseline"),
            AlignContent::FirstBaseline => write!(f, "first baseline"),
            AlignContent::LastBaseline => write!(f, "last baseline"),
            AlignContent::SpaceBetween => write!(f, "space-between"),
            AlignContent::SpaceAround => write!(f, "space-around"),
            AlignContent::SpaceEvenly => write!(f, "space-evenly"),
            AlignContent::Stretch => write!(f, "stretch"),
            AlignContent::SafeCenter => write!(f, "safe center"),
            AlignContent::UnsafeCenter => write!(f, "unsafe center"),
            AlignContent::Inherit => write!(f, "inherit"),
            AlignContent::Initial => write!(f, "initial"),
            AlignContent::Revert => write!(f, "revert"),
            AlignContent::RevertLayer => write!(f, "revert-layer"),
            AlignContent::Unset => write!(f, "unset"),
            AlignContent::Var(var) => write!(f, "{}", var),
        }
    }
}

// Implement From<CssVar> for AlignContent to allow automatic conversion
impl From<crate::variable::CssVar> for AlignContent {
    fn from(var: crate::variable::CssVar) -> Self {
        AlignContent::Var(var)
    }
}

// Implement From<CssVar> for LineHeight to allow automatic conversion
impl From<crate::variable::CssVar> for LineHeight {
    fn from(var: crate::variable::CssVar) -> Self {
        LineHeight::Var(var)
    }
}
