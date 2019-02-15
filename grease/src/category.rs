use super::ebuild::Ebuild;
use super::package::{self, Package};
use std::ffi::OsString;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::io::ErrorKind::NotFound;
use std::path::PathBuf;
use std::result::Result;

pub struct Category {
    root: PathBuf,
    category: OsString,
}

impl Category {
    fn new(root: PathBuf, category: OsString) -> Category { Category { root, category } }

    pub fn path(&self) -> PathBuf { self.root.join(&self.category) }

    pub fn packages(&self) -> Result<Box<Iterator<Item = Result<Package, Error>>>, Error> {
        package::iterator(self.root.to_owned(), self.category.to_owned())
    }
    pub fn ebuilds(&self) -> Result<Box<Iterator<Item = Result<Ebuild, Error>>>, Error> {
        self.packages().map(|pkg_it| {
            Box::new(pkg_it.flat_map(|pkg_res| match pkg_res {
                Ok(pkg) => {
                    match pkg.ebuilds() {
                        Ok(ebuild_iter) => ebuild_iter,
                        Err(e) => Box::new(vec![Err(e)].into_iter()),
                    }
                },
                Err(e) => Box::new(vec![Err(e)].into_iter()),
            })) as Box<Iterator<Item = _>>
        })
    }
}
impl std::fmt::Debug for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "cat: {}",
            self.category.to_str().map(String::from).unwrap_or_else(
                || {
                    String::from("None")
                },
            )
        )
    }
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

fn discover_in(root: PathBuf) -> Result<Box<Iterator<Item = Result<Category, Error>>>, Error> {
    let my_root = root.to_owned();
    Ok(Box::new(
        root.read_dir()?
            .filter(move |e| if let Ok(entry) = e {
                valid_category(
                    root.to_owned(),
                    &entry.to_owned().file_name().into_string().unwrap(),
                )
            } else {
                // Passthrough errors
                true
            })
            .map(move |e| {
                e.map(|ent| Category::new(my_root.to_owned(), ent.file_name()))
            }),
    ))
}

fn read_profile(root: PathBuf) -> Result<Box<Iterator<Item = Result<Category, Error>>>, Error> {
    let my_root = root.to_owned();
    Ok(Box::new(
        BufReader::new(
            File::open(profile_category_file(root.to_owned()))?,
        ).lines()
            .filter(move |line| if let Ok(l) = line {
                root.join(l).is_dir()
            } else {
                true
            })
            .map(move |line_res| {
                line_res.map(|line| {
                    Category::new(my_root.to_owned(), OsString::from(line))
                })
            }),
    ))
}

pub fn iterator(root: PathBuf) -> Result<Box<Iterator<Item = Result<Category, Error>>>, Error> {
    if profile_category_file(root.to_owned()).exists() {
        read_profile(root.to_owned())
    } else {
        discover_in(root.to_owned())
    }
}
pub fn get(root: PathBuf, name: &str) -> Result<Category, Error> {
    if valid_category(root.to_owned(), name) {
        Ok(Category::new(root, OsString::from(name)))
    } else {
        Err(Error::new(
            NotFound,
            "Specified category name was not a directory/not \
             found/illegal",
        ))
    }
}
