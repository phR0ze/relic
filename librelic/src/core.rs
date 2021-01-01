//! `relic` provides Arch Linux build and package automation
//!
//! ## About
//!
//! `relic` provides Arch Linux build and package automation
use crate::{error::*, model::*};
use log::info;
use skellige::prelude::*;
use std::{cell::RefCell, fmt, rc::Rc};

pub const APP_NAME: &'static str = "RELIC";
pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
pub const APP_GIT_COMMIT: &'static str = env!("APP_GIT_COMMIT");
pub const APP_BUILD_DATE: &'static str = env!("APP_BUILD_DATE");

// Relic implementation
// -------------------------------------------------------------------------------------------------
pub struct Relic {
    pub(crate) init: bool,
    pub(crate) test: bool,
    pub(crate) debug: bool,
    pub(crate) quiet: bool,
    pub(crate) config_path: PathBuf,
    pub(crate) config_dir: PathBuf,
    pub(crate) config_dir_set: bool,
    pub(crate) data_dir: PathBuf,
    pub(crate) data_dir_set: bool,
    pub(crate) out: Rc<RefCell<dyn io::Write>>,
    config_w: usize, // configuration width to use for output
}
impl Default for Relic {
    fn default() -> Self {
        Self {
            init: Default::default(),
            test: Default::default(),
            debug: Default::default(),
            quiet: Default::default(),
            config_path: Default::default(),
            config_dir: Default::default(),
            config_dir_set: Default::default(),
            data_dir: Default::default(),
            data_dir_set: Default::default(),
            out: Rc::new(RefCell::new(io::stdout())),
            config_w: 22,
        }
    }
}
impl Relic {
    /// Create a new relic instance with defaults.
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    // Property Setters
    // ---------------------------------------------------------------------------------------------

    /// Set test mode with the given `yes` value.
    pub fn with_test(mut self, yes: bool) -> Self {
        self.test = yes;
        self
    }

    /// Set debug mode with the given `yes` value.
    pub fn with_debug(mut self, yes: bool) -> Self {
        self.debug = yes;
        self
    }

    /// Set quiet mode with the given `yes` value.
    pub fn with_quiet(mut self, yes: bool) -> Self {
        self.quiet = yes;
        self
    }

    /// Set the config_dir to use `[default: ~/.config/relic]`
    pub fn with_config_dir<T: AsRef<Path>>(mut self, path: Option<T>) -> RelicResult<Self> {
        if let Some(x) = path {
            self.config_dir_set = true;
            self.config_dir = x.as_ref().abs()?;
            info!("{:>w$} {}", "setting config_dir:", self.config_dir.cyan(), w = self.config_w);
            self.config_path = self.config_dir.mash(format!("{}.yaml", APP_NAME));
        }
        Ok(self)
    }

    /// Set the data_dir to use `[default: ~/.local/share/relic]`
    pub fn with_data_dir<T: AsRef<Path>>(mut self, path: Option<T>) -> RelicResult<Self> {
        if let Some(x) = path {
            self.data_dir_set = true;
            self.data_dir = x.as_ref().abs()?;
            info!("{:>w$} {}", "setting data_dir:", self.data_dir.cyan(), w = self.config_w);
        }
        Ok(self)
    }

    // Core functions
    // ---------------------------------------------------------------------------------------------

    /// Initialize relic with the configured options
    pub fn init(&mut self) -> RelicResult<()> {
        if self.init {
            return Ok(());
        }

        info!("{}", format!("<< =================={{ {} v{} }}================== >>", APP_NAME, APP_VERSION).green());

        // Configure relic
        self.load_config()?;

        self.init = true;
        Ok(())
    }

    /// Save the current configuration. By default this will be at
    /// $XDG_CONFIG_HOME/relic/relic.yaml unless overridden.
    pub fn save_config(&self) -> RelicResult<()> {
        info!("{}{}", "Persisting configuration: ".yellow(), self.config_path.cyan());
        let config = Config::new();
        config.save(&self.config_path)?;
        Ok(())
    }

    /// Get package info for the given packages
    pub fn info<T: AsRef<str>>(&mut self, pkgs: &[T]) -> RelicResult<()> {
        let pkgstr = pkgs.iter().map(|x| x.as_ref()).collect::<Vec<&str>>().join(", ");
        info!("{}{}", "View package information for: ".yellow(), pkgstr.cyan());
        for x in pkgs {
            println!("{}", x.as_ref());
        }
        Ok(())
    }

    /// Remove the given `components`
    pub fn remove<T: AsRef<[Component]>>(&mut self, components: T) -> RelicResult<()> {
        if components.as_ref().iter().any(|x| x == &Component::None) {
            return Err(ComponentError::InvalidName.into());
        }
        let mut component_vec = components.as_ref().iter().map(|x| x.to_owned()).collect();

        // Remove all components
        // -----------------------------------------------------------------------------------------
        if components.as_ref().iter().any(|x| x == &Component::All) {
            component_vec = Component::all();
        }

        info!("{}{}", "Removing components: ".yellow(), Component::join(&component_vec).cyan());
        for component in component_vec {
            let path = match component {
                Component::Config => &self.config_dir,
                _ => continue,
            };
            if sys::exists(path) {
                info!("Removing: {}", path.to_string()?.cyan());
                sys::remove_all(&path)?;
            } else {
                info!("Doesn't exist: {}", path.to_string()?.cyan());
            }
        }
        Ok(())
    }

    // Private functions
    // ---------------------------------------------------------------------------------------------

    // Load the current configuration
    pub(crate) fn load_config(&mut self) -> RelicResult<()> {
        info!("{}", "Loading configuration...".yellow());

        // Config dir by default is $XDG_CONFIG_HOME/relic/relic.yaml
        if !self.config_dir_set {
            self.config_dir = user::config_dir()?.mash("relic");
            self.config_path = self.config_dir.mash(format!("{}.yaml", APP_NAME));
            info!("{:>w$} {}", "defaulting config_dir:", self.config_dir.cyan(), w = self.config_w);
        }

        // Data dir by default is $XDG_DATA_HOME/relic
        if !self.data_dir_set {
            self.data_dir = user::data_dir()?.mash("relic");
            info!("{:>w$} {}", "defaulting data_dir:", self.data_dir.cyan(), w = self.config_w);
        }
        Ok(())
    }

    // Implement support for write*! macro varients to use Relic as a Writer.
    // We actually don't need to implement the entire fmt::Write trait only this func
    // as macros don't seem to honor the full trait contract only existance of the func.
    pub fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) {
        if !self.quiet {
            self.out.borrow_mut().write_fmt(fmt).unwrap();
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::prelude::*;

//     // Test setup
//     fn setup<T: AsRef<Path>>(path: T) -> Relic {
//         let temp = PathBuf::from("tests/temp").abs().unwrap();
//         sys::mkdir(&temp).unwrap();
//         let root = temp.mash(path.as_ref());
//         assert!(sys::remove_all(&root).is_ok());

//         let mut relic = Relic::new();
//         relic.quiet(true);
//         assert!(relic.config_dir(&root).is_ok());
//         assert!(relic.data_dir(&root).is_ok());
//         assert!(relic.init().is_ok());
//         relic
//     }

//     #[test]
//     fn test_info() {
//         let mut relic = setup("core_info");

//         assert!(relic.info(&["foo", "bar"]).is_ok());
//         assert!(sys::remove_all(&relic.data_dir).is_ok());
//     }
// }
