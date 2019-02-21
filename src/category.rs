use super::{
    ebuild::Ebuild,
    package::{self, Package},
    util::optfilter::OptFilter,
};
use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind::NotFound},
    path::PathBuf,
    result::Result,
};

/// Represents a discrete Gentoo category
pub struct Category {
    root:     PathBuf,
    category: String,
}

impl Category {
    fn new(root: PathBuf, category: String) -> Category { Category { root, category } }

    /// Return the path to the category
    pub fn path(&self) -> PathBuf { self.root.join(&self.category) }

    pub fn name(&self) -> String { self.category.to_owned() }

    /// Return an iterator over all packages in this category
    pub fn packages(&self) -> Result<Box<dyn Iterator<Item = Result<Package, Error>>>, Error> {
        package::iterator(self.root.to_owned(), self.category.to_owned())
    }

    /// Return an iterator over all ebuilds in this category
    pub fn ebuilds(&self) -> Result<Box<dyn Iterator<Item = Result<Ebuild, Error>>>, Error> {
        self.packages().map(|pkg_it| {
            Box::new(pkg_it.flat_map(|pkg_res| match pkg_res {
                Ok(pkg) => match pkg.ebuilds() {
                    Ok(ebuild_iter) => ebuild_iter,
                    Err(e) => Box::new(vec![Err(e)].into_iter()),
                },
                Err(e) => Box::new(vec![Err(e)].into_iter()),
            })) as Box<dyn Iterator<Item = _>>
        })
    }

    /// Get a validated package within this category
    pub fn get_package(&self, name: &str) -> Result<Package, Error> {
        let c = self.category.to_owned();
        package::get(self.root.to_owned(), &c, name)
    }

    /// returns if a given category exists or not
    pub fn exists(&self) -> bool { self.path().exists() }

    /// determines if a category has a legal name or not
    pub fn has_legal_name(&self) -> bool {
        match self.category.as_str() {
            "metadata" | "profiles" | "eclass" | ".git" | "distfiles" | "packages" | "scripts" => false,
            _ => true,
        }
    }

    /// Returns if a category is "legal" or not
    ///
    /// This means the category has both a legal name, and its path is a directory
    pub fn is_legal(&self) -> bool { self.has_legal_name() && self.path().is_dir() }

    /// Determines if a category has children
    ///
    /// This is a perfomance hit because it has to invoke readdir on the category
    /// and begins package discovery, but returns true as soon as readdir yeilds a package
    pub fn is_non_empty(&self) -> bool { self.packages().unwrap().any(|x| x.is_ok()) }
}
impl std::fmt::Debug for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "cat: {}", &self.category) }
}

#[inline]
fn dirname_blacklisted(name: &str) -> bool {
    match name {
        "metadata" | "profiles" | "eclass" | ".git" | "distfiles" | "packages" | "scripts" => true,
        _ => false,
    }
}

#[inline]
fn valid_category(root: PathBuf, name: &str) -> bool { !dirname_blacklisted(name) && root.join(name).is_dir() }

#[inline]
fn profile_category_file(root: PathBuf) -> PathBuf { root.join("profiles").join("categories") }

fn discover_in(root: PathBuf) -> Result<Box<dyn Iterator<Item = Result<Category, Error>>>, Error> {
    let my_root = root.to_owned();
    Ok(Box::new(
        root.read_dir()?
            .map(move |e| e.map(|ent| Category::new(my_root.to_owned(), ent.file_name().to_str().unwrap().to_owned())))
            .filter_oks(self::Category::has_legal_name),
    ))
}

fn read_profile(root: PathBuf) -> Result<Box<dyn Iterator<Item = Result<Category, Error>>>, Error> {
    let my_root = root.to_owned();
    Ok(Box::new(
        BufReader::new(File::open(profile_category_file(root.to_owned()))?)
            .lines()
            .map(move |line_res| line_res.map(|line| Category::new(my_root.to_owned(), line)))
    ))
}

/// Return an iterator over all categories in the given root
pub fn iterator(root: PathBuf) -> Result<Box<dyn Iterator<Item = Result<Category, Error>>>, Error> {
    if profile_category_file(root.to_owned()).exists() {
        read_profile(root.to_owned())
    } else {
        discover_in(root.to_owned())
    }
}

/// Get a validated category from the root
pub fn get(root: PathBuf, name: &str) -> Result<Category, Error> {
    if valid_category(root.to_owned(), name) {
        Ok(Category::new(root, name.to_owned()))
    } else {
        Err(Error::new(NotFound, "Specified category name was not a directory/not found/illegal"))
    }
}
