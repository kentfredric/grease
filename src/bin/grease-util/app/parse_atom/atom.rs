use clap::{App, Arg, ArgMatches, Error, ErrorKind, SubCommand};

pub(crate) fn subcommand<'x, 'y>() -> App<'x, 'y> {
    SubCommand::with_name("atom").about("Validate/Parse a category/package-version set").arg(
        Arg::with_name("ATOM")
            .help("The name of a category/package-version set to parse")
            .required(true)
            .takes_value(true)
            .multiple(true)
            .empty_values(false),
    )
}

pub(crate) fn run(command: &ArgMatches<'_>) -> Result<(), Error> {
    Err(Error::with_description(command.usage(), ErrorKind::MissingSubcommand))
}