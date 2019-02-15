use super::category::{self, Category};
use super::package::Package;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::result::Result;

pub struct Repository {
    root: PathBuf,
}

impl Repository {
    pub fn new(root: &Path) -> Repository { Repository { root: root.to_path_buf() } }

    pub fn categories(&self) -> Result<Box<Iterator<Item = Result<Category, Error>>>, Error> {
        category::iterator(self.root.clone())
    }

    pub fn packages(&self) -> Result<Box<Iterator<Item = Result<Package, Error>>>, Error> {
        self.categories().map(|cat_it| {
            Box::new(cat_it.flat_map(|cat_res| match cat_res {
                Ok(cat) => {
                    match cat.packages() {
                        Ok(package_iter) => package_iter,
                        Err(e) => Box::new(vec![Err(e)].into_iter()),
                    }
                },
                Err(e) => Box::new(vec![Err(e)].into_iter()),
            })) as Box<Iterator<Item = _>>
        })
    }
}
