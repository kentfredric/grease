extern crate once_cell;

use super::package;
use super::version::{self, Version};
use once_cell::sync::OnceCell;
use std::ffi::OsString;
use std::io::Error;
use std::io::ErrorKind::NotFound;
use std::option::Option;
use std::path::PathBuf;
use std::result::Result;

/// Represent a discrete Gentoo ebuild
pub struct Ebuild {
    root: PathBuf,
    category: OsString,
    package: OsString,
    ebuild: OsString,
    version: OnceCell<Version>,
}

impl Ebuild {
    fn new(root: PathBuf, category: OsString, package: OsString, ebuild: OsString) -> Ebuild {
        Ebuild {
            root,
            category,
            package,
            ebuild,
            version: OnceCell::INIT,
        }
    }

    /// Returns a path to the ebuild file
    pub fn path(&self) -> PathBuf {
        self.root.join(&self.category).join(&self.package).join(
            &self.ebuild,
        )
    }

    pub fn version(&self) -> &Version {
        self.version.get_or_init(|| {
            version::parse(&ebuild_to_pvr(
                self.package.to_owned(),
                self.ebuild.to_owned(),
            ))
        })
    }

    /// Returns the ebuilds category similar to `PMS` variable `CATEGORY`
    pub fn category(&self) -> Option<String> { self.category.to_str().map(String::from) }

    /// Returns the ebuilds package name similar to `PMS` variable `PN`
    pub fn pn(&self) -> Option<String> { self.package.to_str().map(String::from) }

    /// Returns the ebuilds full package version similar to `PMS` variable `PF`
    pub fn pf(&self) -> Option<String> {
        self.path().file_stem().and_then(|osstr| {
            osstr.to_str().map(String::from)
        })
    }
    /// Returns the ebuilds version with revision similar to `PMS` variable
    /// `PVR`
    pub fn pvr(&self) -> String {
        self.version().pvr()
    }
    /// Returns the ebuilds version without revision similar to `PMS` variable
    /// `PV`
    pub fn pv(&self) -> &str { self.version().pv() }
    /// Returns the ebuilds revision, or r0, similar to `PMS` variable `PR`
    pub fn pr(&self) -> &str { self.version().pr() }

    /// Returns the ebuilds package name without revision, similar to `PMS`
    /// variable `P`
    pub fn p(&self) -> Option<String> { self.pn().and_then(|pn| Some(pn + &self.pv())) }
}

impl std::fmt::Debug for Ebuild {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let none_str = || String::from("None");
        write!(f, "cat: {}, pf: {}, pn: {}, pvr: {} pv: {} pr: {}",
               self.category().unwrap_or_else(none_str),
               self.pf().unwrap_or_else(none_str),
               self.pn().unwrap_or_else(none_str),
               self.pvr(),
               self.pv(),
               self.pr(),
        )
    }
}

fn ebuild_to_pvr(package: OsString, ebuild: OsString) -> String {
    let package_str = package.to_str().map(String::from).expect(
        "Can't decode \
         package:OsString to \
         UTF8 String",
    );
    let ebuild_str = ebuild.to_str().map(String::from).expect(
        "Can't decode \
         ebuild:OsStrting to UTF8 \
         String",
    );
    let pf = ebuild_str.trim_end_matches(".ebuild");
    pf.trim_start_matches((package_str + "-").as_str())
        .to_owned()
}

/// Iterate all ebuilds within a package
pub fn iterator(root: PathBuf, category: OsString, package: OsString)
    -> Result<Box<Iterator<Item = Result<Ebuild, Error>>>, Error> {
    Ok(Box::new(
        root.join(&category)
            .join(&package)
            .read_dir()?
            .filter(|e| if let Ok(entry) = e {
                let p = entry.path();
                if let Some(ext) = p.extension() {
                    ext.eq("ebuild") && !p.is_dir()
                } else {
                    false
                }
            } else {
                true
            })
            .map(move |dirent| {
                dirent.map(|entry| {
                    Ebuild::new(
                        root.to_owned(),
                        category.to_owned(),
                        package.to_owned(),
                        entry.file_name(),
                    )
                })
            }),
    ))
}

/// Get a validated Ebuild object by explicit path
pub fn get(root: PathBuf, category: &str, package: &str, ebuild: &str) -> Result<Ebuild, Error> {
    let my_root = root.to_owned();
    package::get(root, category, package).and_then(|pkg| {
        let ebuild_path = pkg.path().join(ebuild);
        if ebuild_path.exists() && !ebuild_path.is_dir() {
            Ok(Ebuild::new(
                my_root.to_owned(),
                OsString::from(category),
                OsString::from(package),
                OsString::from(ebuild),
            ))
        } else {
            Err(Error::new(NotFound, "Ebuild not found/ is a directory"))
        }
    })
}
