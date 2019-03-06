/*! An atom type for Gentoo packages and parsing tools
!*/

pub mod parse;
mod regex;
mod rule;
pub mod validate;

/** A container for aspects of a Portage Atom

A portage [`Atom`] is a unique qualifier that identifies a specfic, unique package, with a specific exact version

It does not support dependency range or equality specifiers
*/

#[derive(Debug)]
pub struct Atom {
    category: String,
    package:  String,
    version:  String,
    revision: Option<String>,
}

/** A container for aspects of a Portage Package

A portage [`Package`] is a unique qualifier, but without a version, and does not support range or equality specifiers
*/

#[derive(Debug)]
pub struct Package {
    category: String,
    package:  String,
}

/** A container for aspects of a Portage Category

A portage [`Category`] is a unique qualifier of a *class* of [`Package`]'s,
but without an actual package name or version and does not support range or requality specifiers
*/

#[derive(Debug)]
pub struct Category {
    category: String,
}
