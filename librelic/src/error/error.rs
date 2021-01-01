use crate::error::ComponentError;
use skellige::{fungus::errors::*, prelude::git};
use std::{error::Error as StdError, fmt, io};

/// `Result<T>` provides a simplified result type with a common error type
pub type RelicResult<T> = std::result::Result<T, RelicError>;

// An error indicating that something went wrong with an arch linux operation
#[derive(Debug)]
pub enum RelicError {
    // An error from the component module
    Component(ComponentError),

    // std::io::Error from lower down
    Io(io::Error),

    /// An error indicating that the given package was not found.
    PackageNotFound(String),

    /// An error indicating that the given repo was not found.
    RepoNotFound(String),

    /// An error from the fungus crate
    Fungus(FuError),

    // An error from the serde_yaml crate
    SerdeYaml(serde_yaml::Error),

    /// An error from the skellige crate
    Skellige(git::Error),
}
impl RelicError {
    /// Return an error indicating that the given package was not found.
    pub fn package_not_found<T: AsRef<str>>(pkg: T) -> RelicError {
        RelicError::PackageNotFound(pkg.as_ref().to_string())
    }

    /// Return an error indicating that the given repo was not found.
    pub fn repo_not_found<T: AsRef<str>>(repo: T) -> RelicError {
        RelicError::RepoNotFound(repo.as_ref().to_string())
    }

    /// Implemented directly on the `Error` type to reduce casting required
    pub fn is<T: StdError+'static>(&self) -> bool {
        self.as_ref().is::<T>()
    }

    /// Implemented directly on the `Error` type to reduce casting required
    pub fn downcast_ref<T: StdError+'static>(&self) -> Option<&T> {
        self.as_ref().downcast_ref::<T>()
    }

    /// Implemented directly on the `Error` type to reduce casting required
    pub fn downcast_mut<T: StdError+'static>(&mut self) -> Option<&mut T> {
        self.as_mut().downcast_mut::<T>()
    }

    /// Implemented directly on the `Error` type to reduce casting required
    /// which allows for using as_ref to get the correct pass through.
    pub fn source(&self) -> Option<&(dyn StdError+'static)> {
        self.as_ref().source()
    }
}
impl StdError for RelicError {}

impl fmt::Display for RelicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RelicError::Component(ref err) => write!(f, "{}", err),
            RelicError::Io(ref err) => write!(f, "{}", err),
            RelicError::PackageNotFound(ref pkg) => write!(f, "failed to find package: {}", pkg),
            RelicError::RepoNotFound(ref repo) => write!(f, "failed to find repo: {}", repo),
            RelicError::Fungus(ref err) => write!(f, "{}", err),
            RelicError::SerdeYaml(ref err) => write!(f, "{}", err),
            RelicError::Skellige(ref err) => write!(f, "{}", err),
        }
    }
}

