use super::version::{self, Version};
use once_cell::sync::OnceCell;
use std::path::PathBuf;

/// Represent a discrete Gentoo ebuild
pub struct Ebuild {
    root:     PathBuf,
    category: String,
    package:  String,
    ebuild:   String,
    version:  OnceCell<Version>,
}

impl Ebuild {
    pub fn new(root: PathBuf, category: String, package: String, ebuild: String) -> Ebuild {
        Ebuild { root, category, package, ebuild, version: OnceCell::INIT }
    }

    /// Returns a path to the ebuild file
    pub fn path(&self) -> PathBuf { self.root.join(&self.category).join(&self.package).join(&self.ebuild) }

    pub fn version(&self) -> &Version {
        self.version.get_or_init(|| {
            version::parse(
                self.ebuild.trim_end_matches(".ebuild").trim_start_matches((self.package.to_owned() + "-").as_str()),
            )
        })
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

    pub fn is_legal(&self) -> bool {
        let p = self.path();
        p.exists() && !p.is_dir()
    }
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
