# Mew CSS Style Builder - Development Guidelines

This document provides guidelines and information for developers working on the Mew CSS Style Builder library.

## Build/Configuration Instructions

### Prerequisites
- Rust (edition 2021)
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
- `src/properties/` - CSS property definitions (organized in modules)
- `src/variable.rs` - CSS variable support
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

### CSS Property Structure

Each CSS property in the Mew CSS Style Builder library follows a consistent structure with three main components:

#### 1. Value Definition (in `values.rs`)

CSS property values are defined as enums with variants for all possible values. Each variant should have a detailed documentation comment explaining what it does.

Example for `AlignContent`:

```rust
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
    // ... other variants
    /// CSS variable
    Var(crate::variable::CssVar),
}

impl fmt::Display for AlignContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlignContent::Normal => write!(f, "normal"),
            AlignContent::Start => write!(f, "start"),
            // ... other variants
        }
    }
}

// Implement From<CssVar> for the enum to allow automatic conversion
impl From<crate::variable::CssVar> for AlignContent {
    fn from(var: crate::variable::CssVar) -> Self {
        AlignContent::Var(var)
    }
}
```

#### 2. Property Function (in `properties/` modules)

Each CSS property has a dedicated module in the `properties/` directory with a function that creates a `Property` instance for the CSS property. The function should have comprehensive documentation including:

- A brief description of what the property does
- Detailed explanation of all possible values
- Examples of usage

Example for `align-content`:

```rust
//! # Align Content Property
//!
//! This module provides a function for creating the CSS `align-content` property.
//! The `align-content` property sets the distribution of space between and around content items
//! along a flexbox's cross axis, or a grid or block-level element's block axis.
//!
//! ... (syntax examples and usage)

use crate::properties::Property;
use crate::values::AlignContent;

/// Creates a CSS `align-content` property.
///
/// The `align-content` property sets the distribution of space between and around content items
/// along a flexbox's cross axis, or a grid or block-level element's block axis.
///
/// ## Values
///
/// - `normal`: The items are packed in their default position as if no align-content value was set.
/// - `start`: The items are packed flush to each other against the start edge of the alignment container in the cross axis.
/// ... (other values)
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::align_content;
/// use mew_css::values::AlignContent;
///
/// // Basic positional alignment
/// let prop = align_content::align_content(AlignContent::Start);
/// assert_eq!(prop.to_string(), "align-content: start;");
/// ```
pub fn align_content(value: AlignContent) -> Property {
    Property::new("align-content", value)
}

#[cfg(test)]
mod tests {
    // Tests for all possible values
}
```

#### 3. Style Builder Integration (in `style.rs`)

Each property is exposed in the fluent API through a method in the `Style` struct that takes the property value and adds it to the style.

Example for `align-content`:

```rust
impl Style {
    /// Sets the align-content property.
    ///
    /// The align-content property sets the distribution of space between and around content items
    /// along a flexbox's cross axis, or a grid or block-level element's block axis.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mew_css::style;
    /// use mew_css::values::AlignContent;
    ///
    /// let css = style().align_content(AlignContent::Center).apply();
    /// ```
    pub fn align_content(&mut self, value: AlignContent) -> &mut Self {
        self.add_property(align_content::align_content(value))
    }
}
```

### Adding New CSS Properties

To add a new CSS property to the library:

1. Define the value type in `values.rs` if needed:
   - Create an enum with variants for all possible values
   - Add detailed documentation for each variant
   - Implement the `Display` trait to convert the enum values to CSS string values
   - Implement `From<CssVar>` to support CSS variables

2. Create a new module in the `properties/` directory:
   - Add module-level documentation explaining the property
   - Create a function that takes the value type and returns a `Property` instance
   - Add comprehensive documentation including examples
   - Add unit tests for all possible values

3. Add a method to the `Style` struct in `style.rs`:
   - The method should take the value type and call the property function
   - Add documentation with examples
   - Return `&mut self` to support method chaining

4. Update the module exports in `properties/mod.rs` and `lib.rs` if needed

5. Add integration tests in `tests.rs` if appropriate

### Performance Considerations
- The library uses a `Vec<Property>` to store properties, which is efficient for the typical use case
- The `apply()` method generates the CSS string on demand, avoiding unnecessary allocations
- Consider using `with_capacity` when creating a new `Style` if you know the approximate number of properties

### Error Handling
- The library currently uses panic for invalid values (e.g., invalid font weights)
- Consider adding a Result-based API for error handling in future versions
