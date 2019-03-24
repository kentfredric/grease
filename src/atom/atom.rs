use crate::{
    atom::{regex, Category, Package},
    err,
};
use std::{
    cmp::Ordering,
    convert::From,
    fmt::{self, Display},
    str::FromStr,
};

/** A container for aspects of a Portage Atom

A portage [`Atom`] is a unique qualifier that identifies a specfic, unique package, with a specific exact version

It does not support dependency range or equality specifiers

### Usage
```rust
use grease::atom::{Atom,Category,Package};

let a: Atom = "dev-lang/perl-5.22.0-r1".parse().unwrap();
let p: Package = a.to_owned().into();
let c: Category = p.into();
let oc: Category = a.into();

```

### Construction
Creating an [`Atom`] involves parsing an input string, eg:
```rust
use grease::atom::Atom;
let a: Atom = "dev-lang/perl-5.22.0-r1".parse().unwrap();
```

This includes error handling for the parse as parse() returns a [`Result`] as per [`FromStr`]

### Conversion
[`Atom`]'s can be converted to [`Category`]'s and [`Package`]'s using the [`From`] trait, each
resulting in a loss of some data in the conversion.
```rust
use grease::atom::{Atom,Category,Package};
let a: Atom = "dev-lang/perl-5.24.1-r1".parse().unwrap();
let c: Category = a.to_owned().into();
let p: Package  = a.into();
```

### Comparison
[`Atom`]'s can be compared with other [`Atom`]'s, and even vs [`Category`] and [`Package`]'s due
to implementing [`PartialEq`] and [`PartialOrd`]

```rust
use grease::atom::{Atom,Category,Package};
assert!(   "dev-lang/perl-5.24.1-r1".parse::<Atom>().unwrap()
         < "dev-lang/perl-5.24.1-r2".parse::<Atom>().unwrap() );
assert!(   "dev-lang/perl-5.24.1".parse::<Atom>().unwrap()
         < "dev-lang/perl-5.24.1-r2".parse::<Atom>().unwrap() );
assert!(   "dev-lang/perl-5.24".parse::<Atom>().unwrap()
         < "dev-lang/perl-5.24.1-r2".parse::<Atom>().unwrap() );
assert!(   "dev-lang/perl-5".parse::<Atom>().unwrap()
         < "dev-lang/perl-5.24.1-r2".parse::<Atom>().unwrap() );
assert!(   "dev-lang/perl".parse::<Package>().unwrap()
         < "dev-lang/perl-5.24.1-r2".parse::<Atom>().unwrap() );
assert!(   "dev-lang".parse::<Category>().unwrap()
         < "dev-lang/perl-5.24.1-r2".parse::<Atom>().unwrap() );
```

Collective types (eg: [`Category`] and [`Package`]) sort "lesser" than all members of their collective, eg:
```text
c: dev-lang
p: dev-lang/perl
a: dev-lang/perl-5.21.1
```

This does **NOT** yet implement the full specification for Gentoo version sorting, however,
the naive native string sorting is apparently 90% equivalent!


*/
#[derive(Debug, Clone)]
pub struct Atom {
    pub(crate) category: String,
    pub(crate) package:  String,
    pub(crate) version:  String,
    pub(crate) revision: Option<String>,
}

impl Atom {
    /** A getter for this instances category

    */

    pub fn category(&self) -> &str { &self.category }

    /** A getter for this instances package

    */

    pub fn package(&self) -> &str { &self.package }

    /** A getter for this instances version

    */

    pub fn version(&self) -> &str { &self.version }

    /** A getter for this instances revision

    */

    pub fn revision(&self) -> Option<String> { self.revision.to_owned() }
}

impl Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}-{}", self.category, self.package, self.version).and_then(|_| match &self.revision {
            Some(rv) => write!(f, "-r{}", rv),
            None => Ok(()),
        })
    }
}

impl From<Atom> for Category {
    fn from(a: Atom) -> Self { Category { category: a.category } }
}

impl From<Atom> for Package {
    fn from(a: Atom) -> Self { Package { category: a.category, package: a.package } }
}

impl FromStr for Atom {
    type Err = err::AtomParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::err::AtomParseError;

        let parts: Vec<&str> = s.splitn(2, '/').collect();
        if parts.len() != 2 {
            return Err(AtomParseError::BadAtomPair(s.to_owned()));
        }
        let operator = match regex::ATOM_SPEC_CATEGORY.captures(parts[0]) {
            None => {
                return Err(AtomParseError::BadCategory(parts[0].to_owned()));
            },
            Some(rparts) => rparts.name("operator").map(|i| i.as_str().to_owned()),
        };
        match operator {
            None => (),
            Some(other) => match other.as_str() {
                "=" => (),
                _ => return Err(AtomParseError::BadCategory(parts[0].to_owned())),
            },
        }
        match regex::ATOM.captures(s) {
            None => Err(AtomParseError::BadPackageVersion(parts[1].to_owned())),
            Some(rparts) => Ok(Atom {
                category: rparts.name("category").map(|i| i.as_str().to_owned()).unwrap(),
                package:  rparts.name("package").map(|i| i.as_str().to_owned()).unwrap(),
                version:  rparts.name("version").map(|i| i.as_str().to_owned()).unwrap(),
                revision: rparts.name("revision").map(|i| i.as_str().to_owned()),
            }),
        }
    }
}

impl PartialEq<Category> for Atom {
    fn eq(&self, _other: &Category) -> bool { false }
}

impl PartialEq<Package> for Atom {
    fn eq(&self, _other: &Package) -> bool { false }
}

impl PartialEq<Atom> for Category {
    fn eq(&self, _other: &Atom) -> bool { false }
}

impl PartialEq<Atom> for Package {
    fn eq(&self, _other: &Atom) -> bool { false }
}
impl PartialEq for Atom {
    fn eq(&self, other: &Atom) -> bool {
        self.category == other.category
            && self.package == other.package
            && self.version == other.version
            && self.revision == other.revision
    }
}

impl PartialOrd<Category> for Atom {
    fn partial_cmp(&self, other: &Category) -> Option<Ordering> {
        chain_cmp!(self.category.partial_cmp(&other.category), Some(Ordering::Greater))
    }
}

impl PartialOrd<Package> for Atom {
    fn partial_cmp(&self, other: &Package) -> Option<Ordering> {
        chain_cmp!(
            self.category.partial_cmp(&other.category),
            self.package.partial_cmp(&other.package),
            Some(Ordering::Greater)
        )
    }
}

impl PartialOrd<Atom> for Category {
    fn partial_cmp(&self, other: &Atom) -> Option<Ordering> { other.partial_cmp(self).map(Ordering::reverse) }
}

impl PartialOrd<Atom> for Package {
    fn partial_cmp(&self, other: &Atom) -> Option<Ordering> { other.partial_cmp(self).map(Ordering::reverse) }
}

impl PartialOrd for Atom {
    fn partial_cmp(&self, other: &Atom) -> Option<Ordering> {
        chain_cmp!(
            self.category.partial_cmp(&other.category),
            self.package.partial_cmp(&other.package),
            self.version.partial_cmp(&other.version),
            self.revision.partial_cmp(&other.revision)
        )
    }
}
