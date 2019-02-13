extern crate grease;

use grease::repository::Repository;
use std::path::Path;
use std::result::Result;
use std::io::Error;

#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;

fn main() -> Result<(), Error> {
    let p = Path::new("/usr/local/gentoo");
    let r = Repository::new(p);
    for cat in r.categories()? {
        let cat_u = cat?;
        println!("{:?}", cat_u);
        for pkg in cat_u.packages()? {
            let pkg_u = pkg?;
            println!("{:?}", pkg_u);
            for ebuild in pkg_u.ebuilds()? {
                if let Ok(e) = ebuild {
                    println!("{:?}", e);
                }
            }
        }
    }
    Ok(())
}
