use clap::{App, Arg, ArgMatches, Error, ErrorKind, SubCommand};

pub(crate) fn subcommand<'x, 'y>() -> App<'x, 'y> {
    SubCommand::with_name("package").about("Validate/Parse a package(with category)").arg(
        Arg::with_name("PACKAGE")
            .help("The name of a package to parse")
            .required(true)
            .takes_value(true)
            .multiple(true)
            .empty_values(false),
    )
}

pub(crate) fn run(command: &ArgMatches<'_>) -> Result<(), Error> {
    Err(Error::with_description(command.usage(), ErrorKind::MissingSubcommand))
}