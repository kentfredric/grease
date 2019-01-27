
use std::path;
use std::fs;
use std::result;
use std::io;

use std::io::BufRead;

pub struct Repository {
    root: path::PathBuf,
}
type CategoryIter = Box<Iterator<Item = String>>;
type CategoryIterResult = result::Result<CategoryIter, io::Error>;

impl Repository {
    pub fn new(root: &path::Path) -> Repository {
        Repository { root: root.to_owned() }
    }

    pub fn categories(&self) -> CategoryIterResult {
        match self.has_category_file() {
            true => self.categories_from_profile(),
            false => self.categories_discover(),
        }
    }
    fn categories_discover(&self) -> CategoryIterResult {
        let dir_fh = self.root.read_dir()?;

        Ok(Box::new(
            dir_fh
                .filter(|entry_result| if let Ok(entry) = entry_result {
                    match entry.file_name().to_str() {
                        Some("metadata") |
                        Some("profiles") |
                        Some("eclass") |
                        Some(".git") |
                        Some("distfiles") |
                        Some("packages") |
                        Some("scripts") => false,
                        _ => entry.path().is_dir(),
                    }
                } else {
                    true
                })
                .map(|e| e.unwrap().file_name().to_str().unwrap().to_owned()),
        ))
    }
    fn categories_from_profile(&self) -> CategoryIterResult {
        let fh = fs::File::open(self.category_file())?;
        let buf = io::BufReader::new(fh);
        let root = self.root.clone();
        let iter = buf.lines().map(|line| line.unwrap()).filter(
            move |line| -> bool {
                root.join(line).is_dir()
            },
        );
        Ok(Box::new(iter))
    }

    fn category_file(&self) -> path::PathBuf {
        self.root.join("profiles").join("categories")
    }
    fn has_category_file(&self) -> bool {
        let meta = fs::metadata(self.category_file());
        if let Ok(metadata) = meta {
            !metadata.is_dir()
        } else {
            false
        }
    }

    fn category_exists(&self, category: String) -> bool {
        let meta = fs::metadata(self.root.join(category));
        if let Ok(metadata) = meta {
            metadata.is_dir()
        } else {
            false
        }
    }
}
