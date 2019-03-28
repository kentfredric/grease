use clap::{App, Arg, ArgMatches, Error, SubCommand};
use grease::atom::Category;

pub(crate) const NAME: &str = "category";
pub(crate) const ABOUT: &str = "Validate/Parse a category name";

pub(crate) fn subcommand<'x, 'y>() -> App<'x, 'y> {
    SubCommand::with_name(NAME).about(ABOUT).arg(
        Arg::with_name("CATEGORY")
            .help("The name of a category to parse")
            .required(true)
            .takes_value(true)
            .multiple(true)
            .empty_values(false),
    )
}

pub(crate) fn run(command: &ArgMatches<'_>) -> Result<(), Error> {
    let categories: Vec<&str> =
        command.values_of("CATEGORY").unwrap().collect();
    for i in categories {
        println!("{}", i.parse::<Category>().unwrap().category())
    }
    Ok(())
}
