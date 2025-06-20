# Mew - CSS Style Builder for Rust

A fluent, chainable API for building CSS styles with strong typing in Rust.

## Features

- **Fluent, chainable API** for building CSS styles
- **Strongly-typed CSS values** using enums and structs
- **Extensible design** following SOLID principles
- **No external dependencies**
- **Comprehensive validation** of CSS values

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mew = "0.1.0"
```

## Usage

### Basic Example

```rust
use mew::style;
use mew::values::{Color, Size, Display};

fn main() {
    let css = style()
        .color(Color::White)
        .background_color(Color::Rgba(255, 0, 0, 0.5))
        .font_size(Size::Px(16))
        .display(Display::Flex)
        .apply();

    println!("{}", css);
    // Output: color: white; background-color: rgba(255, 0, 0, 0.5); font-size: 16px; display: flex;
}
```

### Complex Example

```rust
use mew::style;
use mew::values::{Color, Size, Display, FlexDirection, JustifyContent, AlignItems};

fn main() {
    let css = style()
        .display(Display::Flex)
        .flex_direction(FlexDirection::Column)
        .justify_content(JustifyContent::Center)
        .align_items(AlignItems::Center)
        .width(Size::Percent(100.0))
        .height(Size::Vh(100.0))
        .background_color(Color::Rgb(240, 240, 240))
        .color(Color::Hex("333333".to_string()))
        .font_size(Size::Rem(1.2))
        .font_family("Arial, sans-serif")
        .padding(Size::Px(20))
        .margin(Size::Auto)
        .border_radius(Size::Px(8))
        .apply();

    println!("{}", css);
}
```

### CSS Variables

```rust
use mew::{style, var};
use mew::values::{Color};

fn main() {
    let css = style()
        .set_var("primary", Color::Red)
        .custom_property("color", var("primary"))
        .apply();

    println!("{}", css);
    // Output: --primary: red; color: var(--primary);
}
```

## Available CSS Properties

### Color Properties
- `color(Color)`
- `background_color(Color)`
- `border_color(Color)`

### Size Properties
- `width(Size)`
- `height(Size)`
- `margin(Size)`, `margin_top(Size)`, `margin_right(Size)`, `margin_bottom(Size)`, `margin_left(Size)`
- `padding(Size)`, `padding_top(Size)`, `padding_right(Size)`, `padding_bottom(Size)`, `padding_left(Size)`
- `font_size(Size)`
- `line_height(Size)`
- `border_width(Size)`
- `border_radius(Size)`

### Display Properties
- `display(Display)`
- `position(Position)`
- `flex_direction(FlexDirection)`
- `justify_content(JustifyContent)`
- `align_items(AlignItems)`

### Font Properties
- `font_weight(FontWeight)`
- `font_family(&str)`
- `text_align(&str)`

### Border Properties
- `border_style(&str)`

## CSS Value Types

### Color
- Named colors: `Black`, `White`, `Red`, `Green`, `Blue`, `Yellow`, `Purple`, `Gray`, `Transparent`
- RGB: `Rgb(u8, u8, u8)`
- RGBA: `Rgba(u8, u8, u8, f32)`
- Hex: `Hex(String)`

### Size
- Pixels: `Px(u32)`
- Percentage: `Percent(f32)`
- Em: `Em(f32)`
- Rem: `Rem(f32)`
- Viewport width: `Vw(f32)`
- Viewport height: `Vh(f32)`
- Auto: `Auto`

### Display
- `None`, `Block`, `Inline`, `InlineBlock`, `Flex`, `Grid`, `Table`

### Position
- `Static`, `Relative`, `Absolute`, `Fixed`, `Sticky`

### FlexDirection
- `Row`, `RowReverse`, `Column`, `ColumnReverse`

### JustifyContent
- `FlexStart`, `FlexEnd`, `Center`, `SpaceBetween`, `SpaceAround`, `SpaceEvenly`

### AlignItems
- `FlexStart`, `FlexEnd`, `Center`, `Baseline`, `Stretch`

### FontWeight
- `Normal`, `Bold`, `Bolder`, `Lighter`, `Weight(u16)`

## Extending the Library

The library is designed to be easily extensible. To add new CSS properties:

1. Add a new enum in `values.rs` if needed
2. Add a new property function in the appropriate module in `properties.rs`
3. Add a new method to the `Style` struct in `style.rs`

## License

MIT