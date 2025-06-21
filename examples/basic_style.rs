use mew_css::style;
use mew_css::values::{Color, Size, Display};

fn main() {
    // Create a CSS style using the mew library
    let css = style()
        .color(Color::Blue)
        .background_color(Color::Rgb(240, 240, 240))
        .font_size(Size::Px(18))
        .display(Display::Block)
        .apply();

    println!("Generated CSS: {}", css);

    // Verify the output
    let expected = "color: blue; background-color: rgb(240, 240, 240); font-size: 18px; display: block;";
    assert_eq!(css, expected);

    println!("Example completed successfully!");
}
