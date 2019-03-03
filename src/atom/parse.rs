/*! Convert strings into `atom` objects
!*/

use crate::{
    atom::{validate, Category},
    err::AtomParseError,
};

#[test]
fn parse_category() {
    match category("dev-perl") {
        Ok(c) => assert_eq!(c.category, "dev-perl"),
        Err(_) => assert!(false),
    }
    match category("virtual") {
        Ok(c) => assert_eq!(c.category, "virtual"),
        Err(_) => assert!(false),
    }
    match category("-invalid") {
        Ok(c) => assert_ne!(c.category, "-invalid"),
        Err(e) => match e {
            AtomParseError::BadCategory(c) => assert_eq!(c, "-invalid"),
            AtomParseError::BadPackage(_) => assert!(false),
        },
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
