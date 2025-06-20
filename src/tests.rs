//! Integration tests for the CSS style builder library

#[cfg(test)]
mod tests {
    use crate::style;
    use crate::values::{Color, Display, Size, FlexDirection, JustifyContent, AlignItems};

    #[test]
    fn test_example_from_issue_description() {
        let css = style()
            .color(Color::White)
            .background_color(Color::Rgba(255, 0, 0, 0.5))
            .font_size(Size::Px(16))
            .display(Display::Flex)
            .apply();

        assert_eq!(
            css,
            "color: white; background-color: rgba(255, 0, 0, 0.5); font-size: 16px; display: flex;"
        );
    }

    #[test]
    fn test_complex_style() {
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

        // Check that all properties are included in the output
        assert!(css.contains("display: flex;"));
        assert!(css.contains("flex-direction: column;"));
        assert!(css.contains("justify-content: center;"));
        assert!(css.contains("align-items: center;"));
        assert!(css.contains("width: 100%;"));
        assert!(css.contains("height: 100vh;"));
        assert!(css.contains("background-color: rgb(240, 240, 240);"));
        assert!(css.contains("color: #333333;"));
        assert!(css.contains("font-size: 1.2rem;"));
        assert!(css.contains("font-family: Arial, sans-serif;"));
        assert!(css.contains("padding: 20px;"));
        assert!(css.contains("margin: auto;"));
        assert!(css.contains("border-radius: 8px;"));
    }

    #[test]
    fn test_build_alias() {
        let css1 = style()
            .color(Color::Blue)
            .font_size(Size::Px(14))
            .apply();

        let css2 = style()
            .color(Color::Blue)
            .font_size(Size::Px(14))
            .build();

        assert_eq!(css1, css2);
    }
}
