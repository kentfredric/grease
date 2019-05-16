//! Representation of a category in a Gentoo repository

use failure::Fail;
use std::{
    borrow::Cow,
    fs,
    io::{self, BufRead},
    path::PathBuf,
};

/// Represents a concrete Gentoo category
#[cfg_attr(
    feature = "external_doc",
    doc(include = "repository/struct.Category.md")
)]
#[derive(Debug, Clone, PartialEq)]
pub struct Category {
    root:     PathBuf,
    category: String,
    path:     PathBuf,
}

impl Category {
    /// Construct a new [`Category`] explicitly
    pub fn new<'a, P, S>(root: P, category: S) -> Self
    where
        P: Into<PathBuf>,
        S: Into<Cow<'a, str>>,
    {
        let r = root.into();
        let c = category.into().into_owned();
        let path = r.join(&c);
        Self { root: r, category: c, path }
    }

    /// Returns the path to this repository
    pub fn path(&self) -> PathBuf { self.path.to_owned() }

    /// Return the name of the category
    pub fn name(&self) -> String { self.category.to_owned() }
}

impl AsRef<PathBuf> for Category {
    fn as_ref(&self) -> &PathBuf { &self.path }
}

impl From<Category> for PathBuf {
    fn from(other: Category) -> Self { other.path }
}
impl From<&Category> for PathBuf {
    fn from(other: &Category) -> Self { other.path.to_owned() }
}

/// Types of errors that can be emitted when creating an instance of
/// [`CategoryFileIterator`]
#[derive(Fail, Debug)]
#[fail(display = "An error occurred iterating a category file")]
pub enum CategoryFileError {
    /// A specified path did not appear to exist on the filesystem
    #[fail(display = "Path <{:?}> not found", _1)]
    PathNotFound(#[fail(cause)] io::Error, PathBuf),
    /// A specified path generated IO errors during discovery
    #[fail(display = "Error accessing path <{:?}>", _1)]
    PathAccessError(#[fail(cause)] io::Error, PathBuf),
    /// A specified path was not a file, but a file was expected
    #[fail(display = "Path <{:?}> is not a file", _0)]
    NotAFile(PathBuf),
    /// An entry in a file encountered decoding errors
    #[fail(display = "Path <{:?}> encountered decoding errors", _1)]
    FileDecodeError(#[fail(cause)] io::Error, PathBuf),
    /// An IO error occurred reading a file
    #[fail(display = "Path <{:?}> encountered IO errors in read", _1)]
    FileReadError(#[fail(cause)] io::Error, PathBuf),
}

/// Iterate a `categories` file in a portage repository
#[cfg_attr(
    feature = "external_doc",
    doc(include = "repository/struct.CategoryFileIterator.md")
)]
#[derive(Debug)]
pub struct CategoryFileIterator {
    root:      PathBuf,
    file:      PathBuf,
    file_iter: io::Lines<io::BufReader<fs::File>>,
}

impl CategoryFileIterator {
    /// Create a [`CategoryFileIterator`] for a given `file` in a repository
    /// `root`
    pub fn for_file<P>(root: P, file: P) -> Result<Self, CategoryFileError>
    where
        P: Into<PathBuf>,
    {
        let my_file = file.into();
        fs::File::open(&my_file)
            .map_err(|e| {
                CategoryFileError::PathAccessError(e, my_file.to_owned())
            })
            .map(|f| Self {
                root:      root.into(),
                file:      my_file,
                file_iter: io::BufReader::new(f).lines(),
            })
    }
}
impl Iterator for CategoryFileIterator {
    type Item = Result<Category, CategoryFileError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.file_iter.next().map(|i| {
            i.map_err(|e| match e.kind() {
                io::ErrorKind::InvalidData => {
                    CategoryFileError::FileDecodeError(
                        e,
                        self.file.to_owned(),
                    )
                },
                _ => {
                    CategoryFileError::FileReadError(e, self.file.to_owned())
                },
            })
            .map(|line| Category::new(&self.root, line))
        })
    }
}
