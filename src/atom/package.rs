/** A container for aspects of a Portage Package

A portage [`Package`] is a unique qualifier, but without a version, and does not support range or equality specifiers

### Usage
```rust
use grease::atom::{Category,Package};

let p: Package = "dev-lang/perl".parse().unwrap();
let c: Category = p.into();

```
*/

#[derive(Debug, Clone)]
pub struct Package {
    pub(crate) category: String,
    pub(crate) package:  String,
}

use crate::{
    atom::{regex, Category},
    err,
};
use std::{
    cmp::Ordering,
    convert::From,
    fmt::{self, Display},
    str::FromStr,
};

impl Package {
    /** A getter for this instances category

    */
    pub fn category(&self) -> &str { &self.category }

    /** A getter for this instances package

    */
    pub fn package(&self) -> &str { &self.package }
}

impl Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.category, self.package)
    }
}

impl From<Package> for Category {
    fn from(p: Package) -> Self { Category { category: p.category } }
}

impl FromStr for Package {
    type Err = err::AtomParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::err::AtomParseError;

        let parts: Vec<&str> = s.splitn(2, '/').collect();
        if parts.len() != 2 {
            Err(AtomParseError::BadAtomPair(s.to_owned()))
        } else if !regex::CATEGORY_NAME.is_match(parts[0]) {
            Err(AtomParseError::BadCategory(parts[0].to_owned()))
        } else if !regex::PACKAGE_NAME.is_match(parts[1]) {
            Err(AtomParseError::BadPackage(parts[1].to_owned()))
        } else if regex::VERSION_SUFFIX.is_match(parts[1]) {
            Err(AtomParseError::BadPackageWithPV(parts[1].to_owned()))
        } else {
            Ok(Package {
                category: parts[0].to_owned(),
                package:  parts[1].to_owned(),
            })
        }
    }
}

impl PartialEq<Category> for Package {
    fn eq(&self, _other: &Category) -> bool { false }
}
impl PartialEq<Package> for Category {
    fn eq(&self, _other: &Package) -> bool { false }
}

impl PartialOrd<Category> for Package {
    fn partial_cmp(&self, other: &Category) -> Option<Ordering> {
        chain_cmp!(
            self.category.partial_cmp(&other.category),
            Some(Ordering::Greater)
        )
    }
}
impl PartialOrd<Package> for Category {
    fn partial_cmp(&self, other: &Package) -> Option<Ordering> {
        other.partial_cmp(self).map(Ordering::reverse)
    }
}

impl PartialEq for Package {
    fn eq(&self, other: &Package) -> bool {
        self.category == other.category && self.package == other.package
    }
}

impl PartialOrd for Package {
    fn partial_cmp(&self, other: &Package) -> Option<Ordering> {
        chain_cmp!(
            self.category.partial_cmp(&other.category),
            self.package.partial_cmp(&other.package)
        )
    }
}
