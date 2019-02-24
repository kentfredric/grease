#[macro_use]
extern crate clap;

use grease::{
    category::Category, ebuild::Ebuild, package::Package, repository::Repository, util::optfilter::OptFilter,
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
            (@arg REPOSITORY: -r --repository +required +takes_value "Specifies the repository to iterate")
            (@setting SubcommandRequired)
            (@subcommand categories =>
                (about: "Iterate all categories in a repository")
            )
            (@subcommand category_paths =>
                (name: "category-paths")
                (about: "Iterate all categories in a repository by path")
            )

            (@subcommand packages =>
                (about: "Iterate all packages in a repository")
            )
            (@subcommand package_paths =>
                (name: "package-paths")
                (about: "Iterate all packages in a repository by path")
            )
            (@subcommand ebuilds =>
                (about: "Iterate all ebuilds in a repository")
            )
            (@subcommand ebuild_paths =>
                (name: "ebuild-paths")
                (about: "Iterate all ebuilds in a repository by path")
            )
        )
    )
    .get_matches();
    match app_m.subcommand() {
        ("iterate", Some(sub_m)) => match sub_m.subcommand() {
            ("categories", Some(_)) => iter_repo_categories(sub_m.value_of("REPOSITORY").unwrap()),
            ("packages", Some(_)) => iter_repo_packages(sub_m.value_of("REPOSITORY").unwrap()),
            ("ebuilds", Some(_)) => iter_repo_ebuilds(sub_m.value_of("REPOSITORY").unwrap()),
            ("category-paths", Some(_)) => iter_repo_category_paths(sub_m.value_of("REPOSITORY").unwrap()),
            ("package-paths", Some(_)) => iter_repo_package_paths(sub_m.value_of("REPOSITORY").unwrap()),
            ("ebuild-paths", Some(_)) => iter_repo_ebuild_paths(sub_m.value_of("REPOSITORY").unwrap()),

            _ => clap::Error::with_description(sub_m.usage(), clap::ErrorKind::MissingSubcommand).exit(),
        },
        _ => clap::Error::with_description(app_m.usage(), clap::ErrorKind::MissingSubcommand).exit(),
    }
}

fn iter_repo_categories(repo: &str) {
    let r = Repository::new(Path::new(repo));
    let citer = r.categories().expect("Error reading categories from repo");
    for it in OptFilter::filter_oks(citer, Category::is_legal) {
        println!("{}", it.unwrap().name());
    }
}
fn iter_repo_packages(repo: &str) {
    let r = Repository::new(Path::new(repo));
    let citer = r.packages().expect("Error reading ebuilds from repo");
    for it in citer.filter_oks(Package::is_legal) {
        println!("{}", it.unwrap().name());
    }
}
fn iter_repo_ebuilds(repo: &str) {
    let r = Repository::new(Path::new(repo));
    let citer = r.ebuilds().expect("Error reading ebuilds from repo");
    for it in citer.filter_oks(Ebuild::is_legal) {
        println!("{}", it.unwrap().name());
    }
}

fn path_conv(path: &Path) -> String { path.as_os_str().to_str().unwrap().to_string() }
fn iter_repo_category_paths(repo: &str) {
    let r = Repository::new(Path::new(repo));
    let citer = r.categories().expect("Error reading categories from repo");
    for it in citer.filter_oks(Category::is_legal) {
        println!("{}", path_conv(&it.unwrap().path()))
    }
}
fn iter_repo_package_paths(repo: &str) {
    let r = Repository::new(Path::new(repo));
    let citer = r.packages().expect("Error reading ebuilds from repo");
    for it in citer.filter_oks(Package::is_legal) {
        println!("{}", path_conv(&it.unwrap().path()))
    }
}
fn iter_repo_ebuild_paths(repo: &str) {
    let r = Repository::new(Path::new(repo));
    let citer = r.ebuilds().expect("Error reading ebuilds from repo");
    for it in citer.filter_oks(Ebuild::is_legal) {
        println!("{}", path_conv(&it.unwrap().path()))
    }
}
