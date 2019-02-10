
use std::path::{PathBuf, Path};
use std::result::Result;
use std::io::Error;
use std::ffi::{OsString, OsStr};

type EbuildIter = Box<Iterator<Item = Result<OsString, Error>>>;
type EbuildIterResult = Result<EbuildIter, Error>;

#[derive(Debug)]
pub struct Ebuild {
    pub root: PathBuf,
    pub category: OsString,
    pub package: OsString,
    pub ebuild: OsString,
}

impl Ebuild {
    fn new(root: PathBuf, category: &OsStr, package: &OsStr, ebuild: &OsStr) -> Ebuild {
        Ebuild {
            root,
            category: category.to_os_string(),
            package: package.to_os_string(),
            ebuild: ebuild.to_os_string(),
        }
    }
    pub fn ebuild_path(&self) -> PathBuf {
        self.package_path().join(&self.ebuild)
    }
    pub fn package_path(&self) -> PathBuf {
        self.category_path().join(&self.package)
    }
    pub fn category_path(&self) -> PathBuf {
        self.root.join(&self.category)
    }
}

fn in_package_dir(ebuild_root: &Path) -> EbuildIterResult {
    Ok(Box::new(
        ebuild_root
            .read_dir()?
            .filter(move |e| if let Ok(entry) = e {
                let p = entry.path();
                if let Some(ext) = p.extension() {
                    ext.eq("ebuild") && !p.is_dir()
                } else {
                    // Files with no extension are not ebuilds
                    // nor are they errors
                    false
                }
            } else {
                // read_dir failures should be passed through the iterator
                true
            })
            .map(|e|
                 // Convert Ok() into Result(OsString)
                 // but leave Err() as-is
                 e.map(|ent| ent.file_name())),
    ))
}

pub fn iterator(root: &Path, category: &OsStr, package: &OsStr) -> EbuildIterResult {
    in_package_dir(&root.join(category).join(package))
}

pub fn ebuild_iterator(
    root: &'static Path,
    category: &'static OsStr,
    package: &'static OsStr,
) -> Result<Box<Iterator<Item = Result<Ebuild, Error>>>, Error> {
    let eroot = &root.join(&category).join(&package);
    Ok(Box::new(
        eroot
            .read_dir()?
            .filter(move |e| if let Ok(entry) = e {
                let p = entry.path();
                if let Some(ext) = p.extension() {
                    ext.eq("ebuild") && !p.is_dir()
                } else {
                    false
                }
            } else {
                true
            })
            .map(move |dirent| match dirent {
                Ok(entry) => Ok(Ebuild::new(
                    root.to_path_buf(),
                    category,
                    package,
                    &entry.file_name(),
                )),
                Err(err) => Err(err),
            }),
    ))
}
