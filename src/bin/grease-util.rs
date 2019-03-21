/*!

# `grease-util`

## About

`grease-util` is designed as a low-level multi-command that aims to:

1. Be independent of local user configuration auto-magic, instead, preferring
  to use configuration

2. Implement only as much of PMS as is necessary to use each command correctly

3. Be simple enough to be both useful and composable in simple shell commands

4. Be fast and use minimal memory

## `iterate`

The iterate command is for iterating "kinds" of things in a given repository

```bash
  grease-util iterate --repository PATH_TO_REPO <subcommand>
```

### Subcommands

* `categories`
* `packages`
* `ebuilds`

### Common subcommand arguments

#### `--formatter FORMATTER_NAME`

* Default: `ident`

* Possible: `path`, `ident`, `components`, `name`

##### `path`

Emits the full path to the thing in question:

* `categories`: `/usr/portage/category_name`
* `packages`: `/usr/portage/category_name/package_name`
* `ebuilds`: `/usr/portage/category_name/package_name/ebuild_name.ebuild`

##### `ident`

Emits an "identity" of a thing in question

* `categories`: `category_name/`
* `packages`: `category_name/package_name`
* `ebuilds`: `category_name/package_name-pvr`

##### `name`

Emits a simplified name of the item

* `categories`: `category_name`
* `packages`: `package_name`
* `ebuilds`: `ebuild_name.ebuild`

##### `components`

Emits the thing in question as a space-delimited key=value string

* `categories`: `cat=category_name`
* `packages`: `cat=category_name package=package_name`
* `ebuilds`: `cat=category_name package=package_name version=pvr`

!*/

use clap::{clap_app, crate_authors, crate_version, ArgMatches};
use grease::{
    ebuild::Ebuild,
    package::Package,
    repository::{Category, Repository},
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
            (@arg REPOSITORY: -r --repository
                +required +takes_value
                visible_alias[repo]
                env[GREASE_REPOSITORY]
                "Specifies the repository to iterate"
            )
            (@setting SubcommandRequired)
            (@subcommand categories =>
                (about: "Iterate all categories in a repository")
                (@arg FORMATTER: -f --formatter
                    +takes_value
                    possible_value[path ident components name]
                    default_value[ident]
                    visible_alias[format]
                    "Controls the presentation of output"
                )
            )
            (@subcommand packages =>
                (about: "Iterate all packages in a repository")
                (@arg FORMATTER: -f --formatter
                    +takes_value
                    possible_value[path ident components name]
                    default_value[ident]
                    visible_alias[format]
                    "Controls the presentation of output"
                )
                (@arg CATEGORY: -c --category [CATEGORY_NAME] ...
                    +takes_value #{1,100} +require_delimiter
                    visible_alias[cat]
                    "Restrict categories to a list provided"
                )
            )
            (@subcommand ebuilds =>
                (about: "Iterate all ebuilds in a repository")
                (@arg FORMATTER: -f --formatter
                    +takes_value
                    possible_value[path ident components name]
                    default_value[ident]
                    visible_alias[format]
                    "Controls the presentation of output"
                )
                (@arg CATEGORY: -c --category [CATEGORY_NAME] ...
                    +takes_value #{1,100} +require_delimiter
                    visible_alias[cat]
                    "Restrict categories to a list provided"
                )
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

fn iter_repo_categories(repo: &str, opts: &ArgMatches<'_>) {
    let r = Repository::new(Path::new(repo));
    let citer = r.categories().expect("Error reading categories from repo");
    let formatter = repoobject::parse_formatter(opts.value_of("FORMATTER").unwrap()).unwrap();
    for it in citer.filter_oks(Category::is_legal).extract_errs(|e| panic!(e)) {
        println!("{}", it.render(&formatter));
    }
}
fn iter_repo_packages(repo: &str, opts: &ArgMatches<'_>) {
    let r = Repository::new(Path::new(repo));
    let formatter = repoobject::parse_formatter(opts.value_of("FORMATTER").unwrap()).unwrap();
    if opts.is_present("CATEGORY") {
        let values_iter = opts.values_of("CATEGORY").unwrap();
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
}
fn iter_repo_ebuilds(repo: &str, opts: &ArgMatches<'_>) {
    let r = Repository::new(Path::new(repo));
    let formatter = repoobject::parse_formatter(opts.value_of("FORMATTER").unwrap()).unwrap();

    let citer = r.ebuilds().expect("Error reading ebuilds from repo");

    if opts.is_present("CATEGORY") {
        let values_iter = opts.values_of("CATEGORY").unwrap();
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
