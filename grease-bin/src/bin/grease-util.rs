extern crate grease;
extern crate clap;

use std::alloc::System;

#[global_allocator]
static GLOBAL: System = System;

fn main() {
    let matches = clap::App::new("grease-util")
        .version("0.1.0")
        .author("Kent Fredric <kentnl@gentoo.org>")
        .about("Low level utility portage multi-tool")
        .get_matches();
}
