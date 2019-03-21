/*! A Low Level utility kit for performing tasks with Gentoo Portage
!*/

#[macro_use]
mod macros;

pub mod atom;
pub mod err;

/// Representation of an ebuild in a Gentoo repository
pub mod ebuild;
/// Representation of a Gentoo repository
pub mod repository;
/// Common utilities
pub mod util;
/// version objects and parsing
pub mod version;
