use mew_css::style;
use mew_css::values::{Color, Size, BorderStyle};

fn main() {
    // Create a style with directional border properties
    let css = style()
        .border_top(Size::Px(1), BorderStyle::Solid, Color::Red)
        .border_right(Size::Px(2), BorderStyle::Dashed, Color::Blue)
        .border_bottom(Size::Px(3), BorderStyle::Dotted, Color::Green)
        .border_left(Size::Px(4), BorderStyle::Double, Color::Black)
        .apply();

    println!("Generated CSS with directional borders:");
    println!("{}", css);
}
