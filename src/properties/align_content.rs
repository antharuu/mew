//! # Align Content Property
//!
//! This module provides a function for creating the CSS `align-content` property.
//! The `align-content` property sets the distribution of space between and around content items
//! along a flexbox's cross axis, or a grid or block-level element's block axis.
//!
//! This property has no effect on single line flex containers (i.e., ones with flex-wrap: nowrap).
//!
//! ## Syntax
//!
//! ```css
//! /* Normal alignment */
//! align-content: normal;
//!
//! /* Basic positional alignment */
//! align-content: start;
//! align-content: center;
//! align-content: end;
//! align-content: flex-start;
//! align-content: flex-end;
//!
//! /* Baseline alignment */
//! align-content: baseline;
//! align-content: first baseline;
//! align-content: last baseline;
//!
//! /* Distributed alignment */
//! align-content: space-between;
//! align-content: space-around;
//! align-content: space-evenly;
//! align-content: stretch;
//!
//! /* Overflow alignment */
//! align-content: safe center;
//! align-content: unsafe center;
//!
//! /* Global values */
//! align-content: inherit;
//! align-content: initial;
//! align-content: revert;
//! align-content: revert-layer;
//! align-content: unset;
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use mew_css::properties::align_content;
//! use mew_css::values::AlignContent;
//!
//! let prop = align_content::align_content(AlignContent::Center);
//! assert_eq!(prop.to_string(), "align-content: center;");
//!
//! let prop = align_content::align_content(AlignContent::SpaceBetween);
//! assert_eq!(prop.to_string(), "align-content: space-between;");
//! ```

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
/// - `center`: The items are packed flush to each other in the center of the alignment container along the cross axis.
/// - `end`: The items are packed flush to each other against the end edge of the alignment container in the cross axis.
/// - `flex-start`: The items are packed flush to each other against the edge of the alignment container depending on the flex container's cross-start side.
/// - `flex-end`: The items are packed flush to each other against the edge of the alignment container depending on the flex container's cross-end side.
/// - `baseline`, `first-baseline`, `last-baseline`: Specifies participation in first- or last-baseline alignment.
/// - `space-between`: The items are evenly distributed with the first item flush with the start edge and the last item flush with the end edge.
/// - `space-around`: The items are evenly distributed with equal space around each item.
/// - `space-evenly`: The items are evenly distributed with equal space between each item.
/// - `stretch`: If the combined size of the items is less than the container, auto-sized items are stretched to fill it.
/// - `safe center`, `unsafe center`: Overflow alignment options.
/// - `inherit`, `initial`, `revert`, `revert-layer`, `unset`: Global values.
///
/// # Arguments
///
/// * `value` - The align-content value to use
///
/// # Returns
///
/// A new `Property` instance representing the align-content property
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
///
/// let prop = align_content::align_content(AlignContent::Center);
/// assert_eq!(prop.to_string(), "align-content: center;");
///
/// // Distributed alignment
/// let prop = align_content::align_content(AlignContent::SpaceBetween);
/// assert_eq!(prop.to_string(), "align-content: space-between;");
///
/// let prop = align_content::align_content(AlignContent::SpaceAround);
/// assert_eq!(prop.to_string(), "align-content: space-around;");
///
/// // Overflow alignment
/// let prop = align_content::align_content(AlignContent::SafeCenter);
/// assert_eq!(prop.to_string(), "align-content: safe center;");
/// ```
pub fn align_content(value: AlignContent) -> Property {
    Property::new("align-content", value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::AlignContent;

    #[test]
    fn test_normal_alignment() {
        let prop = align_content(AlignContent::Normal);
        assert_eq!(prop.to_string(), "align-content: normal;");
    }

    #[test]
    fn test_basic_positional_alignment() {
        let prop = align_content(AlignContent::Start);
        assert_eq!(prop.to_string(), "align-content: start;");

        let prop = align_content(AlignContent::Center);
        assert_eq!(prop.to_string(), "align-content: center;");

        let prop = align_content(AlignContent::End);
        assert_eq!(prop.to_string(), "align-content: end;");

        let prop = align_content(AlignContent::FlexStart);
        assert_eq!(prop.to_string(), "align-content: flex-start;");

        let prop = align_content(AlignContent::FlexEnd);
        assert_eq!(prop.to_string(), "align-content: flex-end;");
    }

    #[test]
    fn test_baseline_alignment() {
        let prop = align_content(AlignContent::Baseline);
        assert_eq!(prop.to_string(), "align-content: baseline;");

        let prop = align_content(AlignContent::FirstBaseline);
        assert_eq!(prop.to_string(), "align-content: first baseline;");

        let prop = align_content(AlignContent::LastBaseline);
        assert_eq!(prop.to_string(), "align-content: last baseline;");
    }

    #[test]
    fn test_distributed_alignment() {
        let prop = align_content(AlignContent::SpaceBetween);
        assert_eq!(prop.to_string(), "align-content: space-between;");

        let prop = align_content(AlignContent::SpaceAround);
        assert_eq!(prop.to_string(), "align-content: space-around;");

        let prop = align_content(AlignContent::SpaceEvenly);
        assert_eq!(prop.to_string(), "align-content: space-evenly;");

        let prop = align_content(AlignContent::Stretch);
        assert_eq!(prop.to_string(), "align-content: stretch;");
    }

    #[test]
    fn test_overflow_alignment() {
        let prop = align_content(AlignContent::SafeCenter);
        assert_eq!(prop.to_string(), "align-content: safe center;");

        let prop = align_content(AlignContent::UnsafeCenter);
        assert_eq!(prop.to_string(), "align-content: unsafe center;");
    }

    #[test]
    fn test_global_values() {
        let prop = align_content(AlignContent::Inherit);
        assert_eq!(prop.to_string(), "align-content: inherit;");

        let prop = align_content(AlignContent::Initial);
        assert_eq!(prop.to_string(), "align-content: initial;");

        let prop = align_content(AlignContent::Revert);
        assert_eq!(prop.to_string(), "align-content: revert;");

        let prop = align_content(AlignContent::RevertLayer);
        assert_eq!(prop.to_string(), "align-content: revert-layer;");

        let prop = align_content(AlignContent::Unset);
        assert_eq!(prop.to_string(), "align-content: unset;");
    }
}
