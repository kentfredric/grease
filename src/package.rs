use super::ebuild::Ebuild;
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
        let root = self.root.to_owned();
        let category = self.category.to_owned();
        let package = self.package.to_owned();
        Ok(Box::new(
            root.join(&category)
                .join(&package)
                .read_dir()?
                .filter(|e| {
                    if let Ok(entry) = e {
                        let p = entry.path();
                        if let Some(ext) = p.extension() {
                            ext.eq("ebuild") && !p.is_dir()
                        } else {
                            false
                        }
                    } else {
                        true
                    }
                })
                .map(move |dirent| {
                    dirent.map(|entry| {
                        let e_fn = entry.file_name();
                        let e = e_fn.to_str().expect("Could not decode filename to UTF8");
                        Ebuild::new(root.to_owned(), category.to_owned(), package.to_owned(), e.to_owned())
                    })
                }),
        ))
    }

    /// Get an ebuild within this category
    pub fn get_ebuild(&self, name: &str) -> Ebuild {
        Ebuild::new(self.root.to_owned(), self.category.to_owned(), self.package.to_owned(), name.to_string())
    }

    pub fn is_legal(&self) -> bool { self.path().is_dir() }
}

impl std::fmt::Debug for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cat: {}, pn: {}", self.category(), self.pn())
    }
}
