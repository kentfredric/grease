use crate::{
    atom::{regex, Category, Package},
    err,
};
use std::{cmp::Ordering, convert::From, str::FromStr};

macro_rules! chain_cmp {
    ($cmp:expr) => {
        $cmp
    };
    ($cmp:expr, $res:expr) => {
        match $cmp {
            Some(Ordering::Equal) => $res,
            e => e,
        }
    };
    ($cmp:expr, $res:expr, $resb:expr) => {
        chain_cmp!($cmp, chain_cmp!($res, $resb))
    };
    ($cmp:expr, $res:expr, $resb:expr, $resc:expr) => {
        chain_cmp!($cmp, $res, chain_cmp!($resb, $resc))
    };
}

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
*/

#[derive(Debug, Clone)]
pub struct Atom {
    category: String,
    package:  String,
    version:  String,
    revision: Option<String>,
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
        } else if !regex::CATEGORY_NAME.is_match(parts[0]) {
            return Err(AtomParseError::BadCategory(parts[0].to_owned()));
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
    fn partial_cmp(&self, other: &Atom) -> Option<Ordering> {
        chain_cmp!(other.category.partial_cmp(&self.category), Some(Ordering::Less))
    }
}

impl PartialOrd<Atom> for Package {
    fn partial_cmp(&self, other: &Atom) -> Option<Ordering> {
        chain_cmp!(other.category.partial_cmp(&self.category), Some(Ordering::Less))
    }
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
