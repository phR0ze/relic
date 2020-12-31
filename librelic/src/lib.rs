pub mod abs;
pub mod error;

/// Types exported directly
pub use crate::error::RelicError;
pub use crate::error::RelicResult;

// Re-exports
pub use skellige::{fungus, git2, prelude::git};

/// All essential symbols in a simple consumable way
///
/// ### Examples
/// ```
/// use relic::prelude::*;
/// ```
pub mod prelude {
    pub use super::{abs, error::*, git2};
    pub use skellige::prelude::*;

    // Re-exports
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_color_enabled() {
    //     assert!(Color::enabled() || !Color::enabled());
    // }
}
