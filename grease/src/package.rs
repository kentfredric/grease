
use std::io;
use std::result;
use std::path;
use std::ffi;

type PackageIter = Box<Iterator<Item = result::Result<ffi::OsString, io::Error>>>;
type PackageIterResult = result::Result<PackageIter, io::Error>;

fn in_category_dir(category_root: &path::Path) -> PackageIterResult {
    Ok(Box::new(
        category_root
            .read_dir()?
            .filter(move |e| if let Ok(entry) = e {
                entry.path().is_dir()
            } else {
                // readdir entry failures passthrough
                true
            })

            // Munge Ok(), passthru Err()
            .map(|e| e.map( |ent| ent.file_name() ) ),
    ))
}

pub fn iterator(root: &'static path::Path, category: &ffi::OsStr) -> PackageIterResult {
    in_category_dir(&root.join(category))
}

pub fn ebuild_iterator(root: &'static path::Path, category: &ffi::OsStr) -> PackageIterResult {
    let packages = iterator(&root, &category)?;
    let mut out = Vec::new();
    for packageResult in packages {
        let package = packageResult?;
        let ebuilds = super::ebuild::iterator(&root, &category, &package)?;
        for ebuildResult in ebuilds {
            let ebuild = ebuildResult?;
            out.push(Ok(ffi::OsString::from(format!(
                "{}/{}",
                package.clone().into_string().unwrap(),
                ebuild.into_string().unwrap()
            ))));
        }
    }
    Ok(Box::new(out.into_iter()))
}
