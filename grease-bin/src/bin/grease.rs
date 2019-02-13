extern crate grease;

use std::alloc::System;
use std::path::Path;

#[global_allocator]
static GLOBAL: System = System;

fn main() -> std::result::Result<(), std::io::Error> {
    let p = Path::new("/usr/local/gentoo");
    for ent in grease::category::iterator(p)? {
        let ent_u = ent?;
        for pkg in grease::package::iterator(p, &ent_u)? {
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
