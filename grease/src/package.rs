use super::ebuild::{self, Ebuild};
use std::ffi::OsString;
use std::io::Error;
use std::path::PathBuf;
use std::result::Result;

pub struct Package {
    root: PathBuf,
    category: OsString,
    package: OsString,
}
impl Package {
    fn new(root: PathBuf, category: OsString, package: OsString) -> Package {
        Package {
            root,
            category,
            package,
        }
    }
    pub fn package_path(&self) -> PathBuf { self.category_path().join(&self.package) }
    pub fn category_path(&self) -> PathBuf { self.root.join(&self.category) }

    pub fn category(&self) -> Option<String> { self.category.to_str().map(String::from) }
    pub fn pn(&self) -> Option<String> { self.package.to_str().map(String::from) }

    pub fn ebuilds(&self) -> Result<Box<Iterator<Item = Result<Ebuild, Error>>>, Error> {
        ebuild::iterator(
            self.root.clone(),
            self.category.clone(),
            self.package.clone(),
        )
    }
}

impl std::fmt::Debug for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "cat: {}, pn: {}",
               self.category().unwrap_or_else(||String::from("None")),
               self.pn().unwrap_or_else(||String::from("None")),
        )
    }
}

pub fn iterator(root: PathBuf, category: OsString) -> Result<Box<Iterator<Item = Result<Package, Error>>>, Error> {
    Ok(Box::new(
        root.join(category.clone()).read_dir()?
        .filter(move |e| if let Ok(entry) = e {
            entry.path().is_dir()
        } else {
            // readdir entry failures passthrough
            true
        })
        // Munge Ok(), passthru Err()
        .map( move |e| e.map(  |ent|
                         Package::new( root.to_path_buf(), category.clone(), ent.file_name() ) )),
    ))
}
