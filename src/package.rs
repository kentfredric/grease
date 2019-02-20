use super::{
    category,
    ebuild::{self, Ebuild},
};
use std::{
    io::{Error, ErrorKind::NotFound},
    path::PathBuf,
    result::Result,
};

/// Represent a discrete gentoo package
pub struct Package {
    root:     PathBuf,
    category: String,
    package:  String,
}
impl Package {
    fn new(root: PathBuf, category: String, package: String) -> Package { Package { root, category, package } }

    /// Return the path to a gentoo package
    pub fn path(&self) -> PathBuf { self.root.join(&self.category).join(&self.package) }

    /// Get the category name of the package
    pub fn category(&self) -> String { self.category.to_owned() }

    /// Get the package name of the package
    pub fn pn(&self) -> String { self.package.to_owned() }

    pub fn name(&self) -> String { self.category.to_owned() + "/" + &self.package }

    /// Iterate all ebuilds within the package
    pub fn ebuilds(&self) -> Result<Box<dyn Iterator<Item = Result<Ebuild, Error>>>, Error> {
        ebuild::iterator(self.root.to_owned(), self.category.to_owned(), self.package.to_owned())
    }

    /// Get a validated ebuild within this category
    pub fn get_ebuild(&self, name: &str) -> Result<Ebuild, Error> {
        ebuild::get(self.root.to_owned(), &self.category, &self.package, name)
    }
}

impl std::fmt::Debug for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cat: {}, pn: {}", self.category(), self.pn())
    }
}

/// Create an iterator of all Packages
pub fn iterator(root: PathBuf, category: String) -> Result<Box<dyn Iterator<Item = Result<Package, Error>>>, Error> {
    Ok(Box::new(
        root.join(&category).read_dir()?
        .filter(move |e| if let Ok(entry) = e {
            entry.path().is_dir()
        } else {
            // readdir entry failures passthrough
            true
        })
        // Munge Ok(), passthru Err()
        .map( move |e| e.map(  |ent| {
                        let ent_fn = ent.file_name();
                         Package::new( root.to_owned(), category.to_owned(), ent_fn.to_str().expect("Could not decode filename as UTF8").to_owned()  )
            }
    )),
    ))
}

/// Get a validated package
pub fn get(root: PathBuf, category: &str, package: &str) -> Result<Package, Error> {
    let my_root = root.to_owned();
    category::get(root, category).and_then(|cat| {
        let pkg_path = cat.path().join(package);
        if pkg_path.exists() && pkg_path.is_dir() {
            Ok(Package::new(my_root.to_owned(), category.to_owned(), package.to_owned()))
        } else {
            Err(Error::new(NotFound, "Package not found/ not a directory"))
        }
    })
}
