
use std::io;
use std::result;
use std::path;

type PackageIter = Box<Iterator<Item = String>>;
type PackageIterResult = result::Result<PackageIter, io::Error>;

fn in_category_dir(category_root: &path::Path) -> PackageIterResult {
    Ok(Box::new(
        category_root
            .read_dir()?
            .filter(move |e| if let Ok(entry) = e {
                entry.path().is_dir()
            } else {
                false
            })
            .map(|e| e.unwrap().file_name().into_string().unwrap()),
    ))
}

pub fn iterator(root: &'static path::Path, category: &str) -> PackageIterResult {
    in_category_dir(&root.join(category))
}
