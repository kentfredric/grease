use std::ffi::{OsString, OsStr};
use std::io::Error;
use std::option::Option;
use std::path::{PathBuf, Path};
use std::result::Result;

pub struct Ebuild {
    root: PathBuf,
    category: OsString,
    package: OsString,
    ebuild: OsString,
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
    pub fn ebuild_path(&self) -> PathBuf { self.package_path().join(&self.ebuild) }
    pub fn package_path(&self) -> PathBuf { self.category_path().join(&self.package) }
    pub fn category_path(&self) -> PathBuf { self.root.join(&self.category) }

    pub fn category(&self) -> Option<String> { self.category.to_str().map(String::from) }
    pub fn pn(&self) -> Option<String> { self.package.to_str().map(String::from) }
    pub fn pf(&self) -> Option<String> {
        self.ebuild_path().file_stem().and_then(|osstr| {
            osstr.to_str().map(|str| String::from(str))
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
        write!(f, "cat: {}, pf: {}, pn: {}, pvr: {} pv: {}",
               self.category().unwrap_or_else(||String::from("None")),
               self.pf().unwrap_or_else(||String::from("None")),
               self.pn().unwrap_or_else(||String::from("None")),
               self.pvr().unwrap_or_else(||String::from("None")),
               self.pv().unwrap_or_else(||String::from("None")),
        )
    }
}

pub fn iterator<'a>(root: &'a Path, category: &'a OsStr, package: &'a OsStr)
    -> Result<Box<Iterator<Item = Result<Ebuild, Error>> + 'a>, Error> {
    let eroot = &root.join(&category).join(&package);
    Ok(Box::new(
        eroot
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
                    Ebuild::new(root.to_path_buf(), category, package, &entry.file_name())
                })
            }),
    ))
}
