// Arch Linux Build System (ABS)
//
// https://wiki.archlinux.org/index.php/Arch_Build_System
// https://wiki.archlinux.org/index.php/Arch_Build_System#Retrieve_PKGBUILD_source_using_Git
// https://www.archlinux.org/pacman/
// https://git.archlinux.org/pacman.git/tree/
// https://wiki.archlinux.org/index.php/pacman
// https://users.rust-lang.org/t/half-baked-alpm-arch-linux-package-manager-implementation/18969
use crate::errors::*;
use skellige::prelude::*;

const TMPDIR: &str = "abs";
const REPO_PACKAGES_NAME: &str = "packages";
const REPO_COMMUNITY_NAME: &str = "community";
const REPO_BASE: &str = "https://git.archlinux.org/svntogit";

// An repo identifier
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Repo {
    /// Arch Linux packages repository
    Packages,

    /// Arch Linux community repository
    Community,
}
impl Repo {
    pub fn from<T: AsRef<str>>(repo: T) -> RelicResult<Repo> {
        match repo.as_ref() {
            REPO_PACKAGES_NAME => Ok(Repo::Packages),
            REPO_COMMUNITY_NAME => Ok(Repo::Community),
            _ => Err(RelicError::repo_not_found(repo.as_ref().to_string()).into()),
        }
    }
}

/// Get the linux kernel version for the standard `linux` package
///
/// ### Examples
/// ```
/// use relic::prelude::*;
///
/// println!("current linux kernel version: {:?}", abs::kernel_ver().unwrap());
/// ```
pub fn kernel_ver() -> RelicResult<String> {
    // Download source to tmpdir
    let tmpdir = user::temp_dir(TMPDIR)?;
    defer!(sys::remove_all(&tmpdir).unwrap());
    let src = source("linux", &tmpdir)?;

    // Extract the kernel version
    lazy_static! {
        static ref RX: Regex = Regex::new(r"(?m)^pkgver=((\d+\.\d+\.\d+)|(\d+\.\d+)|(\d+)).*").unwrap();
    }
    Ok(sys::extract_string(src.mash("PKGBUILD"), &RX)?)
}

/// Get the repo the given `pkg` lives in.
///
/// ### Examples
/// ```
/// use relic::prelude::*;
///
/// assert_eq!(abs::repo("pkgfile").unwrap(), abs::Repo::Packages);
/// ```
pub fn repo<T: AsRef<str>>(pkg: T) -> RelicResult<Repo> {
    for name in &vec![REPO_PACKAGES_NAME, REPO_COMMUNITY_NAME] {
        let url = format!("{}/{}.git", REPO_BASE, name);
        let branch = format!("packages/{}", pkg.as_ref());
        if git::remote_branch_exists(url, branch).is_ok() {
            return Repo::from(name);
        }
    }
    Err(RelicError::package_not_found(pkg).into())
}

/// Download the package source for `pkg` to `dst`.
///
/// ### Examples
/// ```
/// use relic::prelude::*;
///
/// let tmpdir = PathBuf::from("tests/temp").abs().unwrap().mash("abs_soure_doc");
/// assert!(sys::remove_all(&tmpdir).is_ok());
/// assert!(sys::mkdir(&tmpdir).is_ok());
///
/// assert!(abs::source("pkgfile", &tmpdir).is_ok());
/// assert_eq!(tmpdir.is_dir(), true);
/// assert_eq!(tmpdir.mash("PKGBUILD").exists(), true);
///
/// assert!(sys::remove_all(&tmpdir).is_ok());
/// ```
pub fn source<T: AsRef<str>, U: AsRef<Path>>(pkg: T, dst: U) -> RelicResult<PathBuf> {
    for name in &vec![REPO_PACKAGES_NAME, REPO_COMMUNITY_NAME] {
        let url = format!("{}/{}.git", REPO_BASE, name);
        let branch = format!("packages/{}", pkg.as_ref());

        // Clone the single branch from the repo if it exists
        let tmpdir = user::temp_dir(TMPDIR)?;
        defer!(sys::remove_all(&tmpdir).unwrap());

        // Copy out the target source in <tmpdir>/trunk/* to dst
        if let Ok(_) = git::Repo::new(&tmpdir)?.url(url).branch(branch).branch_only(true).clone() {
            let dir = sys::mkdir(&dst)?;
            sys::copy(tmpdir.mash("trunk/*"), &dir)?;
            return Ok(dir);
        }
    }
    Err(RelicError::package_not_found(pkg).into())
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::prelude::*;

    // Test setup
    fn setup<T: AsRef<Path>>(path: T) -> PathBuf {
        let temp = PathBuf::from("tests/temp").abs().unwrap();
        sys::mkdir(&temp).unwrap();
        temp.mash(path.as_ref())
    }

    #[test]
    fn test_kernel_ver() {
        assert!(abs::kernel_ver().is_ok());

        // Validate regex
        let rx = Regex::new(r"(?m)^pkgver=((\d+\.\d+\.\d+)|(\d+\.\d+)|(\d+)).*").unwrap();

        // 6.arch
        let caps = rx.captures("pkgver=6.arch").unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "6");

        // 5.5.arch
        let caps = rx.captures("pkgver=5.5.arch").unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "5.5");

        // 5.4.14.arch
        let caps = rx.captures("pkgver=5.4.14.arch").unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "5.4.14");
    }

    #[test]
    fn test_repo() {
        assert!(abs::repo("foobar").is_err());
        assert_eq!(abs::repo("pkgfile").unwrap(), abs::Repo::Packages);
        assert_eq!(abs::repo("acme").unwrap(), abs::Repo::Community);
        assert_eq!(abs::repo("linux").unwrap(), abs::Repo::Packages);
    }

    #[test]
    fn test_source() {
        let tmpdir = setup("abs_source");
        assert!(sys::remove_all(&tmpdir).is_ok());

        assert!(abs::source("pkgfile", &tmpdir).is_ok());
        assert_eq!(tmpdir.is_dir(), true);
        assert_eq!(tmpdir.mash("PKGBUILD").exists(), true);
        assert!(abs::source("foobar", &tmpdir).is_err());

        assert!(sys::remove_all(&tmpdir).is_ok());
    }
}
