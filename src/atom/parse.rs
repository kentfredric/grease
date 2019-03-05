/*! Convert strings into `atom` objects
!*/

use crate::{
    atom::{validate, Category, Package},
    err::AtomParseError,
};

#[test]
fn parse_category() {
    match category("dev-perl") {
        Ok(c) => assert_eq!(c.category, "dev-perl"),
        e => panic!("{:?}", e),
    }
    match category("virtual") {
        Ok(c) => assert_eq!(c.category, "virtual"),
        e => panic!("{:?}", e),
    }
    match category("-invalid") {
        Err(AtomParseError::BadCategory(c)) => assert_eq!(c, "-invalid"),
        e => panic!("{:?}", e),
    }
}

/** Decode a string into a [`Category`]

**/
pub fn category(c: &str) -> Result<Category, AtomParseError> {
    if validate::category_name(c) {
        Ok(Category { category: c.to_owned() })
    } else {
        Err(AtomParseError::BadCategory(c.to_owned()))
    }
}

#[test]
fn parse_package() {
    match package("dev-perl") {
        Err(AtomParseError::BadAtomPair(e)) => assert_eq!(e, "dev-perl"),
        e => panic!("{:?}", e),
    }
    match package("virtual") {
        Err(AtomParseError::BadAtomPair(e)) => assert_eq!(e, "virtual"),
        e => panic!("{:?}", e),
    }
    match package("-invalid") {
        Err(AtomParseError::BadAtomPair(e)) => assert_eq!(e, "-invalid"),
        e => panic!("{:?}", e),
    }
    match package("-invalid/name") {
        Err(AtomParseError::BadCategory(e)) => assert_eq!(e, "-invalid"),
        e => panic!("{:?}", e),
    }
    match package("virtual/-invalid") {
        Err(AtomParseError::BadPackage(e)) => assert_eq!(e, "-invalid"),
        e => panic!("{:?}", e),
    }
    match package("virtual/valid") {
        Ok(Package { category: c, package: p }) => {
            assert_eq!(c, "virtual");
            assert_eq!(p, "valid");
        },
        e => panic!("{:?}", e),
    }
    // Versions in package names are illegal
    match package("virtual/valid-1") {
        Err(AtomParseError::BadPackage(e)) => assert_eq!(e, "valid-1"),
        e => panic!("{:?}", e),
    }
}

/** Decode a string containing a category and package name pair into a [`Package`]

*/
pub fn package(c: &str) -> Result<Package, AtomParseError> {
    let parts: Vec<&str> = c.splitn(2, '/').collect();
    if parts.len() != 2 {
        Err(AtomParseError::BadAtomPair(c.to_owned()))
    } else if !validate::category_name(parts[0]) {
        Err(AtomParseError::BadCategory(parts[0].to_owned()))
    } else if !validate::package_name(parts[1]) {
        Err(AtomParseError::BadPackage(parts[1].to_owned()))
    } else {
        Ok(Package { category: parts[0].to_owned(), package: parts[1].to_owned() })
    }
}
