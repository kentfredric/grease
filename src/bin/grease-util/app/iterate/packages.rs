use clap::{App, Arg, ArgMatches, Error, SubCommand};
use grease::{
    repository::{Package, Repository},
    util::{
        optfilter::OptFilter,
        repoobject::{self, RepoObject},
    },
};
use std::path::Path;
pub(crate) fn subcommand<'x, 'y>() -> App<'x, 'y> {
    SubCommand::with_name("packages")
        .about("Iterate all packages in a repository")
        .arg(
            Arg::with_name("FORMATTER")
                .short("f")
                .long("formatter")
                .takes_value(true)
                .possible_values(&["path", "ident", "components", "name"])
                .default_value("ident")
                .visible_alias("format")
                .help("Controls the presentation of output"),
        )
        .arg(
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
    let formatter = repoobject::parse_formatter(command.value_of("FORMATTER").unwrap()).unwrap();
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
