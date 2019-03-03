/*! Convert strings into `atom` objects
!*/

use crate::{
    atom::{validate, Category},
    err::AtomParseError,
};

/** Decode a string into a [`Category`]

**/
pub fn category(c: &str) -> Result<Category, AtomParseError> {
    if validate::category_name(c) {
        Ok(Category { category: c.to_owned() })
    } else {
        unimplemented!()
    }
}
