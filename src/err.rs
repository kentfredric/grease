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
    /// The package name in question looks almost valid as per PMS regex,
    /// but fails being valid due to it having a version suffix, when version suffixes
    /// are illegal in the "package" part of an atom
    BadPackageWithPV(String),
    /// The package atom string didn't have sufficient "/" to be a valid
    /// atom
    BadAtomPair(String),
    /// The package atom string didn't satisfy the category/pn-pv regex
    BadPackageVersion(String),
}

impl fmt::Display for AtomParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AtomParseError::BadCategory(input) => write!(f, "Given category <{}> is invalid", input),
            AtomParseError::BadPackage(input) => write!(f, "Given package <{}> is invalid", input),
            AtomParseError::BadPackageWithPV(input) => {
                write!(f, "Given package <{}> is invalid due to having a version suffix", input)
            },
            AtomParseError::BadAtomPair(input) => {
                write!(f, "Given string <{}> does not have both package and category parts", input)
            },
            AtomParseError::BadPackageVersion(input) => {
                write!(f, "Given string <{}> does not have a valid package name and version", input)
            },
        }
    }
}

impl Error for AtomParseError {}
