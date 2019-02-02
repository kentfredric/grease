

use std::path;
use std::result;
use std::io;
use std::ffi;

use super::category;

pub struct Repository {
    root: path::PathBuf,
}

type CategoryIter = Box<Iterator<Item = result::Result<ffi::OsString, io::Error>>>;
type CategoryIterResult = result::Result<CategoryIter, io::Error>;

impl Repository {
    pub fn new(root: &path::Path) -> Repository {
        Repository { root: root.to_path_buf() }
    }
    pub fn cateories(&'static self) -> CategoryIterResult {
        category::iterator(&self.root)
    }
}
