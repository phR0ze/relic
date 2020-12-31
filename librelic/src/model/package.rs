/// The Arch Linux package provides access to common properties across the different databases
pub trait Package {
    // Name of this package (e.g. linux)
    fn name(&self) -> &str;

    /// The package version (e.g. 5.4.15.arch1-1)
    fn version(&self) -> &str;

    /// The base of this package.
    fn base(&self) -> Option<&str>;

    /// The package description (e.g. The Linux kernel and modules)
    fn desc(&self) -> &str;

    /// Architecture this package is meant to be installed on (e.g. x86_64)
    fn arch(&self) -> &str;

    /// The package URL (e.g. https://git.archlinux.org/linux.git/log/?h=v5.4.15-arch1)
    fn url(&self) -> &str;

    /// The package licenses (e.g. GPL2)
    fn licenses(&self) -> &[String];

    /// The package groups this package is in (e.g. None)
    fn groups(&self) -> &[String];

    /// The virtual packages this package provides (e.g. None)
    fn provides(&self) -> &[String];

    /// The packages this package depends on (e.g. coreutils kmod initramfs)
    fn depends(&self) -> &[String];

    /// The packages this package optionally depends on (e.g. crda: to set the correcdt wireless channels, linux-firmware: firmware images needed for some devices)
    fn optional_depends(&self) -> &[String];

    /// Packages make depends on
    fn make_depends(&self) -> &[String];

    /// Packages this package depends on when checking the build
    fn check_depends(&self) -> &[String];

    /// The packages this package conflicts with (e.g. None)
    fn conflicts(&self) -> &[String];

    /// The packages this package replaces (e.g. None)
    fn replaces(&self) -> &[String];

    /// The size in bytes of this package
    fn size(&self) -> u64;

    /// The package creator's name (e.g. Jan Alexander Steffens (heftig) <jan.steffens@gmail.com>)
    fn packager(&self) -> &str;

    /// The package's build date (e.g. Sun 26 Jan 2020 02:48:50 AM MST)
    fn build_date(&self) -> &str;
}
