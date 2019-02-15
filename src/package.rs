
use super::category;
use super::ebuild::{self, Ebuild};
use std::ffi::OsString;
use std::io::Error;
use std::io::ErrorKind::{NotFound,InvalidData};
use std::path::PathBuf;
use std::result::Result;

/// Represent a discrete gentoo package
pub struct Package {
    root: PathBuf,
    category: OsString,
    package: OsString,
}
impl Package {
    fn new(root: PathBuf, category: OsString, package: OsString) -> Package {
        Package {
            root,
            category,
            package,
        }
    }

    /// Return the path to a gentoo package
    pub fn path(&self) -> PathBuf { self.root.join(&self.category).join(&self.package) }

    /// Get the category name of the package
    pub fn category(&self) -> Option<String> { self.category.to_str().map(String::from) }

    /// Get the package name of the package
    pub fn pn(&self) -> Option<String> { self.package.to_str().map(String::from) }

    /// Iterate all ebuilds within the package
    pub fn ebuilds(&self) -> Result<Box<Iterator<Item = Result<Ebuild, Error>>>, Error> {
        ebuild::iterator(
            self.root.to_owned(),
            self.category.to_owned(),
            self.package.to_owned(),
        )
    }

    /// Get a validated ebuild within this category
    pub fn get_ebuild(&self, name: &str) -> Result<Ebuild, Error> {
        match self.package.to_owned().into_string() {
            Ok(pkg) => match self.category.to_owned().into_string() {
                Ok(cat) => ebuild::get(self.root.to_owned(), &cat, &pkg, name),
                Err(_) => Err(Error::new(
                    InvalidData,
                    "Failed converting category to UTF8 String",
                )),
            },
            Err(_) => Err(Error::new(
                    InvalidData,
                    "Failed converting package to UTF8 String",
            )),
        }
    }
}

impl std::fmt::Debug for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let none_str = || String::from("None");
        write!(f, "cat: {}, pn: {}",
               self.category().unwrap_or_else(none_str),
               self.pn().unwrap_or_else(none_str),
        )
    }
}

/// Create an iterator of all Packages
pub fn iterator(root: PathBuf, category: OsString) -> Result<Box<Iterator<Item = Result<Package, Error>>>, Error> {
    Ok(Box::new(
        root.join(&category).read_dir()?
        .filter(move |e| if let Ok(entry) = e {
            entry.path().is_dir()
        } else {
            // readdir entry failures passthrough
            true
        })
        // Munge Ok(), passthru Err()
        .map( move |e| e.map(  |ent|
                         Package::new( root.to_owned(), category.to_owned(), ent.file_name() ) )),
    ))
}

/// Get a validated package
pub fn get(root: PathBuf, category: &str, package: &str) -> Result<Package, Error> {
    let my_root = root.to_owned();
    category::get(root, category).and_then(|cat| {
        let pkg_path = cat.path().join(package);
        if pkg_path.exists() && pkg_path.is_dir() {
            Ok(Package::new(
                my_root.to_owned(),
                OsString::from(category),
                OsString::from(package),
            ))
        } else {
            Err(Error::new(NotFound, "Package not found/ not a directory"))
        }
    })
}
