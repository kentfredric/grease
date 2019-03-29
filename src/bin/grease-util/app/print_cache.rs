pub(crate) const ABOUT: &str = "Print a computed cache for a given atom";
pub(crate) const NAME: &str = "print-cache";

use crate::app::arg::repository;
use clap::{App, Arg, ArgMatches, Error, SubCommand};
use grease::repository::{MetaDataCache, Repository};
use std::path::Path;

pub(crate) fn subcommand<'x, 'y>() -> App<'x, 'y> {
    SubCommand::with_name(NAME).about(ABOUT).arg(repository::arg()).arg(
        Arg::with_name("ATOM")
            .help("The cat/package-ver to get the cache for")
            .required(true)
            .takes_value(true)
            .multiple(false)
            .empty_values(false),
    )
}

pub(crate) fn run(command: &ArgMatches<'_>) -> Result<(), Error> {
    let repo = repository::get(command);
    let atom = command.value_of("ATOM").unwrap();
    let cache = MetaDataCache::new(Repository::new(Path::new(repo)));
    Ok(())
}
