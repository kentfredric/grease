//! Representation of a category in a Gentoo repository

use failure::Fail;
use libc;
use std::{
    borrow::{Cow, ToOwned},
    ffi, fs,
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

/// Types of errors that can be emitted when creating an instance of
/// [`CategoryDirsIterator`]
#[derive(Fail, Debug)]
#[fail(display = "An error occurred iterating directories in a repository")]
pub enum CategoryDirsError {
    /// A specified repository root was not found
    #[fail(display = "Repository Root <{:?}> does not exist", _1)]
    RepoNotFound(#[cause] io::Error, PathBuf),
    /// An IO error occurred accessing the repository root
    #[fail(display = "Error accessing repository root <{:?}>", _1)]
    RepoAccessError(#[fail(cause)] io::Error, PathBuf),
    /// A specified repository root was not a dir, but a dir was expected
    #[fail(display = "Repository Root <{:?}> is not a dir", _0)]
    RepoNotADir(PathBuf),
    /// An error occurred in invoking readdir()
    #[fail(
        display = "Repository Root <{:?}> encountered an error in readdir()",
        _1
    )]
    RepoReadDirError(#[fail(cause)] io::Error, PathBuf),
    /// An error occurred decoding a filename to Unicode
    #[fail(display = "Category name <{:?}> did not decode as Unicode", _0)]
    CategoryNameDecodeError(ffi::OsString),
    /// A discovered category in root was not found
    #[fail(display = "Discovered Category <{:?}> does not exist", _1)]
    CategoryNotFound(#[cause] io::Error, String),
    /// A discovered category in root had IO Access Errors
    #[fail(
        display = "Discovered Category <{:?}> had errors while accessing it",
        _1
    )]
    CategoryAccessError(#[cause] io::Error, String),

    /// Weirdly, the category iterator passed a PathBuf for category_path
    /// that lacked a tail component
    #[fail(
        display = "Category <{:?}> passed for validation lacked a base \
                   filename part",
        _0
    )]
    CategoryPathLacksFileName(PathBuf),
    /// A discovered cat/pn file had IO Access Errors
    #[fail(
        display = "Discovered package <{:?}> had errors while accessing it",
        _1
    )]
    PackageAccessError(#[cause] io::Error, PathBuf),
    /// A discovered cat/pn file had IO Access Errors
    #[fail(
        display = "Discovered file <{:?}> had errors while accessing it",
        _1
    )]
    EbuildAccessError(#[cause] io::Error, PathBuf),
    /// An error with decoding a file within a cat/pn dir
    #[fail(
        display = "Discovered file name <{:?}> did not decode as Unicode",
        _0
    )]
    EbuildNameDecodeError(ffi::OsString),

    /// An error occurred in invoking readdir() on a category
    #[fail(
        display = "Repository Category <{:?}> encountered an error in \
                   readdir()",
        _1
    )]
    CategoryReadDirError(#[fail(cause)] io::Error, PathBuf),
}
/// Kinds of match rules for category validation
#[derive(Debug, Clone, Copy)]
pub enum CategoryMatcher {
    /// A matcher rule for categories that excludes members from a
    /// predetermined hardcoded set
    ExcludeBlackListed(),
    /// A matcher rule for categories that excludes members if the category
    /// does not have a subdirectory containing at least one .ebuild file
    RequireEbuild(),
}

impl CategoryMatcher {
    /// Apply the given matcher against a category at path `category_path`
    /// and returns a [`Some`] for valid entries and
    /// errors, and returns a [`None`] for paths that are excluded
    pub fn select<P>(
        self, category_path: P,
    ) -> Result<bool, CategoryDirsError>
    where
        P: Into<PathBuf> + std::fmt::Debug,
        Self: Copy,
    {
        match self {
            CategoryMatcher::ExcludeBlackListed() => {
                Self::match_blacklisted(&category_path.into())
            },
            CategoryMatcher::RequireEbuild() => {
                Self::match_contains_ebuild(&category_path.into())
            },
        }
    }

    fn get_category_name(
        category_path: &PathBuf,
    ) -> Result<String, CategoryDirsError> {
        category_path
            .file_name()
            .ok_or_else(|| {
                CategoryDirsError::CategoryPathLacksFileName(
                    category_path.to_owned(),
                )
            })
            .and_then(|f| {
                f.to_str()
                    .ok_or_else(|| {
                        CategoryDirsError::CategoryNameDecodeError(
                            f.to_owned(),
                        )
                    })
                    .map(ToOwned::to_owned)
            })
    }

