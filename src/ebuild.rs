use super::{
    package,
    version::{self, Version},
};
use once_cell::sync::OnceCell;
use std::{
    io::{Error, ErrorKind::NotFound},
    path::PathBuf,
    result::Result,
};

/// Represent a discrete Gentoo ebuild
pub struct Ebuild {
    root:     PathBuf,
    category: String,
    package:  String,
    ebuild:   String,
    version:  OnceCell<Version>,
}

impl Ebuild {
    fn new(root: PathBuf, category: String, package: String, ebuild: String) -> Ebuild {
        Ebuild { root, category, package, ebuild, version: OnceCell::INIT }
    }

    /// Returns a path to the ebuild file
    pub fn path(&self) -> PathBuf { self.root.join(&self.category).join(&self.package).join(&self.ebuild) }

    pub fn version(&self) -> &Version {
        self.version.get_or_init(|| version::parse(&ebuild_to_pvr(self.package.to_owned(), self.ebuild.to_owned())))
    }

    pub fn name(&self) -> String { self.category.to_owned() + "/" + &self.package + "/" + &self.ebuild }

    /// Returns the ebuilds category similar to `PMS` variable `CATEGORY`
    pub fn category(&self) -> String { self.category.to_owned() }

    /// Returns the ebuilds package name similar to `PMS` variable `PN`
    pub fn pn(&self) -> String { self.package.to_owned() }

    /// Returns the ebuilds full package version similar to `PMS` variable `PF`
    pub fn pf(&self) -> String {
        String::from(
            self.path()
                .file_stem()
                .expect("Could not extract file stem from file")
                .to_str()
                .expect("Could not decode UTF8"),
        )
    }

    /// Returns the ebuilds version with revision similar to `PMS` variable
    /// `PVR`
    pub fn pvr(&self) -> String { self.version().pvr() }

    /// Returns the ebuilds version without revision similar to `PMS` variable
    /// `PV`
    pub fn pv(&self) -> &str { self.version().pv() }

    /// Returns the ebuilds revision, or r0, similar to `PMS` variable `PR`
    pub fn pr(&self) -> &str { self.version().pr() }

    /// Returns the ebuilds package name without revision, similar to `PMS`
    /// variable `P`
    pub fn p(&self) -> String { self.pn() + self.pv() }
}

impl std::fmt::Debug for Ebuild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "cat: {}, pf: {}, pn: {}, pvr: {} pv: {} pr: {}",
            self.category(),
            self.pf(),
            self.pn(),
            self.pvr(),
            self.pv(),
            self.pr(),
        )
    }
}

fn ebuild_to_pvr(package: String, ebuild: String) -> String {
    let pf = ebuild.trim_end_matches(".ebuild");
    pf.trim_start_matches((package + "-").as_str()).to_owned()
}

/// Iterate all ebuilds within a package
pub fn iterator(
    root: PathBuf, category: String, package: String,
) -> Result<Box<dyn Iterator<Item = Result<Ebuild, Error>>>, Error> {
    Ok(Box::new(
        root.join(&category)
            .join(&package)
            .read_dir()?
            .filter(|e| {
                if let Ok(entry) = e {
                    let p = entry.path();
                    if let Some(ext) = p.extension() {
                        ext.eq("ebuild") && !p.is_dir()
                    } else {
                        false
                    }
                } else {
                    true
                }
            })
            .map(move |dirent| {
                dirent.map(|entry| {
                    let e_fn = entry.file_name();
                    let e = e_fn.to_str().expect("Could not decode filename to UTF8");
                    Ebuild::new(root.to_owned(), category.to_owned(), package.to_owned(), e.to_owned())
                })
            }),
    ))
}

/// Get a validated Ebuild object by explicit path
pub fn get(root: PathBuf, category: &str, package: &str, ebuild: &str) -> Result<Ebuild, Error> {
    let my_root = root.to_owned();
    package::get(root, category, package).and_then(|pkg| {
        let ebuild_path = pkg.path().join(ebuild);
        if ebuild_path.exists() && !ebuild_path.is_dir() {
            Ok(Ebuild::new(my_root.to_owned(), category.to_owned(), package.to_owned(), ebuild.to_owned()))
        } else {
            Err(Error::new(NotFound, "Ebuild not found/ is a directory"))
        }
    })
}
