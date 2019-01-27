extern crate grease;

use std::path::Path;

fn main() {
    let p = Path::new("/usr/local/gentoo");
    let iter = grease::repository::Repository::new(p).categories();
    for ent in iter.unwrap().into_iter() {
        println!("> {}", ent);
    }

}
