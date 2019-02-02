extern crate grease;

use std::path::Path;
use std::alloc::System;

#[global_allocator]
static GLOBAL: System = System;

fn main() {
    let p = Path::new("/usr/local/gentoo");
    for ent in grease::category::iterator(p).unwrap() {
        for pkg in grease::package::iterator(p, &ent).unwrap() {
            for ebuild in grease::ebuild::iterator(p, &ent, &pkg).unwrap() {
                println!("> {}/{}/{}", ent, pkg, ebuild);
            }
        }
    }

}
