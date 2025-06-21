//! # Align Items Property
//!
//! This module provides a function for creating the CSS `align-items` property.
//! The `align-items` property sets the align-self value on all direct children as a group.
//! In flexbox, it controls the alignment of items on the cross axis.
//! In grid layout, it controls the alignment of items on the block axis within their grid areas.
//!
//! ## Syntax
//!
//! ```css
//! /* Basic keywords */
//! align-items: normal;
//! align-items: stretch;
//!
//! /* Positional alignment */
//! align-items: center;
//! align-items: start;
//! align-items: end;
//! align-items: flex-start;
//! align-items: flex-end;
//! align-items: self-start;
//! align-items: self-end;
//! align-items: anchor-center;
//!
//! /* Baseline alignment */
//! align-items: baseline;
//! align-items: first baseline;
//! align-items: last baseline;
//!
//! /* Overflow alignment */
//! align-items: safe center;
//! align-items: unsafe center;
//!
//! /* Global values */
//! align-items: inherit;
//! align-items: initial;
//! align-items: revert;
//! align-items: revert-layer;
//! align-items: unset;
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use mew_css::properties::align_items;
//! use mew_css::values::AlignItems;
//!
//! let prop = align_items::align_items(AlignItems::Center);
//! assert_eq!(prop.to_string(), "align-items: center;");
//!
//! let prop = align_items::align_items(AlignItems::Stretch);
//! assert_eq!(prop.to_string(), "align-items: stretch;");
//! ```

use crate::properties::Property;
use crate::values::AlignItems;

/// Creates a CSS `align-items` property.
///
/// The `align-items` property sets the align-self value on all direct children as a group.
/// In flexbox, it controls the alignment of items on the cross axis.
/// In grid layout, it controls the alignment of items on the block axis within their grid areas.
///
/// ## Values
///
/// - `normal`: The effect of this keyword is dependent of the layout mode.
/// - `center`: The flex items' margin boxes are centered within the line on the cross-axis.
/// - `start`: The items are packed flush to each other toward the start edge of the alignment container.
/// - `end`: The items are packed flush to each other toward the end edge of the alignment container.
/// - `flex-start`: Used in flex layout only, aligns the flex items flush against the flex container's cross-start side.
/// - `flex-end`: Used in flex layout only, aligns the flex items flush against the flex container's cross-end side.
/// - `self-start`: The items are packed flush to the edge of the alignment container's start side of the item.
/// - `self-end`: The items are packed flush to the edge of the alignment container's end side of the item.
/// - `baseline`, `first-baseline`, `last-baseline`: All flex items are aligned such that their baselines align.
/// - `stretch`: If the items are smaller than the alignment container, auto-sized items will be equally enlarged to fill the container.
/// - `anchor-center`: Aligns the items to the center of the associated anchor element in the block direction.
/// - `safe center`, `unsafe center`: Overflow alignment options.
/// - `inherit`, `initial`, `revert`, `revert-layer`, `unset`: Global values.
///
/// # Arguments
///
/// * `value` - The align-items value to use
///
/// # Returns
///
/// A new `Property` instance representing the align-items property
///
/// # Examples
///
/// ```rust
/// use mew_css::properties::align_items;
/// use mew_css::values::AlignItems;
///
/// // Basic keywords
/// let prop = align_items::align_items(AlignItems::Normal);
/// assert_eq!(prop.to_string(), "align-items: normal;");
///
/// let prop = align_items::align_items(AlignItems::Stretch);
/// assert_eq!(prop.to_string(), "align-items: stretch;");
///
/// // Positional alignment
/// let prop = align_items::align_items(AlignItems::Center);
/// assert_eq!(prop.to_string(), "align-items: center;");
///
/// // Baseline alignment
/// let prop = align_items::align_items(AlignItems::Baseline);
/// assert_eq!(prop.to_string(), "align-items: baseline;");
/// ```
pub fn align_items(value: AlignItems) -> Property {
    Property::new("align-items", value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::AlignItems;

    #[test]
    fn test_basic_keywords() {
        let prop = align_items(AlignItems::Normal);
        assert_eq!(prop.to_string(), "align-items: normal;");

        let prop = align_items(AlignItems::Stretch);
        assert_eq!(prop.to_string(), "align-items: stretch;");
    }

    #[test]
    fn test_positional_alignment() {
        let prop = align_items(AlignItems::Center);
        assert_eq!(prop.to_string(), "align-items: center;");

        let prop = align_items(AlignItems::Start);
        assert_eq!(prop.to_string(), "align-items: start;");

        let prop = align_items(AlignItems::End);
        assert_eq!(prop.to_string(), "align-items: end;");

        let prop = align_items(AlignItems::FlexStart);
        assert_eq!(prop.to_string(), "align-items: flex-start;");

        let prop = align_items(AlignItems::FlexEnd);
        assert_eq!(prop.to_string(), "align-items: flex-end;");

        let prop = align_items(AlignItems::SelfStart);
        assert_eq!(prop.to_string(), "align-items: self-start;");

        let prop = align_items(AlignItems::SelfEnd);
        assert_eq!(prop.to_string(), "align-items: self-end;");

        let prop = align_items(AlignItems::AnchorCenter);
        assert_eq!(prop.to_string(), "align-items: anchor-center;");
    }

    #[test]
    fn test_baseline_alignment() {
        let prop = align_items(AlignItems::Baseline);
        assert_eq!(prop.to_string(), "align-items: baseline;");

        let prop = align_items(AlignItems::FirstBaseline);
        assert_eq!(prop.to_string(), "align-items: first baseline;");

        let prop = align_items(AlignItems::LastBaseline);
        assert_eq!(prop.to_string(), "align-items: last baseline;");
    }

    #[test]
    fn test_overflow_alignment() {
        let prop = align_items(AlignItems::SafeCenter);
        assert_eq!(prop.to_string(), "align-items: safe center;");

        let prop = align_items(AlignItems::UnsafeCenter);
        assert_eq!(prop.to_string(), "align-items: unsafe center;");
    }

    #[test]
    fn test_global_values() {
        let prop = align_items(AlignItems::Inherit);
        assert_eq!(prop.to_string(), "align-items: inherit;");

        let prop = align_items(AlignItems::Initial);
        assert_eq!(prop.to_string(), "align-items: initial;");

        let prop = align_items(AlignItems::Revert);
        assert_eq!(prop.to_string(), "align-items: revert;");

        let prop = align_items(AlignItems::RevertLayer);
        assert_eq!(prop.to_string(), "align-items: revert-layer;");

        let prop = align_items(AlignItems::Unset);
        assert_eq!(prop.to_string(), "align-items: unset;");
    }
}