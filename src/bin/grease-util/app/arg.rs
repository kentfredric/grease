pub(crate) mod formatter {
    use clap::{Arg, ArgMatches};
    pub(crate) const NAME: &str = "FORMATTER";
    pub(crate) const SHORT: &str = "f";
    pub(crate) const LONG: &str = "formatter";
    pub(crate) const HELP: &str = "Controls the presentation of output";
    pub(crate) fn arg<'x, 'y>() -> Arg<'x, 'y> {
        Arg::with_name(NAME)
            .short(SHORT)
            .long(LONG)
            .help(HELP)
            .takes_value(true)
            .possible_values(&["path", "ident", "components", "name"])
            .default_value("ident")
            .visible_alias("format")
    }
    use grease::util::repoobject;
    pub(crate) fn get(c: &ArgMatches<'_>) -> repoobject::RepoObjectFormatter {
        repoobject::parse_formatter(c.value_of(NAME).unwrap()).unwrap()
    }
}
