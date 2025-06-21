# Mew CSS Style Builder - Development Guidelines

This document provides guidelines and information for developers working on the Mew CSS Style Builder library.

## Build/Configuration Instructions

### Prerequisites
- Rust (edition 2024)
- Cargo (Rust's package manager)

### Building the Project
1. Clone the repository
2. Navigate to the project root directory
3. Run `cargo build` to build the library
4. Run `cargo build --release` for an optimized release build

### Project Structure
- `src/lib.rs` - Main library entry point and module exports
- `src/style.rs` - Style builder implementation with fluent API
- `src/values.rs` - CSS value types (Color, Size, Display, etc.)
- `src/properties.rs` - CSS property definitions
- `src/tests.rs` - Integration tests

## Testing Information

### Running Tests
- Run all tests with `cargo test`
- Run specific tests with `cargo test <test_name>`
- Run tests with output with `cargo test -- --nocapture`

### Adding New Tests
1. For unit tests, add them to the relevant module file in a `mod tests` block
2. For integration tests, add them to `src/tests.rs`

Example of adding a unit test:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_feature() {
        // Test code here
        let result = some_function();
        assert_eq!(result, expected_value);
    }
}
```

### Example Test
The library includes a basic example in the `examples` directory:

```rust
use mew_css::style;
use mew_css::values::{Color, Size, Display};

fn main() {
    let css = style()
        .color(Color::Blue)
        .background_color(Color::Rgb(240, 240, 240))
        .font_size(Size::Px(18))
        .display(Display::Block)
        .apply();
    
    println!("Generated CSS: {}", css);
}
```

Run this example with: `cargo run --example basic_style`

## Additional Development Information

### Code Style
- Follow the Rust standard style guidelines
- Use meaningful variable and function names
- Document public API with doc comments
- Keep the fluent API design consistent

### Adding New CSS Properties
1. Define the value type in `values.rs` if needed
2. Add the property function in the appropriate module in `properties.rs`
3. Add a method to the `Style` struct in `style.rs`
4. Add tests for the new property

### Performance Considerations
- The library uses a `Vec<Property>` to store properties, which is efficient for the typical use case
- The `apply()` method generates the CSS string on demand, avoiding unnecessary allocations
- Consider using `with_capacity` when creating a new `Style` if you know the approximate number of properties

### Error Handling
- The library currently uses panic for invalid values (e.g., invalid font weights)
- Consider adding a Result-based API for error handling in future versions