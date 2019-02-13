use std::path::{PathBuf, Path};

pub struct Repository {
    root: PathBuf,
}

impl Repository {
    pub fn new(root: &Path) -> Repository { Repository { root: root.to_path_buf() } }
}
