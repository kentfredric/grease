pub(crate) mod formatter {
    use clap::{Arg, ArgMatches};
    pub(crate) const HELP: &str = "Controls the presentation of output";
    pub(crate) const LONG: &str = "formatter";
    pub(crate) const NAME: &str = "FORMATTER";
    pub(crate) const SHORT: &str = "f";

    pub(crate) fn arg<'x, 'y>() -> Arg<'x, 'y> {
        Arg::with_name(NAME)
            .help(HELP)
            .long(LONG)
            .short(SHORT)
            .takes_value(true)
            .possible_values(&["path", "ident", "components", "name"])
            .default_value("ident")
            .visible_alias("format")
    }
    use crate::util::repoobject;
    pub(crate) fn get(c: &ArgMatches<'_>) -> repoobject::RepoObjectFormatter {
        repoobject::parse_formatter(c.value_of(NAME).unwrap()).unwrap()
    }
}

pub(crate) mod category {
    use clap::{Arg, ArgMatches, Values};
    pub(crate) const HELP: &str = "Restrict categories to a list provided";
    pub(crate) const LONG: &str = "category";
    pub(crate) const NAME: &str = "CATEGORY";
    pub(crate) const SHORT: &str = "c";
    pub(crate) const VALUE_NAME: &str = "CATEGORY_NAME";
    pub(crate) const VISIBLE_ALIAS: &str = "cat";

    pub(crate) fn arg<'x, 'y>() -> Arg<'x, 'y> {
        Arg::with_name(NAME)
            .help(HELP)
            .long(LONG)
            .short(SHORT)
            .value_name(VALUE_NAME)
            .visible_alias(VISIBLE_ALIAS)
            .takes_value(true)
            .multiple(true)
            .min_values(1)
            .require_delimiter(true)
    }
    pub(crate) fn get<'x>(command: &'x ArgMatches<'_>) -> Option<Values<'x>> {
        if command.is_present(NAME) {
            Some(command.values_of(NAME).unwrap())
        } else {
            None
        }
    }
}
pub(crate) mod repository {
    use clap::{Arg, ArgMatches};
    pub(crate) const ENV: &str = "GREASE_REPOSITORY";
    pub(crate) const HELP: &str = "Specifies the repository to iterate";
    pub(crate) const LONG: &str = "repository";
    pub(crate) const NAME: &str = "REPOSITORY";
    pub(crate) const SHORT: &str = "r";
    pub(crate) const VISIBLE_ALIAS: &str = "repo";
    pub(crate) fn arg<'x, 'y>() -> Arg<'x, 'y> {
        Arg::with_name(NAME)
            .env(ENV)
            .help(HELP)
            .long(LONG)
            .short(SHORT)
            .visible_alias(VISIBLE_ALIAS)
            .required(true)
            .takes_value(true)
    }
    pub(crate) fn get<'x>(command: &'x ArgMatches<'_>) -> &'x str {
        command.value_of(NAME).unwrap()
    }
}
