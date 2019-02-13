extern crate grease;

use std::path::Path;
use std::alloc::System;

#[global_allocator]
static GLOBAL: System = System;

fn main() -> std::result::Result<(), std::io::Error> {
    let p = Path::new("/usr/local/gentoo");
    for ent in grease::category::iterator(p)? {
        let ent_u = ent?;
        for pkg in grease::package::package_iterator(p, &ent_u)? {
            let pkg_u = pkg?;
            for ebuild in pkg_u.ebuilds()? {
                if let Ok(e) = ebuild {
                    println!("{:?}", e);
                }
            }
            /*
            for ebuild in grease::ebuild::iterator(p, &ent_u, &pkg_u).unwrap() {
                println!(
                    "> {}/{}/{}",
                    ent_u.clone().into_string().unwrap(),
                    pkg_u.clone().into_string().unwrap(),
                    ebuild?.into_string().unwrap()
                );
            }
            */
        }
    }
    Ok(())
}
