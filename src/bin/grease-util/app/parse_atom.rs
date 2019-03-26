use clap::{App, AppSettings, ArgMatches, Error, ErrorKind, SubCommand};

mod atom;
mod category;
mod package;

pub(crate) fn subcommand<'x, 'y>() -> App<'x, 'y> {
    SubCommand::with_name("parse_atom")
        .name("parse-atom")
        .about("Parse various forms of Gentoo Atom")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(category::subcommand())
        .subcommand(package::subcommand())
        .subcommand(atom::subcommand())
}

pub(crate) fn run(command: &ArgMatches<'_>) -> Result<(), Error> {
    match command.subcommand() {
        ("category", Some(args)) => category::run(args),
        ("package", Some(args)) => package::run(args),
        ("atom", Some(args)) => atom::run(args),
        _ => Err(Error::with_description(command.usage(), ErrorKind::MissingSubcommand)),
    }
}
