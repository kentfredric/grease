
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

#[inline]
fn path_concat(a: &ffi::OsString, b: &ffi::OsString) -> Result<ffi::OsString, io::Error> {
    let a_pth = a.to_str().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Base path {:?} to concat did not convert to a str", a),
        )
    })?;
    let b_pth = b.to_str().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Leaf path {:?} to concat did not convert to a str", b),
        )
    })?;
    Ok(ffi::OsString::from(format!("{}/{}", a_pth, b_pth)))
}

pub fn ebuild_iterator(root: &'static path::Path, category: &ffi::OsStr) -> PackageIterResult {
    let mut out = Vec::new();
    for package_result in iterator(&root, &category)? {
        let package = package_result?;
        out.push(super::ebuild::iterator(&root, &category, &package)?.map(
            move |ebuild| path_concat(&package, &ebuild?),
        ));
    }
    Ok(Box::new(out.into_iter().flatten()))
}
