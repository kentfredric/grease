//! Representation of an ebuild in a Gentoo repository

use std::{borrow::Cow, path::PathBuf};

/// Represent a discrete Gentoo ebuild
#[cfg_attr(
    feature = "external_doc",
    doc(include = "repository/struct.Ebuild.md")
)]
#[derive(Debug, Clone)]
pub struct Ebuild {
    root:     PathBuf,
    category: String,
    package:  String,
    ebuild:   String,
    path:     PathBuf,
}

impl Ebuild {
    /// Construct a new ebuild explicitly
    pub fn new<'a, P, S>(root: P, category: S, package: S, ebuild: S) -> Self
    where
        P: Into<PathBuf>,
        S: Into<Cow<'a, str>>,
    {
        let r = root.into();
        let c = category.into().into_owned();
        let p = package.into().into_owned();
        let e = ebuild.into().into_owned();
        let path = r.join(&c).join(&p).join(&e);
        Self { root: r, category: c, package: p, ebuild: e, path }
    }

    /// Returns a path to the ebuild file
    pub fn path(&self) -> PathBuf { self.path.to_owned() }
}

impl AsRef<PathBuf> for Ebuild {
    fn as_ref(&self) -> &PathBuf { &self.path }
}
