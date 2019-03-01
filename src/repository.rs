use super::{category::Category, ebuild::Ebuild, package::Package, util::optfilter::OptFilter};
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    path::{Path, PathBuf},
    result::Result,
};

/// Represents a gentoo repository
#[derive(Debug)]
pub struct Repository {
    root: PathBuf,
}

impl Repository {
    /// Construct a new Repository trait object
    pub fn new(root: &Path) -> Repository { Repository { root: root.to_path_buf() } }

    /// Returns the path to this repository
    pub fn path(&self) -> PathBuf { self.root.to_owned() }

    /// Returns an iterator over all categories in this repository
    pub fn categories(&self) -> Result<Box<dyn Iterator<Item = Result<Category, Error>>>, Error> {
        let my_root = self.root.to_owned();
        let profile_category_file = my_root.join("profiles").join("categories");
        if profile_category_file.exists() {
            Ok(Box::new(
                BufReader::new(File::open(profile_category_file)?)
                    .lines()
                    .map_oks(move |line| Ok(Category::new(my_root.to_owned(), line.to_owned()))),
            ))
        } else {
            Ok(Box::new(
                self.root
                    .read_dir()?
                    .map_oks(move |ent| {
                        Ok(Category::new(my_root.to_owned(), ent.file_name().to_str().unwrap().to_owned()))
                    })
                    .filter_oks(Category::has_legal_name),
            ))
        }
    }

    /// Returns an iterator over all packages in this repository
    pub fn packages(&self) -> Result<Box<dyn Iterator<Item = Result<Package, Error>>>, Error> {
        self.categories().map(|cat_it| {
            Box::new(cat_it.filter_oks(Category::is_legal).flat_map(|cat_res| match cat_res {
                Ok(cat) => match cat.packages() {
                    Ok(package_iter) => package_iter,
                    Err(e) => Box::new(vec![Err(e)].into_iter()),
                },
                Err(e) => Box::new(vec![Err(e)].into_iter()),
            })) as Box<dyn Iterator<Item = _>>
        })
    }

    /// Returns an iterator over all ebuilds in this repository
    pub fn ebuilds(&self) -> Result<Box<dyn Iterator<Item = Result<Ebuild, Error>>>, Error> {
        self.packages().map(|pkg_it| {
            Box::new(pkg_it.filter_oks(Package::is_legal).flat_map(|pkg_res| match pkg_res {
                Ok(pkg) => match pkg.ebuilds() {
                    Ok(ebuild_iter) => ebuild_iter,
                    Err(e) => Box::new(vec![Err(e)].into_iter()),
                },
                Err(e) => Box::new(vec![Err(e)].into_iter()),
            })) as Box<dyn Iterator<Item = _>>
        })
    }

    /// Fetch a category by name in this repository
    pub fn get_category(&self, name: &str) -> Category { Category::new(self.root.to_owned(), name.to_string()) }
}
