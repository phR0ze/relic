pub mod abs;
pub mod errors;

/// Types exported directly
pub use crate::errors::RelicError;
pub use crate::errors::RelicResult;

// Re-exports
pub use skellige::fungus;
pub use skellige::git2;
pub use skellige::prelude::git;

/// All essential symbols in a simple consumable way
///
/// ### Examples
/// ```
/// use relic::prelude::*;
/// ```
pub mod prelude {
    pub use super::abs;
    pub use super::errors::*;
    pub use super::git2;
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
