use super::category::{self, Category};
use super::ebuild::Ebuild;
use super::package::Package;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::result::Result;

/// Represents a gentoo repository
pub struct Repository {
    root: PathBuf,
}

impl Repository {
    pub fn new(root: &Path) -> Repository { Repository { root: root.to_path_buf() } }

    /// Returns the path to this repository
    pub fn path(&self) -> PathBuf { self.root.to_owned() }

    /// Returns an iterator over all categories in this repository
    pub fn categories(&self) -> Result<Box<dyn Iterator<Item = Result<Category, Error>>>, Error> {
        category::iterator(self.root.to_owned())
    }

    /// Returns an iterator over all packages in this repository
    pub fn packages(&self) -> Result<Box<dyn Iterator<Item = Result<Package, Error>>>, Error> {
        self.categories().map(|cat_it| {
            Box::new(cat_it.flat_map(|cat_res| match cat_res {
                Ok(cat) => {
                    match cat.packages() {
                        Ok(package_iter) => package_iter,
                        Err(e) => Box::new(vec![Err(e)].into_iter()),
                    }
                },
                Err(e) => Box::new(vec![Err(e)].into_iter()),
            })) as Box<dyn Iterator<Item = _>>
        })
    }

    /// Returns an iterator over all ebuilds in this repository
    pub fn ebuilds(&self) -> Result<Box<dyn Iterator<Item = Result<Ebuild, Error>>>, Error> {
        self.packages().map(|pkg_it| {
            Box::new(pkg_it.flat_map(|pkg_res| match pkg_res {
                Ok(pkg) => {
                    match pkg.ebuilds() {
                        Ok(ebuild_iter) => ebuild_iter,
                        Err(e) => Box::new(vec![Err(e)].into_iter()),
                    }
                },
                Err(e) => Box::new(vec![Err(e)].into_iter()),
            })) as Box<dyn Iterator<Item = _>>
        })
    }

    /// Fetch a validated category by name in this repository
    pub fn get_category(&self, name: &str) -> Result<Category, Error> { category::get(self.root.to_owned(), name) }
}
