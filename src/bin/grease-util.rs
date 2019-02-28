#[macro_use]
extern crate clap;

use clap::ArgMatches;
use grease::{
    category::Category,
    ebuild::Ebuild,
    package::Package,
    repository::Repository,
    util::{
        optfilter::OptFilter,
        repoobject::{self, RepoObject},
    },
};
use std::{alloc::System, path::Path};

#[global_allocator]
static GLOBAL: System = System;

fn main() {
    let app_m = clap_app!(
            greaseutil =>
        (name: "grease-util")
        (version: crate_version!())
        (author: crate_authors!())
        (about: "Low level utility portage multi-tool")
        (@setting SubcommandRequired)
        (@subcommand iterate =>
            (about: "Traverse all kinds of a thing in a given repository")
            (@arg REPOSITORY: -r --repository +required +takes_value visible_alias[repo]
                "Specifies the repository to iterate"
            )
            (@setting SubcommandRequired)
            (@subcommand categories =>
                (about: "Iterate all categories in a repository")
                (@arg FORMATTER: -f --formatter +takes_value possible_value[path ident components name] default_value[ident])
            )
            (@subcommand packages =>
                (about: "Iterate all packages in a repository")
                (@arg FORMATTER: -f --formatter +takes_value possible_value[path ident components name] default_value[ident])
            )
            (@subcommand ebuilds =>
                (about: "Iterate all ebuilds in a repository")
                (@arg FORMATTER: -f --formatter +takes_value possible_value[path ident components name] default_value[ident])
            )
        )
    )
    .get_matches();
    match app_m.subcommand() {
        ("iterate", Some(sub_m)) => match sub_m.subcommand() {
            ("categories", Some(c_opts)) => iter_repo_categories(sub_m.value_of("REPOSITORY").unwrap(), c_opts),
            ("packages", Some(p_opts)) => iter_repo_packages(sub_m.value_of("REPOSITORY").unwrap(), p_opts),
            ("ebuilds", Some(e_opts)) => iter_repo_ebuilds(sub_m.value_of("REPOSITORY").unwrap(), e_opts),
            _ => clap::Error::with_description(sub_m.usage(), clap::ErrorKind::MissingSubcommand).exit(),
        },
        _ => clap::Error::with_description(app_m.usage(), clap::ErrorKind::MissingSubcommand).exit(),
    }
}

fn iter_repo_categories(repo: &str, opts: &ArgMatches) {
    let r = Repository::new(Path::new(repo));
    let citer = r.categories().expect("Error reading categories from repo");
    let formatter = repoobject::parse_formatter(opts.value_of("FORMATTER").unwrap()).unwrap();
    for it in citer.filter_oks(Category::is_legal).extract_errs(|e| panic!(e)) {
        println!("{}", it.render(&formatter));
    }
}
fn iter_repo_packages(repo: &str, opts: &ArgMatches) {
    let r = Repository::new(Path::new(repo));
    let citer = r.packages().expect("Error reading ebuilds from repo");
    let formatter = repoobject::parse_formatter(opts.value_of("FORMATTER").unwrap()).unwrap();
    for it in citer.filter_oks(Package::is_legal).extract_errs(|e| panic!(e)) {
        println!("{}", it.render(&formatter));
    }
}
fn iter_repo_ebuilds(repo: &str, opts: &ArgMatches) {
    let r = Repository::new(Path::new(repo));
    let citer = r.ebuilds().expect("Error reading ebuilds from repo");
    let formatter = repoobject::parse_formatter(opts.value_of("FORMATTER").unwrap()).unwrap();
    for it in citer.filter_oks(Ebuild::is_legal).extract_errs(|e| panic!(e)) {
        println!("{}", it.render(&formatter));
    }
}
