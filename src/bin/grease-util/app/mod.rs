pub(crate) mod iterate;
pub(crate) mod parse_atom;

use clap::{crate_authors, crate_version, App, AppSettings, ArgMatches, Error, ErrorKind};

pub(crate) fn app<'x, 'y>() -> App<'x, 'y> {
    App::new("grease-util")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Low level utility portage multi-tool")
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(iterate::subcommand())
        .subcommand(parse_atom::subcommand())
}

pub(crate) fn run(command: &ArgMatches<'_>) -> Result<(), Error> {
    match command.subcommand() {
        ("iterate", Some(sub_c)) => iterate::run(sub_c),
        ("parse_atom", Some(sub_c)) => parse_atom::run(sub_c),
        _ => Err(Error::with_description(command.usage(), ErrorKind::MissingSubcommand)),
    }
}
