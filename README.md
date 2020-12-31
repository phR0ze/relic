# relic
[![license-badge](https://img.shields.io/crates/l/fungus.svg)](https://opensource.org/licenses/MIT)
[![build](https://github.com/phR0ze/relic/workflows/build/badge.svg?branch=main)](https://github.com/phR0ze/relic/actions)
[![codecov](https://codecov.io/gh/phR0ze/relic/branch/main/graph/badge.svg?token=LZHXZNZcRR)](https://codecov.io/gh/phR0ze/relic)
[![crates.io](https://img.shields.io/crates/v/relic.svg)](https://crates.io/crates/relic)
[![Minimum rustc](https://img.shields.io/badge/rustc-1.30+-lightgray.svg)](https://github.com/phR0ze/gory#rustc-requirements)

***Package management for the Arch Linux packaging ecosystem***

### Quick links
* [Usage](#usage)
  * [Rustc requirments](#rustc-requirements)
* [Research](#research)
  * [Arch build system (ABS)](#arch-build-system-abs)
    * [ABS repository tree](#abs-repository-tree)
    * [asp package](#abs-package)
    * [Build container](#build-container)
    * [Pacman](#pacman)
  * [Arch linux package management (ALPM)](#arch-linux-package-management)
* [Contribute](#contribute)
* [Contribute](#contribute)
  * [Git-Hook](#git-hook)
* [License](#license)
  * [Contribution](#contribution)
* [Backlog](#backlog)
* [Changelog](#changelog)

## Usage <a name="usage"/></a>

#### Requires rustc >= 1.30 <a name="rustc-requirements"/></a>
This minimum rustc requirement is driven by the enhancements made to [Rust's `std::error::Error`
handling improvements](https://doc.rust-lang.org/std/error/trait.Error.html#method.source)

TBD


## Research <a name="research"/></a>
Collecting the research done while developing this project here.

References:
* [Arch Build System](https://wiki.archlinux.org/index.php/Arch_Build_System)
* [Pacman](https://www.archlinux.org/pacman/)
* [Pacman source](https://git.archlinux.org/pacman.git/tree/)
* [Package Management Rosetta Stone](https://wiki.alpinelinux.org/wiki/Comparison_with_other_distros)

## Arch build system (ABS) <a name="arch-build-system-abs"/></a>
The ***Arch build system (ABS)*** packages all the software for the Arch Linux ecosystem and consists
of the following tools:

* ***ABS repository tree*** - a collection of git repositories that contain package descriptions
* ***PKGBUILD*** - a bash script in each software package directory desribing build and packaging steps 
* ***makepkg*** - command line tool for building packages from package directories with PKGBUILD
* ***pacman*** - command line tool for managing Arch Linux packages
* ***AUR*** - the Arch User Repository is a repository similar to the ABS maintained by the community

### ABS repository tree <a name="abs-repository-tree"/></a>
* ***Packages*** - git repo for the Arch Linux `core`, `extra` and `testing` package repositories
* ***Community*** - git repo for the `community` and `multilib` package repositories

Each package has its own subdirectory containing `repos` and `trunk` directories:
* `repos` contains the official arch linux repo configuration
* `trunk` is used for latest development still being tested before being promoted to `repos`

Exmple:
```
acl
acl/repos
acl/repos/core-x86_64
acl/repos/core-x86_64/PKGBUILD
acl/trunk
acl/trunk/PKGBUILD
```

### asp package <a name="asp-package"/></a>
The `asp` package is just a thin wrapper around the svntogit repositories.

To clone the git repository for a specific package use:
```bash
$ asp checkout <pkg-name>
```

To update the cloned repo run:
```bash
$ asp update; git pull
```

### Build container <a name="build-container"/></a>
Building in a clean chroot prevents missing dependencies in packages and allows for a separation from
your current system. The best way to do this is to use a container to build in.

### Pacman <a name="pacman"/></a>
The [pacman package manager](https://wiki.archlinux.org/index.php/pacman) combines a simple binary
package format with an easy-to-use build system. ***pacman*** internally uses the ***libalpm***
library for interacting with the package databases. 

## Arch Linux Package Management (ALPM) <a name="arch-linux-package-management-alpm"/></a>
Arch Linux's package management depends on the ***Arch Linux Package Management (ALPM) library***
`libalpm` for all of its automation.

* [libalpm man](https://www.archlinux.org/pacman/libalpm.3.html)
* [pacman source about](https://git.archlinux.org/pacman.git/about) pacman source README
* [Jguer/go-alpm (MIT)](https://github.com/Jguer/go-alpm) bindings to libalpm in Go
* [derekdreery/alpm (MIT)](https://github.com/derekdreery/alpm) bindings to libalpm in Rust
* [pigeonhands/rust-arch (MIT)](https://github.com/pigeonhands/rust-arch) bindings to libalpm in Rust
* [pigeonhands/kea (MIT)](https://github.com/pigeonhands/kea) aur helper in rust

### ALPM Public Interface <a name="alpm-public-interface"/></a>
`pacman` and `libalpm` are written in C and share the same [git repo](https://git.archlinux.org/pacman.git).
`alpm.h` and `alpm_list.h` constitute the sum of all structures, data and functions declared
available to the frontend i.e. pacman. `pacman` provides a facade to the library. All the library
internal functions are prefixed with `_alpm_` while the public functions are prefixed wih `alpm_`.

### alpm databases <a name="alpm-databases"/></a>

### alpm hooks <a name="alpm-hooks"/></a>
[alpm hooks](https://www.archlinux.org/pacman/alpm-hooks.5.html) provide the ability to specify
scripts to run before or after transactions based on the packages and/or files being modified. Hooks
condist of a single *Action* section describing the action to be run and one or more *Trigger*
sections describing which tranactions it should run.

Hooks are read from files located in the system hook directory `/usr/local/share/libalpm/hooks` and
additional custom directories specified in `pacman.conf` which defaults to
`/usr/local/share/etc/pacman.d/hooks`. File names are required to have the suffix `.hook`. Hooks are
run in alphabetical order of their file names.

### alpm interface <a name="alpm-interface"/></a>

### alpm list <a name="alpm-list"/></a>

### alpm log <a name="alpm-log"/></a>

### alpm misc <a name="alpm-misc"/></a>

### alpm packages <a name="alpm-packages"/></a>

### alpm sync <a name="alpm-sync"/></a>

### alpm trans <a name="alpm-trans"/></a>






## Contribute <a name="Contribute"/></a>
Pull requests are always welcome. However understand that they will be evaluated purely on whether
or not the change fits with my goals/ideals for the project.

### Git-Hook <a name="git-hook"/></a>
Enable the git hooks to have automatic version increments
```bash
cd ~/Projects/relic
git config core.hooksPath .githooks
```

## License <a name="license"/></a>
This project is licensed under either of:
 * MIT license [LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT
 * Apache License, Version 2.0 [LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0

### Contribution <a name="contribution"/></a>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this project by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.

---

## Backlog <a name="backlog"/></a>
* Parse PKGBUILD file
* Add cli for working with relic directly
* ffi bindings for libalpm or rewrite in rust
* pacman database access
  * read `pacman.conf` to locate and load local and sync databases
  * `-Q query` the local database for provided targets
  * `-S sync` search the sync database for provided targets
* rank mirrors
* ABS interactions
  * download, build and package
  * install, update, remove packaages
* Add rust doc comments

## Changelog <a name="changelog"/></a>
