//! A representation of a gentoo atom specification

/// A Use flag constraint on a Gentoo Atom Specification
#[derive(Debug, Clone)]
pub struct UseSpec {
    pub(crate) modifier: Option<String>,
    pub(crate) flag:     String,
    pub(crate) suffix:   Option<String>,
}

/// A Gentoo Atom Specification
#[derive(Debug, Clone)]
pub struct AtomSpec {
    pub(crate) category: String,
    pub(crate) package:  String,

    pub(crate) operator:     Option<String>,
    pub(crate) version:      Option<String>,
    pub(crate) revision:     Option<String>,
    pub(crate) slot:         Option<String>,
    pub(crate) slot_op:      Option<String>,
    pub(crate) required_use: Option<Vec<UseSpec>>,
}

use crate::{
    atom::{Atom, Category, Package},
    err,
};
use std::{
    cmp::Ordering,
    fmt::{self, Display},
    str::FromStr,
    string::ToString,
};

impl Display for UseSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.modifier.to_owned().unwrap_or_else(|| "".to_owned()),
            self.flag,
            self.suffix.to_owned().unwrap_or_else(|| "".to_owned())
        )
    }
}
impl Display for AtomSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{operator}{category}/{package}{version}{slot}{useflags}",
            operator =
                self.operator.to_owned().unwrap_or_else(|| "".to_owned()),
            category = self.category,
            package = self.package,
            version = self.version.to_owned().map_or_else(
                || "".to_owned(),
                |v| {
                    self.revision.to_owned().map_or_else(
                        || format!("-{}", &v),
                        |rv| format!("-{}-r{}", &v, &rv),
                    )
                }
            ),
            slot = self.slot.to_owned().map_or_else(
                || "".to_owned(),
                |s| {
                    self.slot_op.to_owned().map_or_else(
                        || format!(":{}", s),
                        |op| format!(":{}{}", s, op),
                    )
                }
            ),
            useflags = self.required_use.to_owned().map_or_else(
                || "".to_owned(),
                |uf| {
                    if uf.is_empty() {
                        "".to_owned()
                    } else {
                        format!(
                            "[{}]",
                            uf.iter()
                                .map(ToString::to_string)
                                .collect::<Vec<String>>()
                                .join(",")
                        )
                    }
                }
            )
        )
    }
}
impl FromStr for UseSpec {
    type Err = err::AtomParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::atom::regex;
        match regex::USE_FLAG_SPEC.captures(s) {
            None => unimplemented!(),
            Some(rparts) => Ok(UseSpec {
                modifier: rparts
                    .name("prefix")
                    .map(|i| i.as_str().to_owned()),
                flag:     rparts
                    .name("flag")
                    .map(|i| i.as_str().to_owned())
                    .unwrap(),
                suffix:   rparts
                    .name("suffix")
                    .map(|i| i.as_str().to_owned()),
            }),
        }
    }
}

impl FromStr for AtomSpec {
    type Err = err::AtomParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::{atom::regex, err::AtomParseError};

        let parts: Vec<&str> = s.splitn(2, '/').collect();
        if parts.len() != 2 {
            return Err(AtomParseError::BadAtomPair(s.to_owned()));
        }
        let (operator, category) = match regex::ATOM_SPEC_CATEGORY
            .captures(parts[0])
        {
            None => {
                return Err(AtomParseError::BadCategory(parts[0].to_owned()));
            },
            Some(rparts) => (
                rparts.name("operator").map(|i| i.as_str().to_owned()),
                rparts.name("category").map(|i| i.as_str().to_owned()),
            ),
        };
        let (package, version, revision, slot, slot_op, required_use) =
            match regex::ATOM_SPEC_PNV.captures(parts[1]) {
                None => {
                    return Err(AtomParseError::BadPackageVersion(
                        parts[1].to_owned(),
                    ));
                },
                Some(rparts) => {
                    let req_use = match rparts.name("use_flags") {
                        Some(i) => {
                            let iparts: Vec<&str> =
                                i.as_str().split(',').collect();
                            let mut oparts: Vec<UseSpec> =
                                Vec::with_capacity(iparts.len());
                            for p in iparts {
                                match p.parse::<UseSpec>() {
                                    Err(e) => return Err(e),
                                    Ok(x) => oparts.push(x),
                                }
                            }
                            Some(oparts)
                        },
                        None => None,
                    };
                    (
                        rparts.name("package").map(|i| i.as_str().to_owned()),
                        rparts.name("version").map(|i| i.as_str().to_owned()),
                        rparts
                            .name("revision")
                            .map(|i| i.as_str().to_owned()),
                        rparts.name("slot").map(|i| i.as_str().to_owned()),
                        rparts.name("slot_op").map(|i| i.as_str().to_owned()),
                        req_use,
                    )
                },
            };
        match (&operator, &version) {
            (Some(my_op), None) => {
                return Err(AtomParseError::IllegalOperator(
                    my_op.to_owned(),
                    parts[0].to_owned(),
                    s.to_owned(),
                ))
            },
            (None, Some(my_version)) => {
                return Err(AtomParseError::IllegalVersion(
                    my_version.to_owned(),
                    parts[1].to_owned(),
                    s.to_owned(),
                ))
            },
            _ => (),
        }
        Ok(AtomSpec {
            category: category.unwrap(),
            package: package.unwrap(),
            version,
            operator,
            required_use,
            revision,
            slot,
            slot_op,
        })
    }
}

