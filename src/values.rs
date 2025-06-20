//! CSS value types
//!
//! This module contains strongly-typed enums for CSS values.

use std::fmt;

/// Color values for CSS properties
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    /// Named CSS colors
    AliceBlue,
    AntiqueWhite,
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
        }
    }
}

/// Size values for CSS properties
#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    /// Constants
    Zero,
    
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
            Size::Zero => write!(f, "0"),
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
