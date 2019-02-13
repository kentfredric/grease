use super::ebuild::{self, Ebuild};
use std::ffi::{OsString, OsStr};
use std::io::Error;
use std::path::{Path, PathBuf};
use std::result::Result;

pub struct Package {
    root: PathBuf,
    category: OsString,
    package: OsString,
}
impl Package {
    fn new(root: PathBuf, category: &OsStr, package: &OsStr) -> Package {
        Package {
            root,
            category: category.to_os_string(),
            package: package.to_os_string(),
        }
    }
    pub fn package_path(&self) -> PathBuf { self.category_path().join(&self.package) }
    pub fn category_path(&self) -> PathBuf { self.root.join(&self.category) }

    pub fn category(&self) -> Option<String> { self.category.to_str().map(String::from) }
    pub fn pn(&self) -> Option<String> { self.package.to_str().map(String::from) }

    pub fn ebuilds<'a>(&'a self) -> Result<Box<Iterator<Item = Result<Ebuild, Error>> + 'a>, Error> {
        ebuild::iterator(&self.root, &self.category, &self.package)
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

pub fn package_iterator<'a>(root: &'a Path, category: &'a OsStr)
    -> Result<Box<impl Iterator<Item = Result<Package, Error>> + 'a>, Error> {
    Ok(Box::new(
        root.join(category).read_dir()?
        .filter(move |e| if let Ok(entry) = e {
            entry.path().is_dir()
        } else {
            // readdir entry failures passthrough
            true
        })
        // Munge Ok(), passthru Err()
        .map(move |e| e.map(  |ent|
                         Package::new( root.to_path_buf(), &category, &ent.file_name() ) )),
    ))
}


type PackageIter = Box<Iterator<Item = Result<OsString, Error>>>;
type PackageIterResult = Result<PackageIter, Error>;

fn in_category_dir(category_root: &Path) -> PackageIterResult {
    Ok(Box::new(
        category_root
            .read_dir()?
            .filter(move |e| if let Ok(entry) = e {
                entry.path().is_dir()
            } else {
                // readdir entry failures passthrough
                true
            })

            // Munge Ok(), passthru Err()
            .map(|e| e.map( |ent| ent.file_name() ) ),
    ))
}

pub fn iterator(root: &'static Path, category: &OsStr) -> PackageIterResult { in_category_dir(&root.join(category)) }
