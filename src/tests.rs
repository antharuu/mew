//! Integration tests for the CSS style builder library

#[cfg(test)]
mod tests {
    use crate::style;
    use crate::variable::var;
    use crate::values::{
        AlignItems, BorderStyle, BoxShadow, Color, Cursor, Display, FlexDirection, FontSize,
        FontWeight, JustifyContent, LineHeight, Overflow, Position, Size, TextAlign, TextDecoration,
        Transition, Visibility, ZIndex,
    };

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

    #[test]
    fn test_all_properties_from_issue_description() {
        let shadow = BoxShadow {
            h_offset: Size::Px(0),
            v_offset: Size::Px(2),
            blur: Some(Size::Px(4)),
            spread: None,
            color: Some(Color::Rgba(0, 0, 0, 0.1)),
            inset: false,
        };

        let transition = Transition {
            property: "all".to_string(),
            duration: 0.3,
            timing_function: Some("ease".to_string()),
            delay: None,
        };

        let css = style()
            // 1. color
            .color(Color::Red)
            .color(Color::Hex("fff".to_string()))
            .color(Color::Rgb(255, 0, 0))
            .color(Color::Hsl(0, 100, 50))
            .color(Color::CurrentColor)

            // 2. background-color
            .background_color(Color::Transparent)
            .background_color(Color::Hex("000".to_string()))
            .background_color(Color::Rgba(0, 0, 0, 0.5))
            .background_color(Color::Inherit)

            // 3. font-size
            .font_size(Size::Px(16))
            .font_size(Size::Rem(1.0))
            .font_size_enum(FontSize::Smaller)
            .font_size_enum(FontSize::Larger)
            .font_size_enum(FontSize::Calc("100% + 16px".to_string()))

            // 4. font-weight
            .font_weight(FontWeight::Normal)
            .font_weight(FontWeight::Bold)
            .font_weight(FontWeight::Lighter)
            .font_weight(FontWeight::Bolder)
            .font_weight(FontWeight::Weight(500))

            // 5. font-family
            .font_family("Arial")
            .font_family("Helvetica")
            .font_family("sans-serif")
            .font_family("Times New Roman")

            // 6. line-height
            .line_height(Size::Px(24))
            .line_height_enum(LineHeight::Normal)
            .line_height_enum(LineHeight::Percent(120.0))
            .line_height_enum(LineHeight::Calc("1em + 0.5em".to_string()))

            // 7. text-align
            .text_align(TextAlign::Left)
            .text_align(TextAlign::Right)
            .text_align(TextAlign::Center)
            .text_align(TextAlign::Justify)
            .text_align(TextAlign::Start)
            .text_align(TextAlign::End)

            // 8. display
            .display(Display::Block)
            .display(Display::Inline)
            .display(Display::InlineBlock)
            .display(Display::Flex)
            .display(Display::Grid)
            .display(Display::None)

            // 9. position
            .position(Position::Static)
            .position(Position::Relative)
            .position(Position::Absolute)
            .position(Position::Fixed)
            .position(Position::Sticky)

            // 10. top, right, bottom, left
            .top(Size::Px(0))
            .right(Size::Px(10))
            .bottom(Size::Auto)
            .left(Size::Percent(50.0))

            // 11. margin
            .margin(Size::Px(0))
            .margin(Size::Px(10))
            .margin(Size::Em(1.0))
            .margin(Size::Auto)

            // 12. padding
            .padding(Size::Px(0))
            .padding(Size::Px(8))
            .padding(Size::Rem(1.0))

            // 13. width
            .width(Size::Auto)
            .width(Size::Px(100))
            .width(Size::Percent(100.0))

            // 14. height
            .height(Size::Auto)
            .height(Size::Vh(100.0))
            .height(Size::Percent(50.0))

            // 15. max-width / min-width
            .max_width(Size::Px(600))
            .min_width(Size::Percent(100.0))
            .max_width(Size::Auto)

            // 16. border
            .border(Size::Px(1), BorderStyle::Solid, Color::Hex("000".to_string()))
            .border_top(Size::Px(2), BorderStyle::Solid, Color::Red)
            .border_right(Size::Px(3), BorderStyle::Dashed, Color::Blue)
            .border_bottom(Size::Px(4), BorderStyle::Dotted, Color::Green)
            .border_left(Size::Px(5), BorderStyle::Double, Color::Black)
            .border_style(BorderStyle::None)
            .border_style(BorderStyle::Dashed)
            .border_style(BorderStyle::Double)

            // 17. border-radius
            .border_radius(Size::Px(0))
            .border_radius(Size::Px(4))
            .border_radius(Size::Percent(50.0))
            .border_radius(Size::Em(1.0))

            // 18. box-shadow
            .box_shadow_none()
            .box_shadow(shadow)

            // 19. text-decoration
            .text_decoration(TextDecoration::None)
            .text_decoration(TextDecoration::Underline)
            .text_decoration(TextDecoration::LineThrough)
            .text_decoration(TextDecoration::Overline)

            // 20. overflow
            .overflow(Overflow::Visible)
            .overflow(Overflow::Hidden)
            .overflow(Overflow::Scroll)
            .overflow(Overflow::Auto)
            .overflow(Overflow::Clip)

            // 21. cursor
            .cursor(Cursor::Pointer)
            .cursor(Cursor::Default)
            .cursor(Cursor::Text)
            .cursor(Cursor::NotAllowed)

            // 22. z-index
            .z_index(ZIndex::Auto)
            .z_index(ZIndex::Index(0))
            .z_index(ZIndex::Index(10))
            .z_index(ZIndex::Index(9999))

            // 23. opacity
            .opacity(1.0)
            .opacity(0.5)
            .opacity(0.0)
            .opacity(0.75)

            // 24. transition
            .transition_none()
            .transition(transition)
            .transition_all(0.3, Some("ease"), None)

            // 25. flex-direction
            .flex_direction(FlexDirection::Row)
            .flex_direction(FlexDirection::RowReverse)
            .flex_direction(FlexDirection::Column)
            .flex_direction(FlexDirection::ColumnReverse)

            // 26. justify-content
            .justify_content(JustifyContent::FlexStart)
            .justify_content(JustifyContent::Center)
            .justify_content(JustifyContent::SpaceBetween)
            .justify_content(JustifyContent::SpaceAround)
            .justify_content(JustifyContent::SpaceEvenly)

            // 27. align-items
            .align_items(AlignItems::Stretch)
            .align_items(AlignItems::Center)
            .align_items(AlignItems::FlexStart)
            .align_items(AlignItems::FlexEnd)
            .align_items(AlignItems::Baseline)

            // 28. gap
            .gap(Size::Px(0))
            .gap(Size::Px(8))
            .gap(Size::Rem(1.0))

            // 29. grid-template-columns
            .grid_template_columns("repeat(3, 1fr)")
            .grid_template_columns("200px auto")
            .grid_template_columns("1fr 2fr")

            // 30. visibility
            .visibility(Visibility::Visible)
            .visibility(Visibility::Hidden)
            .visibility(Visibility::Collapse)
            .apply();

        // Check that all properties are included in the output
        // 1. color
        assert!(css.contains("color: red;"));
        assert!(css.contains("color: #fff;"));
        assert!(css.contains("color: rgb(255, 0, 0);"));
        assert!(css.contains("color: hsl(0, 100%, 50%);"));
        assert!(css.contains("color: currentColor;"));

        // 2. background-color
        assert!(css.contains("background-color: transparent;"));
        assert!(css.contains("background-color: #000;"));
        assert!(css.contains("background-color: rgba(0, 0, 0, 0.5);"));
        assert!(css.contains("background-color: inherit;"));

        // 3. font-size
        assert!(css.contains("font-size: 16px;"));
        assert!(css.contains("font-size: 1rem;"));
        assert!(css.contains("font-size: smaller;"));
        assert!(css.contains("font-size: larger;"));
        assert!(css.contains("font-size: calc(100% + 16px);"));

        // 4. font-weight
        assert!(css.contains("font-weight: normal;"));
        assert!(css.contains("font-weight: bold;"));
        assert!(css.contains("font-weight: lighter;"));
        assert!(css.contains("font-weight: bolder;"));
        assert!(css.contains("font-weight: 500;"));

        // 5. font-family
        assert!(css.contains("font-family: Arial;"));
        assert!(css.contains("font-family: Helvetica;"));
        assert!(css.contains("font-family: sans-serif;"));
        assert!(css.contains("font-family: Times New Roman;"));

        // 6. line-height
        assert!(css.contains("line-height: 24px;"));
        assert!(css.contains("line-height: normal;"));
        assert!(css.contains("line-height: 120%;"));
        assert!(css.contains("line-height: calc(1em + 0.5em);"));

        // 7. text-align
        assert!(css.contains("text-align: left;"));
        assert!(css.contains("text-align: right;"));
        assert!(css.contains("text-align: center;"));
        assert!(css.contains("text-align: justify;"));
        assert!(css.contains("text-align: start;"));
        assert!(css.contains("text-align: end;"));

        // 8. display
        assert!(css.contains("display: block;"));
        assert!(css.contains("display: inline;"));
        assert!(css.contains("display: inline-block;"));
        assert!(css.contains("display: flex;"));
        assert!(css.contains("display: grid;"));
        assert!(css.contains("display: none;"));

        // 9. position
        assert!(css.contains("position: static;"));
        assert!(css.contains("position: relative;"));
        assert!(css.contains("position: absolute;"));
        assert!(css.contains("position: fixed;"));
        assert!(css.contains("position: sticky;"));

        // 10. top, right, bottom, left
        assert!(css.contains("top: 0px;"));
        assert!(css.contains("right: 10px;"));
        assert!(css.contains("bottom: auto;"));
        assert!(css.contains("left: 50%;"));

        // 11. margin
        assert!(css.contains("margin: 0px;"));
        assert!(css.contains("margin: 10px;"));
        assert!(css.contains("margin: 1em;"));
        assert!(css.contains("margin: auto;"));

        // 12. padding
        assert!(css.contains("padding: 0px;"));
        assert!(css.contains("padding: 8px;"));
        assert!(css.contains("padding: 1rem;"));

        // 13. width
        assert!(css.contains("width: auto;"));
        assert!(css.contains("width: 100px;"));
        assert!(css.contains("width: 100%;"));

        // 14. height
        assert!(css.contains("height: auto;"));
        assert!(css.contains("height: 100vh;"));
        assert!(css.contains("height: 50%;"));

        // 15. max-width / min-width
        assert!(css.contains("max-width: 600px;"));
        assert!(css.contains("min-width: 100%;"));
        assert!(css.contains("max-width: auto;"));

        // 16. border
        assert!(css.contains("border: 1px solid #000;"));
        assert!(css.contains("border-top: 2px solid red;"));
        assert!(css.contains("border-right: 3px dashed blue;"));
        assert!(css.contains("border-bottom: 4px dotted green;"));
        assert!(css.contains("border-left: 5px double black;"));
        assert!(css.contains("border-style: none;"));
        assert!(css.contains("border-style: dashed;"));
        assert!(css.contains("border-style: double;"));

        // 17. border-radius
        assert!(css.contains("border-radius: 0px;"));
        assert!(css.contains("border-radius: 4px;"));
        assert!(css.contains("border-radius: 50%;"));
        assert!(css.contains("border-radius: 1em;"));

        // 18. box-shadow
        assert!(css.contains("box-shadow: none;"));
        assert!(css.contains("box-shadow: 0px 2px 4px rgba(0, 0, 0, 0.1);"));

        // 19. text-decoration
        assert!(css.contains("text-decoration: none;"));
        assert!(css.contains("text-decoration: underline;"));
        assert!(css.contains("text-decoration: line-through;"));
        assert!(css.contains("text-decoration: overline;"));

        // 20. overflow
        assert!(css.contains("overflow: visible;"));
        assert!(css.contains("overflow: hidden;"));
        assert!(css.contains("overflow: scroll;"));
        assert!(css.contains("overflow: auto;"));
        assert!(css.contains("overflow: clip;"));

        // 21. cursor
        assert!(css.contains("cursor: pointer;"));
        assert!(css.contains("cursor: default;"));
        assert!(css.contains("cursor: text;"));
        assert!(css.contains("cursor: not-allowed;"));

        // 22. z-index
        assert!(css.contains("z-index: auto;"));
        assert!(css.contains("z-index: 0;"));
        assert!(css.contains("z-index: 10;"));
        assert!(css.contains("z-index: 9999;"));

        // 23. opacity
        assert!(css.contains("opacity: 1;"));
        assert!(css.contains("opacity: 0.5;"));
        assert!(css.contains("opacity: 0;"));
        assert!(css.contains("opacity: 0.75;"));

        // 24. transition
        assert!(css.contains("transition: none;"));
        assert!(css.contains("transition: all 0.3s ease;"));
        assert!(css.contains("transition: all 0.3s ease;"));

        // 25. flex-direction
        assert!(css.contains("flex-direction: row;"));
        assert!(css.contains("flex-direction: row-reverse;"));
        assert!(css.contains("flex-direction: column;"));
        assert!(css.contains("flex-direction: column-reverse;"));

        // 26. justify-content
        assert!(css.contains("justify-content: flex-start;"));
        assert!(css.contains("justify-content: center;"));
        assert!(css.contains("justify-content: space-between;"));
        assert!(css.contains("justify-content: space-around;"));
        assert!(css.contains("justify-content: space-evenly;"));

        // 27. align-items
        assert!(css.contains("align-items: stretch;"));
        assert!(css.contains("align-items: center;"));
        assert!(css.contains("align-items: flex-start;"));
        assert!(css.contains("align-items: flex-end;"));
        assert!(css.contains("align-items: baseline;"));

        // 28. gap
        assert!(css.contains("gap: 0px;"));
        assert!(css.contains("gap: 8px;"));
        assert!(css.contains("gap: 1rem;"));

        // 29. grid-template-columns
        assert!(css.contains("grid-template-columns: repeat(3, 1fr);"));
        assert!(css.contains("grid-template-columns: 200px auto;"));
        assert!(css.contains("grid-template-columns: 1fr 2fr;"));

        // 30. visibility
        assert!(css.contains("visibility: visible;"));
        assert!(css.contains("visibility: hidden;"));
        assert!(css.contains("visibility: collapse;"));
    }

    #[test]
    fn test_css_variables() {
        let css = style()
            .set_var("primary", Color::Blue)
            .custom_property("color", var("primary"))
            .apply();

        assert!(css.contains("--primary: blue;"));
        assert!(css.contains("color: var(--primary);"));
    }
}
