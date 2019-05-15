use crate::{
    atom::{
        category::Category as CategoryAtom, Atom, Package as PackageAtom,
    },
    repository::{Category, DeriveAtom, Ebuild, Package},
    util::optfilter::OptFilter,
};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Error, ErrorKind},
    path::{Path, PathBuf},
    result::Result,
};

/// Represents a gentoo repository
#[derive(Debug, Clone)]
pub struct Repository {
    root: PathBuf,
}

impl Repository {
    /// Construct a new Repository trait object
    pub fn new(root: &Path) -> Repository {
        Repository { root: root.to_path_buf() }
    }

    /// Returns the path to this repository
    pub fn path(&self) -> PathBuf { self.root.to_owned() }

    /// Returns an iterator over all categories in this repository
    pub fn categories(
        &self,
    ) -> Result<Box<dyn Iterator<Item = Result<Category, Error>>>, Error>
    {
        let my_root = self.root.to_owned();
        let profile_category_file =
            my_root.join("profiles").join("categories");
        if profile_category_file.exists() {
            Ok(Box::new(
                BufReader::new(File::open(profile_category_file)?)
                    .lines()
                    .map_oks(move |line| {
                        Ok(Category::new(my_root.to_owned(), line.to_owned()))
                    }),
            ))
        } else {
            Ok(Box::new(
                self.root
                    .read_dir()?
                    .map_oks(move |ent| {
                        Ok(Category::new(
                            my_root.to_owned(),
                            ent.file_name().to_str().unwrap().to_owned(),
                        ))
                    })
                    .filter_oks(Category::has_legal_name),
            ))
        }
    }

    /// Returns an iterator over all packages in this repository
    pub fn packages(
        &self,
    ) -> Result<Box<dyn Iterator<Item = Result<Package, Error>>>, Error> {
        self.categories().map(|cat_it| {
            Box::new(cat_it.filter_oks(Category::is_legal).flat_map(
                |cat_res| match cat_res {
                    Ok(cat) => match cat.packages() {
                        Ok(package_iter) => package_iter,
                        Err(e) => Box::new(vec![Err(e)].into_iter()),
                    },
                    Err(e) => Box::new(vec![Err(e)].into_iter()),
                },
            )) as Box<dyn Iterator<Item = _>>
        })
    }

    /// Returns an iterator over all ebuilds in this repository
    pub fn ebuilds(
        &self,
    ) -> Result<Box<dyn Iterator<Item = Result<Ebuild, Error>>>, Error> {
        self.packages().map(|pkg_it| {
            Box::new(pkg_it.filter_oks(Package::is_legal).flat_map(
                |pkg_res| match pkg_res {
                    Ok(pkg) => match pkg.ebuilds() {
                        Ok(ebuild_iter) => ebuild_iter,
                        Err(e) => Box::new(vec![Err(e)].into_iter()),
                    },
                    Err(e) => Box::new(vec![Err(e)].into_iter()),
                },
            )) as Box<dyn Iterator<Item = _>>
        })
    }

    /// Fetch a category by name in this repository
    pub fn get_category(&self, name: &str) -> Category {
        Category::new(self.root.to_owned(), name.to_string())
    }

    /// Extract this repositories name from its profiles dir
    pub fn name(&self) -> Result<String, ()> {
        match self.get_file("profiles/repo_name") {
            Ok(p) => {
                Ok(std::fs::read_to_string(p).unwrap().trim_end().to_owned())
            },
            Err(_) => unimplemented!(),
        }
    }

    fn profile_dir(&self) -> Result<PathBuf, ()> { self.get_dir("profiles") }

    fn eclass_dir(&self) -> Result<PathBuf, ()> { self.get_dir("eclass") }

    fn metadata_dir(&self) -> Result<PathBuf, ()> { self.get_dir("metadata") }

    pub(super) fn get_path<P: AsRef<Path>>(&self, name: P) -> PathBuf {
        self.root.join(name)
    }

    pub(super) fn get_or_create_dir<P: AsRef<Path>>(
        &self, name: P,
    ) -> Result<PathBuf, ()> {
        let p = self.get_path(name);
        match p.metadata() {
            Err(e) => match e.kind() {
                ErrorKind::NotFound => match fs::create_dir_all(&p) {
                    Ok(_) => Ok(p),
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            },
            Ok(meta) => {
                if meta.is_dir() {
                    Ok(p)
                } else {
                    unimplemented!()
                }
            },
        }
    }

    pub(super) fn get_dir<P: AsRef<Path>>(
        &self, name: P,
    ) -> Result<PathBuf, ()> {
        let p = self.get_path(name);
        match p.metadata() {
            Err(e) => match e.kind() {
                ErrorKind::NotFound => unimplemented!(),
                _ => unimplemented!(),
            },
            Ok(meta) => {
                if meta.is_dir() {
                    Ok(p)
                } else {
                    unimplemented!()
                }
            },
        }
    }

    pub(super) fn get_file<P: AsRef<Path>>(
        &self, name: P,
    ) -> Result<PathBuf, ()> {
        let p = self.root.join(name.as_ref());
        match p.metadata() {
            Err(e) => match e.kind() {
                ErrorKind::NotFound => unimplemented!(),
                _ => unimplemented!(),
            },
            Ok(meta) => {
                if !meta.is_dir() {
                    Ok(p)
                } else {
                    unimplemented!()
                }
            },
        }
    }

    pub(crate) fn eclass_path(&self, name: &str) -> Result<PathBuf, ()> {
        self.get_file(format!("eclass/{}.eclass", name))
    }
}
impl DeriveAtom<CategoryAtom, Category> for Repository {
    fn derive(&self, cat: CategoryAtom) -> Category {
        self.get_category(&cat.category)
    }
}
impl DeriveAtom<PackageAtom, Package> for Repository {
    fn derive(&self, package: PackageAtom) -> Package {
        self.get_category(&package.category).get_package(&package.package)
    }
}
impl DeriveAtom<Atom, Ebuild> for Repository {
    fn derive(&self, atom: Atom) -> Ebuild {
        let p = self.get_category(&atom.category).get_package(&atom.package);
        if let Some(ref i) = atom.revision {
            p.get_ebuild(&(atom.version + "-r" + i + ".ebuild"))
        } else {
            p.get_ebuild(&(atom.version + ".ebuild"))
        }
    }
}
