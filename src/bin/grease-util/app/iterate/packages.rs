use crate::app::arg::{category, formatter};
use clap::{App, ArgMatches, Error, SubCommand};
use grease::{
    repository::{Package, Repository},
    util::{optfilter::OptFilter, repoobject::RepoObject},
};
use std::path::Path;
pub(crate) const NAME: &str = "packages";
pub(crate) const ABOUT: &str = "Iterate all packages in a repository";

pub(crate) fn subcommand<'x, 'y>() -> App<'x, 'y> {
    SubCommand::with_name(NAME)
        .about(ABOUT)
        .arg(formatter::arg())
        .arg(category::arg())
}

pub(crate) fn run(repo: &str, command: &ArgMatches<'_>) -> Result<(), Error> {
    let r = Repository::new(Path::new(repo));
    let formatter = formatter::get(command);
    if let Some(categories) = category::get(command) {
        for cat in categories {
            let c = r.get_category(cat);
            if !c.is_legal() {
                panic!("{:?} is not legal in the given repository", c);
            }
            let citer =
                c.packages().expect("Error reading packages from repo");
            for it in citer
                .filter_oks(Package::is_legal)
                .extract_errs(|e| panic!(e))
            {
                println!("{}", it.render(&formatter));
            }
        }
    } else {
        let citer = r.packages().expect("Error reading ebuilds from repo");
        for it in
            citer.filter_oks(Package::is_legal).extract_errs(|e| panic!(e))
        {
            println!("{}", it.render(&formatter));
        }
    }
    Ok(())
}
