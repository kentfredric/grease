use super::ebuild::{self, Ebuild};
use std::{io::Error, path::PathBuf, result::Result};

/// Represent a discrete gentoo package
pub struct Package {
    root:     PathBuf,
    category: String,
    package:  String,
}
impl Package {
    pub fn new(root: PathBuf, category: String, package: String) -> Package { Package { root, category, package } }

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
    pub fn get_ebuild(&self, name: &str) -> Ebuild {
        ebuild::get(self.root.to_owned(), &self.category, &self.package, name)
    }

    pub fn is_legal(&self) -> bool { self.path().is_dir() }
}

impl std::fmt::Debug for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cat: {}, pn: {}", self.category(), self.pn())
    }
}

/// Get a package
pub fn get(root: PathBuf, category: &str, package: &str) -> Package {
    Package::new(root.to_owned(), category.to_owned(), package.to_owned())
}
