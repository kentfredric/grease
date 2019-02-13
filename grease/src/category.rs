use super::package::{self, Package};
use std::ffi::{OsString, OsStr};
use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::path::{Path, PathBuf};
use std::result::Result;

pub struct Category {
    root: PathBuf,
    category: OsString,
}

impl Category {
    fn new(root: PathBuf, category: &OsStr) -> Category {
        Category {
            root,
            category: category.to_os_string(),
        }
    }
    pub fn packages<'a>(&'a self) -> Result<Box<impl Iterator<Item = Result<Package, Error>> + 'a>, Error> {
        package::iterator(&self.root, &self.category)
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
fn valid_category(root: &Path, name: &str) -> bool {
    if dirname_blacklisted(name) {
        return false;
    }
    root.join(name).is_dir()
}

#[inline]
fn profile_category_file(root: &Path) -> PathBuf { root.join("profiles").join("categories") }

fn discover_in<'a>(root: &'a Path) -> Result<Box<Iterator<Item = Result<Category, Error>> + 'a>, Error> {
    Ok(Box::new(
        root.read_dir()?
            .filter(move |e| if let Ok(entry) = e {
                valid_category(root, &entry.file_name().into_string().unwrap())
            } else {
                // Passthrough errors
                true
            })
            .map(move |e| {
                e.map(|ent| Category::new(root.to_path_buf(), &ent.file_name()))
            }),
    ))
}

fn read_profile<'a>(root: &'a Path) -> Result<Box<Iterator<Item = Result<Category, Error>> + 'a>, Error> {
    let buf = BufReader::new(File::open(profile_category_file(root))?);
    let iter = buf.lines()
        .filter(move |line| if let Ok(l) = line {
            root.join(l).is_dir()
        } else {
            true
        })
        .map(move |line_res| {
            line_res.map(|line| {
                Category::new(root.to_path_buf(), &OsString::from(line))
            })
        });
    Ok(Box::new(iter))
}

pub fn iterator<'a>(root: &'a Path) -> Result<Box<Iterator<Item = Result<Category, Error>> + 'a>, Error> {
    if profile_category_file(root).as_path().exists() {
        read_profile(root)
    } else {
        discover_in(root)
    }
}
