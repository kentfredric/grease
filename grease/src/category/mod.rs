use std::fs;
use std::io;
use std::io::BufRead;
use std::path;
use std::result;

type CategoryIter = Box<Iterator<Item = String>>;
type CategoryIterResult = result::Result<CategoryIter, io::Error>;

#[inline]
fn dirname_blacklisted(name: &String) -> bool {
    match name.as_str() {
        "metadata" | "profiles" | "eclass" | ".git" | "distfiles" | "packages" | "scripts" => true,
        _ => false,
    }
}

#[inline]
fn valid_category(root: &path::Path, name: &String) -> bool {
    if dirname_blacklisted(name) {
        return false;
    }
    root.join(name).is_dir()
}

#[inline]
fn profile_category_file(root: &path::Path) -> path::PathBuf {
    root.join("profiles").join("categories")
}

pub fn discover_in(root: &'static path::Path) -> CategoryIterResult {
    Ok(Box::new(
        root.read_dir()?
            .filter(move |e| if let Ok(entry) = e {
                valid_category(root, &entry.file_name().into_string().unwrap())
            } else {
                true
            })
            .map(|e| e.unwrap().file_name().into_string().unwrap()),
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
        .map(|line| line.unwrap().clone());
    Ok(Box::new(iter))
}

pub fn iterator(root: &'static path::Path) -> CategoryIterResult {
    match profile_category_file(root).as_path().exists() {
        true => read_profile(root),
        false => discover_in(root),
    }
}