impl From<AtomSpec> for Category {
    fn from(a: AtomSpec) -> Self { Category { category: a.category } }
}

impl From<AtomSpec> for Package {
    fn from(a: AtomSpec) -> Self {
        Package { category: a.category, package: a.package }
    }
}
impl PartialEq<Category> for AtomSpec {
    fn eq(&self, _other: &Category) -> bool { false }
}

impl PartialEq<Package> for AtomSpec {
    fn eq(&self, _other: &Package) -> bool { false }
}

impl PartialEq<Atom> for AtomSpec {
    fn eq(&self, _other: &Atom) -> bool { false }
}

impl PartialEq<AtomSpec> for Category {
    fn eq(&self, _other: &AtomSpec) -> bool { false }
}

impl PartialEq<AtomSpec> for Package {
    fn eq(&self, _other: &AtomSpec) -> bool { false }
}

impl PartialEq<AtomSpec> for Atom {
    fn eq(&self, _other: &AtomSpec) -> bool { false }
}

impl PartialEq for AtomSpec {
    fn eq(&self, other: &AtomSpec) -> bool {
        self.category == other.category && self.package == other.package
    }
}

impl PartialOrd<Category> for AtomSpec {
    fn partial_cmp(&self, other: &Category) -> Option<Ordering> {
        chain_cmp!(
            self.category.partial_cmp(&other.category),
            Some(Ordering::Greater)
        )
    }
}
impl PartialOrd<Package> for AtomSpec {
    fn partial_cmp(&self, other: &Package) -> Option<Ordering> {
        chain_cmp!(
            self.category.partial_cmp(&other.category),
            self.package.partial_cmp(&other.package),
            Some(Ordering::Greater)
        )
    }
}
impl PartialOrd<Atom> for AtomSpec {
    fn partial_cmp(&self, other: &Atom) -> Option<Ordering> {
        // Cat < Package < Atom < AtomSpec
        chain_cmp!(
            self.category.partial_cmp(&other.category),
            self.package.partial_cmp(&other.package),
            // Atoms with greater versions sort after this
            // but otherwise, Atoms all sort before this
            // Atoms with equal versions continue comparing
            match &self.version {
                Some(v) => v.partial_cmp(&other.version),
                None => Some(Ordering::Less),
            },
            // If versions are equal, then revision compare
            // if one has a revision, the one with the revision is greater
            // if both lack revisions, the AtomSpec comes last
            match (&self.revision, &other.revision) {
                (Some(rv), Some(orv)) => rv.partial_cmp(&orv),
                (Some(_), None) => Some(Ordering::Greater),
                (None, Some(_)) => Some(Ordering::Less),
                _ => Some(Ordering::Greater),
            },
            Some(Ordering::Greater)
        )
    }
}

impl PartialOrd<AtomSpec> for Category {
    fn partial_cmp(&self, other: &AtomSpec) -> Option<Ordering> {
        other.partial_cmp(self).map(Ordering::reverse)
    }
}

impl PartialOrd<AtomSpec> for Package {
    fn partial_cmp(&self, other: &AtomSpec) -> Option<Ordering> {
        other.partial_cmp(self).map(Ordering::reverse)
    }
}
impl PartialOrd<AtomSpec> for Atom {
    fn partial_cmp(&self, other: &AtomSpec) -> Option<Ordering> {
        other.partial_cmp(self).map(Ordering::reverse)
    }
}

impl PartialOrd for AtomSpec {
    fn partial_cmp(&self, other: &AtomSpec) -> Option<Ordering> {
        // Cat < Package < Atom < AtomSpec
        chain_cmp!(
            self.category.partial_cmp(&other.category),
            self.package.partial_cmp(&other.package),
            // Can't sort with other AtomSpec's with same <cat>/<pn>
            None
        )
    }
}
