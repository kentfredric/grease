//! Representation of a package in a Gentoo repository
use std::{borrow::Cow, path::PathBuf};

/// Represents a discrete gentoo package
#[cfg_attr(
    feature = "external_doc",
    doc(include = "repository/struct.Package.md")
)]
#[derive(Debug, Clone)]
pub struct Package {
    root:     PathBuf,
    category: String,
    package:  String,
    path:     PathBuf,
}

impl Package {
    /// Construct a new Package Type Object
    pub fn new<'a, P, S>(root: P, category: S, package: S) -> Self
    where
        P: Into<PathBuf>,
        S: Into<Cow<'a, str>>,
    {
        let r = root.into();
        let c = category.into().into_owned();
        let p = package.into().into_owned();
        let path = r.join(&c).join(&p);
        Self { root: r, category: c, package: p, path }
    }

    /// Return the path to a gentoo package
    pub fn path(&self) -> PathBuf { self.path.to_owned() }
}
