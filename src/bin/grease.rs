use grease::repository::Repository;
use std::io::Error;
use std::path::Path;
use std::result::Result;

#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;

fn main() -> Result<(), Error> {
    let p = Path::new("/usr/local/gentoo");
    let r = Repository::new(p);
    for ebuild in r.ebuilds()? {
        println!("{:?}", ebuild?);
    }
    Ok(())
}
