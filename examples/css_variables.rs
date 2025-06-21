use mew_css::style;
use mew_css::values::{Color, Size, Display, FontWeight};
use mew_css::variable::var;

fn main() {
    // Create CSS variables
    let primary_color = var("primary-color");
    let spacing = var("spacing");
    let display_mode = var("display-mode");
    let font_weight = var("font-weight");

    // Use CSS variables in style properties
    // Method 1: Using explicit enum variants
    let css1 = style()
        .color(Color::Var(primary_color.clone()))
        .background_color(Color::Rgba(240, 240, 240, 0.5))
        .margin(Size::Var(spacing.clone()))
        .padding(Size::Px(10))
        .display(Display::Var(display_mode.clone()))
        .font_weight(FontWeight::Var(font_weight.clone()))
        .apply();

    // Method 2: Using Into trait
    let css2 = style()
        .color(primary_color.clone().into())
        .background_color(Color::Rgba(240, 240, 240, 0.5))
        .margin(spacing.clone().into())
        .padding(Size::Px(10))
        .display(display_mode.into())
        .font_weight(font_weight.into())
        .apply();

    println!("Generated CSS with variables (Method 1): {}", css1);
    println!("Generated CSS with variables (Method 2): {}", css2);
}
