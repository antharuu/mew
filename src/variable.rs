//! # CSS Variables Module
//!
//! This module provides support for CSS custom properties (CSS variables) in the Mew CSS library.
//! CSS variables allow you to define reusable values that can be referenced throughout your styles.
//!
//! ## Usage
//!
//! There are two main ways to use CSS variables:
//!
//! 1. **Defining variables** - Use the `set_var` method on `Style` to define a variable
//! 2. **Using variables** - Use the `var` function to create a reference to a variable
//!
//! ## Example
//!
//! ```rust
//! use mew_css::{style, var};
//! use mew_css::values::Color;
//!
//! // Define CSS variables
//! let root_style = style()
//!     .set_var("primary-color", "#3366ff")
//!     .set_var("spacing", "1rem")
//!     .apply();
//!
//! // Use CSS variables
//! let button_style = style()
//!     .color(var("primary-color").into())
//!     .padding(var("spacing").into())
//!     .apply();
//! ```

use std::fmt;

/// Represents a reference to a CSS custom property (CSS variable).
///
/// The `CssVar` struct allows you to reference CSS variables in your styles.
/// When displayed, it formats the variable name as `var(--name)`, which is
/// the correct syntax for using CSS variables in property values.
///
/// CSS variables provide a way to store values that can be reused throughout
/// a document. They help maintain consistency and make it easier to update
/// styles globally.
///
/// # Examples
///
/// ```rust
/// use mew_css::variable::{CssVar, var};
/// use mew_css::style;
/// use mew_css::values::Color;
///
/// // Create a CSS variable reference
/// let primary_color = CssVar::new("primary-color");
///
/// // Use it in a style
/// let css = style()
///     .color(Color::Var(primary_color))
///     .apply();
///
/// assert_eq!(css, "color: var(--primary-color);");
///
/// // The shorter way using the `var` function and `into()`
/// let css = style()
///     .color(var("primary-color").into())
///     .apply();
///
/// assert_eq!(css, "color: var(--primary-color);");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CssVar(String);

impl CssVar {
    /// Creates a new CSS variable reference.
    ///
    /// This method creates a reference to a CSS variable that can be used in
    /// property values. The variable name can be provided with or without the
    /// `--` prefix; if it's missing, it will be added automatically.
    ///
    /// # Arguments
    ///
    /// * `name` - The variable name (with or without the `--` prefix)
    ///
    /// # Returns
    ///
    /// A new `CssVar` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mew_css::variable::CssVar;
    ///
    /// // These are equivalent
    /// let var1 = CssVar::new("primary-color");
    /// let var2 = CssVar::new("--primary-color");
    /// ```
    pub fn new(name: &str) -> Self {
        let trimmed = name.trim();
        let name = if trimmed.starts_with("--") {
            trimmed.to_string()
        } else {
            format!("--{}", trimmed)
        };
        Self(name)
    }
}

impl fmt::Display for CssVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "var({})", self.0)
    }
}

/// Creates a new CSS variable reference.
///
/// This is a convenience function that creates a new `CssVar` instance.
/// It's a shorter alternative to calling `CssVar::new()`.
///
/// # Arguments
///
/// * `name` - The variable name (with or without the `--` prefix)
///
/// # Returns
///
/// A new `CssVar` instance
///
/// # Examples
///
/// ```rust
/// use mew_css::variable::var;
/// use mew_css::style;
/// use mew_css::values::Color;
///
/// // Create a CSS variable reference
/// let primary_color = var("primary-color");
///
/// // Use it in a style
/// let css = style()
///     .color(primary_color.into())
///     .apply();
///
/// assert_eq!(css, "color: var(--primary-color);");
/// ```
pub fn var(name: &str) -> CssVar {
    CssVar::new(name)
}
