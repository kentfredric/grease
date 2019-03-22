/*! An abstract atom type for Gentoo packages and parsing tools

The set of atom types permits and facilliates working with portage dependency tokens in an abstract manner
independent of direct, legal, filesystem representation.

This is an adjunct to its peers in [`repository`]

### Usage
```rust
use grease::atom::{Atom, Category, Package};

// Parse an atom string and return a structured representation
// or throw a parse error
let a: Atom = "dev-lang/perl-5.26.0-r1".parse().unwrap();

// Duplicate the Atom struct and convert it into a Category object
let c: Category = a.to_owned().into();

!*/

pub(crate) mod atom;
pub(crate) mod atomspec;
pub(crate) mod category;
pub(crate) mod package;
pub(crate) mod regex;
pub(crate) mod rule;

pub use atom::Atom;
pub use atomspec::{AtomSpec, UseSpec};
pub use category::Category;
pub use package::Package;
