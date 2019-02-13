

use std::path;

pub struct Repository {
    root: path::PathBuf,
}

impl Repository {
    pub fn new(root: &path::Path) -> Repository {
        Repository { root: root.to_path_buf() }
    }
}
