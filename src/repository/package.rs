//! Representation of a package in a Gentoo repository
use crate::{repository::Ebuild, util::optfilter::OptFilter};
use std::{io::Error, path::PathBuf, result::Result};
/// Represent a discrete gentoo package
pub struct Package {
    root:     PathBuf,
    category: String,
    package:  String,
}
impl Package {
    /// Construct a new Package Type Object
    pub fn new(root: PathBuf, category: String, package: String) -> Package { Package { root, category, package } }

    /// Return the path to a gentoo package
    pub fn path(&self) -> PathBuf { self.root.join(&self.category).join(&self.package) }

    /// Get the category name of the package
    pub fn category(&self) -> String { self.category.to_owned() }

    /// Get the package name of the package
    pub fn pn(&self) -> String { self.package.to_owned() }

    /// Get a full identifier of this package
    pub fn name(&self) -> String { self.category.to_owned() + "/" + &self.package }

    /// Iterate all ebuilds within the package
    pub fn ebuilds(&self) -> Result<Box<dyn Iterator<Item = Result<Ebuild, Error>>>, Error> {
        let root = self.root.to_owned();
        let category = self.category.to_owned();
        let package = self.package.to_owned();
        Ok(Box::new(
            root.join(&category)
                .join(&package)
                .read_dir()?
                .filter_oks(|entry| if let Some(ext) = entry.path().extension() { ext.eq("ebuild") } else { false })
                .map_oks(move |entry| {
                    let e_fn = entry.file_name();
                    let e = e_fn.to_str().expect("Could not decode filename to UTF8");
                    Ok(Ebuild::new(root.to_owned(), category.to_owned(), package.to_owned(), e.to_owned()))
                }),
        ))
    }

    /// Get an ebuild within this category
    pub fn get_ebuild(&self, name: &str) -> Ebuild {
        Ebuild::new(self.root.to_owned(), self.category.to_owned(), self.package.to_owned(), name.to_string())
    }

    /// Determine if this package is a legal package
    pub fn is_legal(&self) -> bool { self.path().is_dir() }
}

impl std::fmt::Debug for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cat: {}, pn: {}", self.category(), self.pn())
    }
}

impl crate::util::repoobject::RepoObject for Package {
    fn name(&self) -> String { self.package.to_owned() }

    fn path(&self) -> PathBuf { self.root.join(&self.category).join(&self.package) }

    fn ident(&self) -> String { self.category.to_owned() + "/" + &self.package }

    fn components(&self) -> String { format!("cat={} package={}", &self.category, &self.package) }
}
