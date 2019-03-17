/*! Error type enums
!*/

use failure::Fail;

/** A collection of error types returnable by parsing atoms

*/
#[derive(Fail, Debug)]
#[fail(display = "An error occurred")]
pub enum AtomParseError {
    /// The category name in question failed validation against PMS regex
    #[fail(display = "Given category <{}> is invalid", _0)]
    BadCategory(String),
    /// The package name in question failed validation against PMS regex
    #[fail(display = "Given package <{}> is invalid", _0)]
    BadPackage(String),
    /// The package name in question looks almost valid as per PMS regex,
    /// but fails being valid due to it having a version suffix, when version suffixes
    /// are illegal in the "package" part of an atom
    #[fail(display = "Given package <{}> is invalid due to having a verison suffix", _0)]
    BadPackageWithPV(String),
    /// The package atom string didn't have sufficient "/" to be a valid
    /// atom
    #[fail(display = "Given string <{}> does not have both package and category parts", _0)]
    BadAtomPair(String),
    /// The package atom string didn't satisfy the category/pn-pv regex
    #[fail(display = "Given string <{}> does not have a valid package name and version", _0)]
    BadPackageVersion(String),
}
