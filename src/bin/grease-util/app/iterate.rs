use clap::{App, AppSettings, Arg, ArgMatches, Error, ErrorKind, SubCommand};

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
        .arg(
            Arg::with_name("REPOSITORY")
                .short("r")
                .long("repository")
                .required(true)
                .takes_value(true)
                .visible_alias("repo")
                .env("GREASE_REPOSITORY")
                .help("Specifies the repository to iterate"),
        )
        .subcommand(categories::subcommand())
        .subcommand(packages::subcommand())
        .subcommand(ebuilds::subcommand())
}

pub(crate) fn run(command: &ArgMatches<'_>) -> Result<(), Error> {
    match command.subcommand() {
        (categories::NAME, Some(c_opts)) => {
            categories::run(command.value_of("REPOSITORY").unwrap(), c_opts)
        },
        (packages::NAME, Some(p_opts)) => {
            packages::run(command.value_of("REPOSITORY").unwrap(), p_opts)
        },
        (ebuilds::NAME, Some(e_opts)) => {
            ebuilds::run(command.value_of("REPOSITORY").unwrap(), e_opts)
        },
        _ => Err(Error::with_description(
            command.usage(),
            ErrorKind::MissingSubcommand,
        )),
    }
}
