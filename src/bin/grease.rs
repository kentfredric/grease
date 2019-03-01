/*! Work in progress

This is mostly a place-holder at present for testing purposes, but will eventually
be a general portage tool if I ever work out how

!*/
use grease::repository::Repository;
use std::{io::Error, path::Path, result::Result};

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
