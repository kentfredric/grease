use clap::{App, Arg, ArgMatches, SubCommand};
use grease::{
    repository::{Ebuild, Repository},
    util::{
        optfilter::OptFilter,
        repoobject::{self, RepoObject},
    },
};
use std::path::Path;
pub(crate) fn subcommand<'x, 'y>() -> App<'x, 'y> {
    SubCommand::with_name("ebuilds")
        .about("Iterate all ebuilds in a repository")
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

pub(crate) fn run(repo: &str, command: &ArgMatches<'_>) {
    let r = Repository::new(Path::new(repo));
    let formatter = repoobject::parse_formatter(command.value_of("FORMATTER").unwrap()).unwrap();

    let citer = r.ebuilds().expect("Error reading ebuilds from repo");

    if command.is_present("CATEGORY") {
        let values_iter = command.values_of("CATEGORY").unwrap();
        for cat in values_iter {
            let c = r.get_category(cat);
            if !c.is_legal() {
                panic!("{:?} is not legal in the given repository", c);
            }
            let citer = c.ebuilds().expect("Error reading ebuilds from repo");
            for it in citer.filter_oks(Ebuild::is_legal).extract_errs(|e| panic!(e)) {
                println!("{}", it.render(&formatter));
            }
        }
    } else {
        for it in citer.filter_oks(Ebuild::is_legal).extract_errs(|e| panic!(e)) {
            println!("{}", it.render(&formatter));
        }
    }
}
