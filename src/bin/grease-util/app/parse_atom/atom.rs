use clap::{App, Arg, ArgMatches, Error, SubCommand};
use grease::atom::Atom;

pub(crate) const NAME: &str = "atom";
pub(crate) const ABOUT: &str =
    "Validate/Parse a category/package-version set";

pub(crate) fn subcommand<'x, 'y>() -> App<'x, 'y> {
    SubCommand::with_name(NAME).about(ABOUT).arg(
        Arg::with_name("ATOM")
            .help("The name of a category/package-version set to parse")
            .required(true)
            .takes_value(true)
            .multiple(true)
            .empty_values(false),
    )
}

pub(crate) fn run(command: &ArgMatches<'_>) -> Result<(), Error> {
    let atoms: Vec<&str> = command.values_of("ATOM").unwrap().collect();
    for i in atoms {
        let p = i.parse::<Atom>().unwrap();
        match p.revision() {
            Some(r) => println!(
                "{} {} {} {}",
                p.category(),
                p.package(),
                p.version(),
                r
            ),
            None => {
                println!("{} {} {}", p.category(), p.package(), p.version())
            },
        }
    }
    Ok(())
}
