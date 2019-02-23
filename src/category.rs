use super::{ebuild::Ebuild, package::Package};
use std::{io::Error, path::PathBuf, result::Result};

/// Represents a discrete Gentoo category
pub struct Category {
    root:     PathBuf,
    category: String,
}

impl Category {
    pub fn new(root: PathBuf, category: String) -> Category { Category { root, category } }

    /// Return the path to the category
    pub fn path(&self) -> PathBuf { self.root.join(&self.category) }

    pub fn name(&self) -> String { self.category.to_owned() }

    /// Return an iterator over all packages in this category
    pub fn packages(&self) -> Result<Box<dyn Iterator<Item = Result<Package, Error>>>, Error> {
        let root = self.root.to_owned();
        let category = self.category.to_owned();
        Ok(Box::new(root.join(&category).read_dir()?.map(move |e| {
            e.map(|ent| {
                let ent_fn = ent.file_name();
                Package::new(
                    root.to_owned(),
                    category.to_owned(),
                    ent_fn.to_str().expect("Could not decode filename as UTF8").to_owned(),
                )
            })
        })))
    }

    /// Return an iterator over all ebuilds in this category
    pub fn ebuilds(&self) -> Result<Box<dyn Iterator<Item = Result<Ebuild, Error>>>, Error> {
        self.packages().map(|pkg_it| {
            Box::new(pkg_it.flat_map(|pkg_res| match pkg_res {
                Ok(pkg) => match pkg.ebuilds() {
                    Ok(ebuild_iter) => ebuild_iter,
                    Err(e) => Box::new(vec![Err(e)].into_iter()),
                },
                Err(e) => Box::new(vec![Err(e)].into_iter()),
            })) as Box<dyn Iterator<Item = _>>
        })
    }

    /// Get a package within this category
    pub fn get_package(&self, name: &str) -> Package {
        Package::new(self.root.to_owned(), self.category.to_owned(), name.to_string())
    }

    /// returns if a given category exists or not
    pub fn exists(&self) -> bool { self.path().exists() }

    /// determines if a category has a legal name or not
    pub fn has_legal_name(&self) -> bool {
        match self.category.as_str() {
            "metadata" | "profiles" | "eclass" | ".git" | "distfiles" | "packages" | "scripts" => false,
            _ => true,
        }
    }

    /// Returns if a category is "legal" or not
    ///
    /// This means the category has both a legal name, and its path is a directory
    pub fn is_legal(&self) -> bool { self.has_legal_name() && self.path().is_dir() }

    /// Determines if a category has children
    ///
    /// This is a perfomance hit because it has to invoke readdir on the category
    /// and begins package discovery, but returns true as soon as readdir yeilds a package
    pub fn is_non_empty(&self) -> bool { self.packages().unwrap().any(|x| x.is_ok()) }
}
impl std::fmt::Debug for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "cat: {}", &self.category) }
}
