use clap::{App, Arg, ArgMatches, Error, SubCommand};
use grease::atom::Package;

pub(crate) const NAME: &str = "package";
pub(crate) const ABOUT: &str = "Validate/Parse a package(with category)";

pub(crate) fn subcommand<'x, 'y>() -> App<'x, 'y> {
    SubCommand::with_name(NAME).about(ABOUT).arg(
        Arg::with_name("PACKAGE")
            .help("The name of a package to parse")
            .required(true)
            .takes_value(true)
            .multiple(true)
            .empty_values(false),
    )
}

pub(crate) fn run(command: &ArgMatches<'_>) -> Result<(), Error> {
    let packages: Vec<&str> = command.values_of("PACKAGE").unwrap().collect();
    for i in packages {
        let p = i.parse::<Package>().unwrap();
        println!("{} {}", p.category(), p.package());
    }
    Ok(())
}
