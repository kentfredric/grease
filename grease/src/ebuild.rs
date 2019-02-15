use super::package;
use std::ffi::OsString;
use std::io::Error;
use std::io::ErrorKind::NotFound;
use std::option::Option;
use std::path::PathBuf;
use std::result::Result;

pub struct Ebuild {
    root: PathBuf,
    category: OsString,
    package: OsString,
    ebuild: OsString,
}

impl Ebuild {
    fn new(root: PathBuf, category: OsString, package: OsString, ebuild: OsString) -> Ebuild {
        Ebuild {
            root,
            category,
            package,
            ebuild,
        }
    }

    pub fn path(&self) -> PathBuf {
        self.root.join(&self.category).join(&self.package).join(
            &self.ebuild,
        )
    }

    pub fn category(&self) -> Option<String> { self.category.to_str().map(String::from) }
    pub fn pn(&self) -> Option<String> { self.package.to_str().map(String::from) }
    pub fn pf(&self) -> Option<String> {
        self.path().file_stem().and_then(|osstr| {
            osstr.to_str().map(String::from)
        })
    }
    pub fn pvr(&self) -> Option<String> {
        self.pf().and_then(|pf| {
            self.pn().and_then(|pn| {
                let suffix = pf.trim_start_matches((pn + "-").as_str());
                if suffix == pf {
                    None
                } else {
                    Some(String::from(suffix))
                }
            })
        })
    }
    pub fn pv(&self) -> Option<String> {
        self.pvr().map(|pvr| {
            let mut chunks: Vec<_> = pvr.split_terminator("-r").collect();
            // This finds the trailing block of PVR which *might* be "-r" + a series of
            // digits if such a block exists, return PVR without it otherwise, perform no
            // changes and return PVR as PV
            if chunks.len() < 2 {
                return pvr;
            }
            match chunks.pop() {
                None => pvr,
                Some(rversion) => {
                    match rversion.parse::<u32>() {
                        Ok(_) => chunks.join("-"),
                        Err(_) => pvr,
                    }
                },
            }
        })
    }
    pub fn p(&self) -> Option<String> {
        self.pn().and_then(
            |pn| self.pv().and_then(|pv| Some(pn + &pv)),
        )
    }
}

impl std::fmt::Debug for Ebuild {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let none_str = || String::from("None");
        write!(f, "cat: {}, pf: {}, pn: {}, pvr: {} pv: {}",
               self.category().unwrap_or_else(none_str),
               self.pf().unwrap_or_else(none_str),
               self.pn().unwrap_or_else(none_str),
               self.pvr().unwrap_or_else(none_str),
               self.pv().unwrap_or_else(none_str),
        )
    }
}

pub fn iterator(root: PathBuf, category: OsString, package: OsString)
    -> Result<Box<Iterator<Item = Result<Ebuild, Error>>>, Error> {
    Ok(Box::new(
        root.join(&category)
            .join(&package)
            .read_dir()?
            .filter(|e| if let Ok(entry) = e {
                let p = entry.path();
                if let Some(ext) = p.extension() {
                    ext.eq("ebuild") && !p.is_dir()
                } else {
                    false
                }
            } else {
                true
            })
            .map(move |dirent| {
                dirent.map(|entry| {
                    Ebuild::new(
                        root.to_owned(),
                        category.to_owned(),
                        package.to_owned(),
                        entry.file_name(),
                    )
                })
            }),
    ))
}

pub fn get(root: PathBuf, category: &str, package: &str, ebuild: &str) -> Result<Ebuild, Error> {
    let my_root = root.to_owned();
    package::get(root, category, package).and_then(|pkg| {
        let ebuild_path = pkg.path().join(ebuild);
        if ebuild_path.exists() && !ebuild_path.is_dir() {
            Ok(Ebuild::new(
                my_root.to_owned(),
                OsString::from(category),
                OsString::from(package),
                OsString::from(ebuild),
            ))
        } else {
            Err(Error::new(NotFound, "Ebuild not found/ is a directory"))
        }
    })
}
