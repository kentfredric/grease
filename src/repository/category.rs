//! Representation of a category in a Gentoo repository

use std::{borrow::Cow, path::PathBuf};

/// Represents a concrete Gentoo category
#[cfg_attr(
    feature = "external_doc",
    doc(include = "repository/struct.Category.md")
)]
#[derive(Debug, Clone)]
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
