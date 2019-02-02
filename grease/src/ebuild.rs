
use std::path;
use std::result;
use std::io;
use std::ffi;

type EbuildIter = Box<Iterator<Item = result::Result<ffi::OsString, io::Error>>>;
type EbuildIterResult = result::Result<EbuildIter, io::Error>;

fn in_package_dir(ebuild_root: &path::Path) -> EbuildIterResult {
    Ok(Box::new(
        ebuild_root
            .read_dir()?
            .filter(move |e| if let Ok(entry) = e {
                let p = entry.path();
                if let Some(ext) = p.extension() {
                    ext.eq("ebuild") && !p.is_dir()
                } else {
                    false
                }
            } else {
                false
            })
            .map(|e| e.map( |ent| ent.file_name())),
    ))
}

pub fn iterator(root: &path::Path, category: &str, package: &str) -> EbuildIterResult {
    in_package_dir(&root.join(category).join(package))
}
