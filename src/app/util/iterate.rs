use crate::app::util::arg::repository;
use clap::{App, AppSettings, ArgMatches, Error, ErrorKind, SubCommand};

mod categories;
mod ebuilds;
mod packages;

pub(crate) const NAME: &str = "iterate";
pub(crate) const ABOUT: &str =
    "Iterate various kinds of things in Gentoo Repositories";

pub(crate) fn subcommand<'x, 'y>() -> App<'x, 'y> {
    SubCommand::with_name(NAME)
        .about(ABOUT)
        .setting(AppSettings::SubcommandRequired)
        .arg(repository::arg())
        .subcommand(categories::subcommand())
        .subcommand(packages::subcommand())
        .subcommand(ebuilds::subcommand())
}

pub(crate) fn run(command: &ArgMatches<'_>) -> Result<(), Error> {
    match command.subcommand() {
        (categories::NAME, Some(c_opts)) => {
            categories::run(repository::get(command), c_opts)
        },
        (packages::NAME, Some(p_opts)) => {
            packages::run(repository::get(command), p_opts)
        },
        (ebuilds::NAME, Some(e_opts)) => {
            ebuilds::run(repository::get(command), e_opts)
        },
        _ => Err(Error::with_description(
            command.usage(),
            ErrorKind::MissingSubcommand,
        )),
    }
}
