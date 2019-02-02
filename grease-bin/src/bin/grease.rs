extern crate grease;

use std::path::Path;
use std::alloc::System;

#[global_allocator]
static GLOBAL: System = System;

fn main() {
    let p = Path::new("/usr/local/gentoo");
    let iter = grease::category::iterator(p);
    for ent in iter.unwrap().into_iter() {
        for pkg in grease::package::iterator(p, &ent).unwrap().into_iter() {
            println!("> {}/{}", ent, pkg);
        }
    }

}