    fn match_blacklisted(
        category_path: &PathBuf,
    ) -> Result<bool, CategoryDirsError> {
        let category_name = match Self::get_category_name(category_path) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };
        match category_name.as_str() {
            ".git" | "distfiles" | "eclass" | "licenses" | "local"
            | "metadata" | "packages" | "profiles" | "scripts" => Ok(false),
            _ => Ok(true),
        }
    }

    fn path_is_ebuild(
        path: &fs::DirEntry,
    ) -> Result<bool, CategoryDirsError> {
        match path.file_name().to_str() {
            None => Err(CategoryDirsError::EbuildNameDecodeError(
                path.file_name(),
            )),
            Some(s) if s.ends_with(".ebuild") => match path.metadata() {
                Err(e) => match e.kind() {
                    io::ErrorKind::NotFound => Ok(false),
                    _ => Err(CategoryDirsError::EbuildAccessError(
                        e,
                        path.path(),
                    )),
                },
                Ok(fmeta) => Ok(!fmeta.is_dir()),
            },
            _ => Ok(false),
        }
    }

    fn dir_contains_ebuild(
        path: &fs::DirEntry,
    ) -> Result<bool, CategoryDirsError> {
        let package_iterator = match path.path().read_dir() {
            Ok(iter) => iter,
            Err(e) => {
                return match (e.kind(), e.raw_os_error()) {
                    (io::ErrorKind::NotFound, _)
                    | (io::ErrorKind::Other, Some(libc::ENOTDIR)) => {
                        Ok(false)
                    },
                    _ => Err(CategoryDirsError::PackageAccessError(
                        e,
                        path.path(),
                    )),
                }
            },
        };
        for file_node in package_iterator {
            let file_path = file_node.map_err(|e| {
                CategoryDirsError::CategoryReadDirError(e, path.path())
            })?;

            if Self::path_is_ebuild(&file_path)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn match_contains_ebuild(
        category_path: &PathBuf,
    ) -> Result<bool, CategoryDirsError> {
        let category_iterator = match category_path.read_dir() {
            Ok(iter) => iter,
            Err(e) => {
                return match (e.kind(), e.raw_os_error()) {
                    (io::ErrorKind::NotFound, _)
                    | (io::ErrorKind::Other, Some(libc::ENOTDIR)) => {
                        Ok(false)
                    },
                    _ => Err(CategoryDirsError::CategoryAccessError(
                        e,
                        Self::get_category_name(category_path)
                            .unwrap_or_else(|_| {
                                "{errors occurred decoding category name}"
                                    .to_owned()
                            }),
                    )),
                }
            },
        };
        for package_node in category_iterator {
            let package_path = package_node.map_err(|e| {
                CategoryDirsError::CategoryReadDirError(
                    e,
                    category_path.to_owned(),
                )
            })?;
            if Self::dir_contains_ebuild(&package_path)? {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

/// Iterate categories in a portage repository via discovery
#[cfg_attr(
    feature = "external_doc",
    doc(include = "repository/struct.CategoryDirsIterator.md")
)]
#[derive(Debug)]
pub struct CategoryDirsIterator {
    root:     PathBuf,
    dir_iter: fs::ReadDir,
    matchers: Vec<CategoryMatcher>,
}

impl CategoryDirsIterator {
    /// Construct a discovery based category iterator for a repository
    pub fn for_repo<P>(root: P) -> Result<Self, CategoryDirsError>
    where
        P: Into<PathBuf>,
    {
        let my_root = root.into();
        my_root
            .read_dir()
            .map_err(|e| {
                CategoryDirsError::RepoAccessError(e, my_root.to_owned())
            })
            .map(|i| Self {
                root:     my_root,
                dir_iter: i,
                matchers: vec![
                    CategoryMatcher::ExcludeBlackListed(),
                    CategoryMatcher::RequireEbuild(),
                ],
            })
    }
}

impl Iterator for CategoryDirsIterator {
    type Item = Result<Category, CategoryDirsError>;

    fn next(&mut self) -> Option<Self::Item> {
        // This is needlessly complicated due to the recursive self.next()
        // call, which of course returns Option<>, so any
        // destructuring functions like .map() or .map_err() become
        // unusable as we don't want Some(Some(...))
        // or Some(Err(Some(Ok))) etc, etc.
        match self.dir_iter.next() {
            None => None,
            Some(Err(e)) => Some(Err(CategoryDirsError::RepoReadDirError(
                e,
                self.root.to_owned(),
            ))),
            Some(Ok(item)) => {
                let first = self
                    .matchers
                    .iter()
                    .map(|matcher| matcher.select(&item.path()))
                    .find(|matched| match matched {
                        // first false match or err aborts
                        Ok(false) | Err(_) => true,
                        // Good matches progress to next matcher
                        _ => false,
                    });

                match first {
                    Some(Ok(boolstate)) => {
                        // either an Ok(false), an Err()
                        debug_assert!(!boolstate);
                        self.next()
                    },
                    Some(Err(e)) => Some(Err(e)),
                    None => {
                        // either no matchers or all matchers passed
                        match item.file_name().to_str() {
                            None => unimplemented!(),
                            Some(s) => Some(Ok(Category::new(&self.root, s))),
                        }
                    },
                }
            },
        }
    }
}
