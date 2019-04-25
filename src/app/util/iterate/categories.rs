use crate::{
    app::util::arg::formatter,
    repository::{Category, Repository},
    util::{optfilter::OptFilter, repoobject::RepoObject},
};
use clap::{App, ArgMatches, Error, SubCommand};
use std::path::Path;

pub(crate) const NAME: &str = "categories";
pub(crate) const ABOUT: &str = "Iterate all categories in a repository";

pub(crate) fn subcommand<'x, 'y>() -> App<'x, 'y> {
    SubCommand::with_name(NAME).about(ABOUT).arg(formatter::arg())
}

pub(crate) fn run(repo: &str, command: &ArgMatches<'_>) -> Result<(), Error> {
    let r = Repository::new(Path::new(repo));
    let citer = r.categories().expect("Error reading categories from repo");
    let formatter = formatter::get(command);
    for it in citer.filter_oks(Category::is_legal).extract_errs(|e| panic!(e))
    {
        println!("{}", it.render(&formatter));
    }
    Ok(())
}
