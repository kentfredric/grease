use std::io::Error;

pub trait RepoObject {
    fn name(&self) -> String;

    fn path(&self) -> std::path::PathBuf;

    fn ident(&self) -> String;

    fn components(&self) -> String;

    fn render(&self, f: &RepoObjectFormatter) -> String
    where
        Self: Sized,
    {
        match f {
            RepoObjectFormatter::Path => self.path().to_str().unwrap().to_owned(),
            RepoObjectFormatter::Ident => self.ident(),
            RepoObjectFormatter::Components => self.components(),
            RepoObjectFormatter::Name => self.name(),
            RepoObjectFormatter::Callback(myfn) => (myfn)(self),
        }
    }
}

pub enum RepoObjectFormatter {
    Path,
    Ident,
    Components,
    Name,
    Callback(fn(&RepoObject) -> String),
}

pub fn parse_formatter(f: &str) -> Result<RepoObjectFormatter, Error> {
    match f.to_ascii_lowercase().as_str() {
        "path" => Ok(RepoObjectFormatter::Path),
        "ident" => Ok(RepoObjectFormatter::Ident),
        "components" => Ok(RepoObjectFormatter::Components),
        "name" => Ok(RepoObjectFormatter::Name),
        _ => Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Parameter {} is not a valid RepoObjectFormatter", f),
        )),
    }
}
