use std::{error::Error as StdError, fmt};

// An error indicating that something went wrong with a component operation
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ComponentError {
    /// An error indicating that an invalid component name was given
    InvalidName,
}

impl fmt::Display for ComponentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ComponentError::InvalidName => write!(f, "invalid component name was given"),
        }
    }
}

impl StdError for ComponentError {}

#[cfg(test)]
mod tests {
    use crate::error::*;

    #[test]
    fn test_errors() {
        assert_eq!("invalid component name was given", format!("{}", ComponentError::InvalidName));
    }
}
