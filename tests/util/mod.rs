use failure::Fail;
use std::{
    io,
    path::{Path, PathBuf},
};

#[derive(Fail, Debug)]
#[fail(display = "An error occurred resolving path to test repository")]
enum ReposError {
    #[fail(display = "Path <{:?}> lacked expected parent", _0)]
    NoParentPath(PathBuf),
    #[fail(display = "Path <{:?}> errored during canonicalization", _1)]
    CanonicalizationError(#[fail(cause)] io::Error, PathBuf),
    #[fail(display = "Could not access repo root <{:?}>", _1)]
    NoRepoDir(#[fail(cause)] io::Error, PathBuf),
    #[fail(display = "Path <{:?}> exists but is not a directory", _0)]
    NotADir(PathBuf),
}

pub(crate) fn repos(name: &str) -> Result<PathBuf, impl Fail> {
    let root = Path::new(file!());
    let canon_path = match root.canonicalize() {
        Ok(c) => c,
        Err(e) => {
            return Err(ReposError::CanonicalizationError(
                e,
                root.to_path_buf(),
            ))
        },
    };
    let parent = match canon_path.parent() {
        Some(p) => p,
        None => return Err(ReposError::NoParentPath(canon_path)),
    };
    let super_parent = match parent.parent() {
        Some(p) => p,
        None => return Err(ReposError::NoParentPath(parent.to_owned())),
    };
    let repos_root = super_parent.join("repos").join(name);
    match repos_root.metadata() {
        Err(e) => Err(ReposError::NoRepoDir(e, repos_root)),
        Ok(m) => {
            if m.is_dir() {
                Ok(repos_root)
            } else {
                Err(ReposError::NotADir(repos_root))
            }
        },
    }
}
