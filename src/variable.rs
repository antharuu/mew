use std::fmt;

/// A CSS custom property reference.
///
/// This struct represents a reference to a CSS variable used in property
/// values. It simply stores the variable name and formats it as
/// `var(--name)` when displayed.
#[derive(Debug, Clone, PartialEq)]
pub struct CssVar(String);

impl CssVar {
    /// Create a new CSS variable reference.
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

/// Convenience function to create a [`CssVar`].
pub fn var(name: &str) -> CssVar {
    CssVar::new(name)
}
