#[macro_use]
extern crate clap;

use std::alloc::System;

#[global_allocator]
static GLOBAL: System = System;

fn main() {
    let matches = clap_app!(
            greaseutil =>
        (name: "grease-util")
        (version: crate_version!())
        (author: crate_authors!())
        (about: "Low level utility portage multi-tool")
    )
    .get_matches();
}
