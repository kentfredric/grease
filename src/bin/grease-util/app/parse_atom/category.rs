use clap::{App, Arg, ArgMatches, Error, ErrorKind, SubCommand};

pub(crate) fn subcommand<'x, 'y>() -> App<'x, 'y> {
    SubCommand::with_name("category").about("Validate/Parse a category name").arg(
        Arg::with_name("CATEGORY")
            .help("The name of a category to parse")
            .required(true)
            .takes_value(true)
            .multiple(true)
            .empty_values(false),
    )
}

pub(crate) fn run(command: &ArgMatches<'_>) -> Result<(), Error> {
    Err(Error::with_description(command.usage(), ErrorKind::MissingSubcommand))
}
