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
