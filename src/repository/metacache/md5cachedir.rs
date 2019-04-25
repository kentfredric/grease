#![allow(dead_code)]
use ::std::{
    boxed::Box,
    io::ErrorKind,
    iter::Extend,
    option::Option::{self, None, Some},
    path::PathBuf,
    result::Result::{self, Err, Ok},
    str, unimplemented,
    vec::Vec,
};

pub(super) struct Md5CacheDir {
    root:  PathBuf,
    child: Option<Box<Md5CacheDir>>,
}

impl Md5CacheDir {
    pub(super) fn new(
        root: PathBuf, fallbacks: Option<Vec<PathBuf>>,
    ) -> Self {
        let mut i = Self { root, child: None };
        if let Some(fb) = fallbacks {
            i.add_children(fb);
        }
        i
    }

    fn add_child(&mut self, root: PathBuf) {
        match &mut self.child {
            None => self.child = Some(Box::new(Self { root, child: None })),
            Some(c) => c.add_child(root),
        }
    }

    pub(crate) fn add_children(&mut self, roots: Vec<PathBuf>) {
        for root in roots {
            self.add_child(root)
        }
    }

    fn get(&self, path: &str) -> Result<Option<PathBuf>, ()> {
        let fpath = self.root.join(path);
        match fpath.metadata() {
            Err(e) => match e.kind() {
                ErrorKind::NotFound => Ok(None),
                _ => unimplemented!(),
            },
            Ok(m) => {
                if m.is_dir() {
                    unimplemented!()
                } else {
                    Ok(Some(fpath))
                }
            },
        }
    }

    pub(crate) fn get_iter(
        &self, path: &str,
    ) -> Box<dyn Iterator<Item = PathBuf>> {
        match self.get(path) {
            Err(_) => unimplemented!(),
            Ok(Some(p)) => {
                if let Some(c) = &self.child {
                    Box::new(std::iter::once(p).chain(c.get_iter(path)))
                } else {
                    Box::new(std::iter::once(p))
                }
            },
            Ok(None) => {
                if let Some(c) = &self.child {
                    c.get_iter(path)
                } else {
                    Box::new(std::iter::empty())
                }
            },
        }
    }

    fn get_all(&self, path: &str) -> Result<Option<Vec<PathBuf>>, ()> {
        let mut out: Vec<PathBuf> = Vec::new();
        match self.get(path) {
            Err(_) => unimplemented!(),
            Ok(Some(p)) => out.push(p),
            Ok(None) => (),
        };
        if let Some(c) = &self.child {
            match c.get_all(path) {
                Err(_) => unimplemented!(),
                Ok(Some(p)) => {
                    out.extend(p);
                },
                Ok(None) => (),
            }
        }
        if out.is_empty() {
            return Ok(None);
        }
        Ok(Some(out))
    }
}