impl AsRef<dyn StdError> for RelicError {
    fn as_ref(&self) -> &(dyn StdError+'static) {
        match *self {
            RelicError::Component(ref err) => err,
            RelicError::Io(ref err) => err,
            RelicError::PackageNotFound(_) => self,
            RelicError::RepoNotFound(_) => self,
            // Call as_ref on inner to make transparent
            RelicError::Fungus(ref err) => err.as_ref(),
            RelicError::SerdeYaml(ref err) => err as &(dyn StdError+'static),
            RelicError::Skellige(ref err) => err.as_ref(),
        }
    }
}

impl AsMut<dyn StdError> for RelicError {
    fn as_mut(&mut self) -> &mut (dyn StdError+'static) {
        match *self {
            RelicError::Component(ref mut err) => err,
            RelicError::Io(ref mut err) => err,
            RelicError::PackageNotFound(_) => self,
            RelicError::RepoNotFound(_) => self,
            // Call as_ref on inner to make transparent
            RelicError::Fungus(ref mut err) => err.as_mut(),
            RelicError::SerdeYaml(ref mut err) => err as &mut (dyn StdError+'static),
            RelicError::Skellige(ref mut err) => err.as_mut(),
        }
    }
}

impl From<ComponentError> for RelicError {
    fn from(err: ComponentError) -> RelicError {
        RelicError::Component(err)
    }
}

impl From<io::Error> for RelicError {
    fn from(err: io::Error) -> RelicError {
        RelicError::Io(err)
    }
}

impl From<FuError> for RelicError {
    fn from(err: FuError) -> RelicError {
        RelicError::Fungus(err)
    }
}

impl From<git::Error> for RelicError {
    fn from(err: git::Error) -> RelicError {
        RelicError::Skellige(err)
    }
}

impl From<serde_yaml::Error> for RelicError {
    fn from(err: serde_yaml::Error) -> RelicError {
        RelicError::SerdeYaml(err)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_errors() {
        // PackageNotFound(String),
        let mut err = RelicError::PackageNotFound("foo".to_string());
        assert_eq!(RelicError::package_not_found("foo").to_string(), err.to_string());
        assert_eq!("failed to find package: foo", err.to_string());
        assert_eq!("failed to find package: foo", err.as_ref().to_string());
        assert_eq!("failed to find package: foo", err.as_mut().to_string());
        assert!(err.is::<RelicError>());
        assert!(err.downcast_ref::<RelicError>().is_some());
        assert!(err.downcast_mut::<RelicError>().is_some());
        assert!(err.source().is_none());

        // RepoNotFound(String),
        let mut err = RelicError::RepoNotFound("foo".to_string());
        assert_eq!(RelicError::repo_not_found("foo").to_string(), err.to_string());
        assert_eq!("failed to find repo: foo", err.to_string());
        assert_eq!("failed to find repo: foo", err.as_ref().to_string());
        assert_eq!("failed to find repo: foo", err.as_mut().to_string());
        assert!(err.is::<RelicError>());
        assert!(err.downcast_ref::<RelicError>().is_some());
        assert!(err.downcast_mut::<RelicError>().is_some());
        assert!(err.source().is_none());

        // Fungus(FuError),
        let mut err = RelicError::from(FuError::from(FileError::FailedToExtractString));
        assert_eq!("failed to extract string from file", err.to_string());
        assert_eq!("failed to extract string from file", err.as_ref().to_string());
        assert_eq!("failed to extract string from file", err.as_mut().to_string());
        assert!(err.is::<FileError>());
        assert!(err.downcast_ref::<FileError>().is_some());
        assert!(err.downcast_mut::<FileError>().is_some());
        assert!(err.source().is_none());

        // Skellige(git::Error), - transparent
        let mut err = RelicError::from(git::Error::from(FuError::from(FileError::FailedToExtractString)));
        assert_eq!("failed to extract string from file", err.to_string());
        assert_eq!("failed to extract string from file", err.as_ref().to_string());
        assert_eq!("failed to extract string from file", err.as_mut().to_string());
        assert!(err.is::<FileError>());
        assert!(err.downcast_ref::<FileError>().is_some());
        assert!(err.downcast_mut::<FileError>().is_some());
        assert!(err.source().is_none());

        // Skellige(git::Error), - transparent
        let mut err = git::Error::from(git2::Error::new(git2::ErrorCode::Ambiguous, git2::ErrorClass::Checkout, "foo"));
        assert_eq!("foo; class=Checkout (20); code=Ambiguous (-5)", err.to_string());
        assert_eq!("foo; class=Checkout (20); code=Ambiguous (-5)", err.as_ref().to_string());
        assert_eq!("foo; class=Checkout (20); code=Ambiguous (-5)", err.as_mut().to_string());
        assert!(err.is::<git2::Error>());
        assert!(err.downcast_ref::<git2::Error>().is_some());
        assert!(err.downcast_mut::<git2::Error>().is_some());
        assert!(err.source().is_none());
    }
}
