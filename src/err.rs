/*! Error type enums
!*/

use std::{error::Error, fmt};

/** A collection of error types returnable by parsing atoms
 **/
#[derive(Debug)]
pub enum AtomParseError {
    /// The category name in question failed validation against PMS regex
    BadCategory(String),
    /// The package name in question failed validation against PMS regex
    BadPackage(String),
    /// The package atom string didn't have sufficient "/" to be a valid
    /// atom
    BadAtomPair(String),
}

impl fmt::Display for AtomParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AtomParseError::BadCategory(input) => write!(f, "Given category <{}> is invalid", input),
            AtomParseError::BadPackage(input) => write!(f, "Given package <{}> is invalid", input),
            AtomParseError::BadAtomPair(input) => {
                write!(f, "Given string <{}> does not have both package and category parts", input)
            },
        }
    }
}

impl Error for AtomParseError {}
