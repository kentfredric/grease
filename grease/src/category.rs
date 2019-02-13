use std::ffi;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path;
use std::result;

type CategoryIter = Box<Iterator<Item = result::Result<ffi::OsString, io::Error>>>;
type CategoryIterResult = result::Result<CategoryIter, io::Error>;

#[inline]
fn dirname_blacklisted(name: &str) -> bool {
    match name {
        "metadata" | "profiles" | "eclass" | ".git" | "distfiles" | "packages" | "scripts" => true,
        _ => false,
    }
}

#[inline]
fn valid_category(root: &path::Path, name: &str) -> bool {
    if dirname_blacklisted(name) {
        return false;
    }
    root.join(name).is_dir()
}

#[inline]
fn profile_category_file(root: &path::Path) -> path::PathBuf { root.join("profiles").join("categories") }

pub fn discover_in(root: &'static path::Path) -> CategoryIterResult {
    Ok(Box::new(
        root.read_dir()?
            .filter(move |e| if let Ok(entry) = e {
                valid_category(root, &entry.file_name().into_string().unwrap())
            } else {
                // Passthrough errors
                true
            })
            .map(|e| e.map(|ent| ent.file_name())),
    ))
}

pub fn read_profile(root: &'static path::Path) -> CategoryIterResult {
    let buf = io::BufReader::new(fs::File::open(profile_category_file(root))?);
    let iter = buf.lines()
        .filter(move |line| if let Ok(l) = line {
            root.join(l).is_dir()
        } else {
            true
        })
        .map(|line| line.map(ffi::OsString::from));
    Ok(Box::new(iter))
}

pub fn iterator(root: &'static path::Path) -> CategoryIterResult {
    if profile_category_file(root).as_path().exists() {
        read_profile(root)
    } else {
        discover_in(root)
    }
}
