use clap::{App, AppSettings, SubCommand};

pub(crate) fn subcommand<'x, 'y>() -> App<'x, 'y> {
    SubCommand::with_name("parse_atom")
        .name("parse-atom")
        .about("Parse various forms of Gentoo Atom")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("category").about("Validate/Parse a category name"))
        .subcommand(SubCommand::with_name("package").about("Validate/Parse a package(with category)"))
        .subcommand(SubCommand::with_name("atom").about("Validate/Parse a category/package-atom set"))
}
