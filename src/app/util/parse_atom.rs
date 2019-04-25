use clap::{App, AppSettings, ArgMatches, Error, ErrorKind, SubCommand};

mod atom;
mod category;
mod package;

pub(crate) const NAME: &str = "parse-atom";
pub(crate) const ABOUT: &str = "Parse variouys forms of Gentoo Atom";

pub(crate) fn subcommand<'x, 'y>() -> App<'x, 'y> {
    SubCommand::with_name(NAME)
        .about(ABOUT)
        .setting(AppSettings::SubcommandRequired)
        .subcommand(category::subcommand())
        .subcommand(package::subcommand())
        .subcommand(atom::subcommand())
}

pub(crate) fn run(command: &ArgMatches<'_>) -> Result<(), Error> {
    match command.subcommand() {
        (category::NAME, Some(args)) => category::run(args),
        (package::NAME, Some(args)) => package::run(args),
        (atom::NAME, Some(args)) => atom::run(args),
        _ => Err(Error::with_description(
            command.usage(),
            ErrorKind::MissingSubcommand,
        )),
    }
}
