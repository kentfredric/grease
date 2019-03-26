pub(crate) mod iterate;
pub(crate) mod parse_atom;

use clap::{crate_authors, crate_version, App, AppSettings, ArgMatches, Error, ErrorKind};

pub(crate) fn app<'x, 'y>() -> App<'x, 'y> {
    App::new("grease-util")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Low level utility portage multi-tool")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(iterate::subcommand())
        .subcommand(parse_atom::subcommand())
}

pub(crate) fn run(command: &ArgMatches<'_>) -> Result<(), Error> {
    match command.subcommand() {
        ("iterate", Some(sub_m)) => iterate::run(sub_m),
        _ => Err(Error::with_description(command.usage(), ErrorKind::MissingSubcommand)),
    }
}
