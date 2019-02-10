extern crate grease;

use std::path::Path;
use std::alloc::System;

#[global_allocator]
static GLOBAL: System = System;

fn main() -> std::result::Result<(), std::io::Error> {
    let p = Path::new("/usr/local/gentoo");
    for ent in grease::category::iterator(p)? {
        let ent_u = ent?;
        for pkg in grease::package::iterator(p, &ent_u)? {
            let pkg_u = pkg?;
            for ebuild in grease::ebuild::ebuild_iterator(p, &ent_u, &pkg_u)? {
                if let Ok(e) = ebuild {
                    let v = e.version();
                    match v {
                        Some(version) => println!( ">v={}, {:?} ", version, e),
                        _ => println!( ">v=None, {:?}", e),
                    }
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
