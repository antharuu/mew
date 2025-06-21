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
pub fn var(name: &str) -> CssVar {
    CssVar::new(name)
}
