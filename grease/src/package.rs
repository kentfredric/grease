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

    pub fn path(&self) -> PathBuf { self.root.join(&self.category).join(&self.package) }

    pub fn category(&self) -> Option<String> { self.category.to_str().map(String::from) }
    pub fn pn(&self) -> Option<String> { self.package.to_str().map(String::from) }

    pub fn ebuilds(&self) -> Result<Box<Iterator<Item = Result<Ebuild, Error>>>, Error> {
        ebuild::iterator(
            self.root.to_owned(),
            self.category.to_owned(),
            self.package.to_owned(),
        )
    }
}

impl std::fmt::Debug for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let none_str = || String::from("None");
        write!(f, "cat: {}, pn: {}",
               self.category().unwrap_or_else(none_str),
               self.pn().unwrap_or_else(none_str),
        )
    }
}

pub fn iterator(root: PathBuf, category: OsString) -> Result<Box<Iterator<Item = Result<Package, Error>>>, Error> {
    Ok(Box::new(
        root.join(&category).read_dir()?
        .filter(move |e| if let Ok(entry) = e {
            entry.path().is_dir()
        } else {
            // readdir entry failures passthrough
            true
        })
        // Munge Ok(), passthru Err()
        .map( move |e| e.map(  |ent|
                         Package::new( root.to_owned(), category.to_owned(), ent.file_name() ) )),
    ))
}
