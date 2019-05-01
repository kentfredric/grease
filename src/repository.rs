//! A Representation of a Gentoo repository
//!
#![cfg_attr(
    feature="external_doc",
    doc(include = "repository/repository.md")
)]

use std::path::PathBuf;

/// Represents a gentoo repository
#[cfg_attr(
    feature = "external_doc",
    doc(include = "repository/struct.Repository.md")
)]
#[derive(Debug, Clone)]
pub struct Repository {
    root: PathBuf,
}

impl Repository {
    /// Construct a new Repository trait object
    pub fn new<P>(root: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self { root: root.into() }
    }

    /// Returns the path to this repository
    pub fn path(&self) -> PathBuf { self.root.to_owned() }
}
