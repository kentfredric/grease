use super::category::{self, Category};
use std::io::Error;
use std::path::{Path, PathBuf};
use std::result::Result;

pub struct Repository {
    root: PathBuf,
}

impl Repository {
    pub fn new(root: &Path) -> Repository { Repository { root: root.to_path_buf() } }

    pub fn categories<'a>(&'a self) -> Result<Box<Iterator<Item = Result<Category, Error>> + 'a>, Error> {
        category::iterator(&self.root)
    }
}
