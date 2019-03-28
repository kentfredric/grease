use crate::app::arg::formatter;
use clap::{App, Arg, ArgMatches, Error, SubCommand};
use grease::{
    repository::{Package, Repository},
    util::{
        optfilter::OptFilter,
        repoobject::{self, RepoObject},
    },
};
use std::path::Path;
pub(crate) const NAME: &str = "packages";
pub(crate) const ABOUT: &str = "Iterate all packages in a repository";

pub(crate) fn subcommand<'x, 'y>() -> App<'x, 'y> {
    SubCommand::with_name(NAME).about(ABOUT).arg(formatter::arg()).arg(
        Arg::with_name("CATEGORY")
            .short("c")
            .long("category")
            .value_name("CATEGORY_NAME")
            .takes_value(true)
            .multiple(true)
            .takes_value(true)
            .min_values(1)
            .max_values(100)
            .require_delimiter(true)
            .visible_alias("cat")
            .help("Restrict categories to a list provided"),
    )
}

pub(crate) fn run(repo: &str, command: &ArgMatches<'_>) -> Result<(), Error> {
    let r = Repository::new(Path::new(repo));
    let formatter = formatter::get(command);
    if command.is_present("CATEGORY") {
        let values_iter = command.values_of("CATEGORY").unwrap();
        for cat in values_iter {
            let c = r.get_category(cat);
            if !c.is_legal() {
                panic!("{:?} is not legal in the given repository", c);
            }
            let citer = c.packages().expect("Error reading packages from repo");
            for it in citer.filter_oks(Package::is_legal).extract_errs(|e| panic!(e)) {
                println!("{}", it.render(&formatter));
            }
        }
    } else {
        let citer = r.packages().expect("Error reading ebuilds from repo");
        for it in citer.filter_oks(Package::is_legal).extract_errs(|e| panic!(e)) {
            println!("{}", it.render(&formatter));
        }
    }
    Ok(())
}
