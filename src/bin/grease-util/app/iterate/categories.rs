use clap::{App, Arg, ArgMatches, SubCommand};
use grease::{
    repository::{Category, Repository},
    util::{
        optfilter::OptFilter,
        repoobject::{self, RepoObject},
    },
};
use std::path::Path;
pub(crate) fn subcommand<'x, 'y>() -> App<'x, 'y> {
    SubCommand::with_name("categories").about("Iterate all categories in a repository").arg(
        Arg::with_name("FORMATTER")
            .short("f")
            .long("formatter")
            .takes_value(true)
            .possible_value("path")
            .possible_value("ident")
            .possible_value("components")
            .possible_value("name")
            .default_value("ident")
            .visible_alias("format")
            .help("Controls the presentation of output"),
    )
}

pub(crate) fn run(repo: &str, command: &ArgMatches<'_>) {
    let r = Repository::new(Path::new(repo));
    let citer = r.categories().expect("Error reading categories from repo");
    let formatter = repoobject::parse_formatter(command.value_of("FORMATTER").unwrap()).unwrap();
    for it in citer.filter_oks(Category::is_legal).extract_errs(|e| panic!(e)) {
        println!("{}", it.render(&formatter));
    }
}
