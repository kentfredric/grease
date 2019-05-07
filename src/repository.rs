//! A Representation of a Gentoo repository
//!
#![cfg_attr(
    feature="external_doc",
    doc(include = "repository/repository.md")
)]

use failure::Fail;
use std::{
    fs,
    io::{self, ErrorKind},
    path::PathBuf,
};

pub mod category;
pub mod package;
pub use category::Category;
pub use package::Package;

/// Class of errors producable by Repositories
#[derive(Fail, Debug)]
#[fail(display = "An error occurred in repository handling")]
pub enum RepositoryError {
    /// A specified path did not appear to exist on the filesystem
    #[fail(display = "Path <{:?}> not found", _1)]
    PathNotFound(#[fail(cause)] io::Error, PathBuf),
    /// A specified path generated IO errors during discovery
    #[fail(display = "Error accessing path <{:?}>", _1)]
    PathAccessError(#[fail(cause)] io::Error, PathBuf),
    /// A specified path was not a directory, but a directory was expected
    #[fail(display = "Path <{:?}> is not a directory", _0)]
    NotADir(PathBuf),
    /// A specified path was not a file, but a file was expected
    #[fail(display = "Path <{:?}> is not a file", _0)]
    NotAFile(PathBuf),
    /// A specified file had IO errors when reading it
    #[fail(display = "Error reading <{:?}>", _1)]
    FileReadError(#[fail(cause)] io::Error, PathBuf),
}

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

    /// Extract this repositories name from its profiles dir
    pub fn name(&self) -> Result<String, RepositoryError> {
        let p = self.root.join("profiles/repo_name");
        fs::read_to_string(&p)
            .map_err(|e| match e.kind() {
                ErrorKind::NotFound => RepositoryError::PathNotFound(e, p),
                _ => RepositoryError::FileReadError(e, p),
            })
            .map(|content| content.trim_end().to_owned())
    }
}

impl AsRef<PathBuf> for Repository {
    fn as_ref(&self) -> &PathBuf { &self.root }
}

impl From<Repository> for PathBuf {
    fn from(other: Repository) -> Self { other.root }
}
impl From<&Repository> for PathBuf {
    fn from(other: &Repository) -> Self { other.root.to_owned() }
}
