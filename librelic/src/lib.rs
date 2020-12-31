pub mod abs;
pub mod core;
pub mod error;
pub mod model;

pub use crate::error::{RelicError, RelicResult};

// Re-exports
pub use skellige::{fungus, git2, prelude::git};

/// All essential symbols in a simple consumable way
///
/// ### Examples
/// ```
/// use relic::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{abs, core::*, error::*, git2, model::*};
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
