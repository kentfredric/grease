use std::io::Error;

/// Shared mechanims for iterable entities in a gentoo repository
pub trait RepoObject {
    /// Return a string containing the objects lowest level name
    fn name(&self) -> String;

    /// Return a string locating the path to the object
    fn path(&self) -> std::path::PathBuf;

    /// Return a string uniquely describing the object
    fn ident(&self) -> String;

    /// Return a string describing the objects components
    fn components(&self) -> String;

    /// Render the attached trait object using the specified mechanism
    fn render(&self, f: &RepoObjectFormatter) -> String
    where
        Self: Sized,
    {
        match f {
            RepoObjectFormatter::Path => {
                self.path().to_str().unwrap().to_owned()
            },
            RepoObjectFormatter::Ident => self.ident(),
            RepoObjectFormatter::Components => self.components(),
            RepoObjectFormatter::Name => self.name(),
            RepoObjectFormatter::Callback(myfn) => (myfn)(self),
        }
    }
}

/// A collection of enums that control behaviour of a repo-objects
/// renderer mechanism
#[derive(Copy, Clone)]
pub enum RepoObjectFormatter {
    /// Format as a path to the object in question
    Path,
    /// Format as a unique identifier
    Ident,
    /// Format as a string describing the objects components
    Components,
    /// Format as the objects name
    Name,
    /// Format via custom callback
    Callback(fn(&dyn RepoObject) -> String),
}

impl std::fmt::Debug for RepoObjectFormatter {
    fn fmt(
        &self, f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        match self {
            RepoObjectFormatter::Path => {
                write!(f, "RepoObjectFormatter<Path>")
            },
            RepoObjectFormatter::Ident => {
                write!(f, "RepoObjectFormatter<Ident>")
            },
            RepoObjectFormatter::Components => {
                write!(f, "RepoObjectFormatter<Components>")
            },
            RepoObjectFormatter::Name => {
                write!(f, "RepoObjectFormatter<Name>")
            },
            RepoObjectFormatter::Callback(_) => {
                write!(f, "RepoObjectFormatter<Callback(fn)>")
            },
        }
    }
}

/// Convert a string to a RepoObjectFormatter Enum
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
