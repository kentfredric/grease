pub mod util {
    pub(crate) mod arg;
    pub(crate) mod iterate;
    pub(crate) mod parse_atom;
    pub(crate) mod print_cache;

    use clap::{
        crate_authors, crate_version, App, AppSettings, ArgMatches, Error,
        ErrorKind,
    };

    pub(crate) const NAME: &str = "grease-util";
    pub(crate) const ABOUT: &str = "Low level utility portage multi-tool";

    pub fn app<'x, 'y>() -> App<'x, 'y> {
        App::new(NAME)
            .version(crate_version!())
            .author(crate_authors!())
            .about(ABOUT)
            .setting(AppSettings::SubcommandRequired)
            .setting(AppSettings::VersionlessSubcommands)
            .subcommand(iterate::subcommand())
            .subcommand(parse_atom::subcommand())
            .subcommand(print_cache::subcommand())
    }

    pub fn run(command: &ArgMatches<'_>) -> Result<(), Error> {
        match command.subcommand() {
            (iterate::NAME, Some(sub_c)) => iterate::run(sub_c),
            (parse_atom::NAME, Some(sub_c)) => parse_atom::run(sub_c),
            (print_cache::NAME, Some(sub_c)) => print_cache::run(sub_c),
            _ => Err(Error::with_description(
                command.usage(),
                ErrorKind::MissingSubcommand,
            )),
        }
    }
}
