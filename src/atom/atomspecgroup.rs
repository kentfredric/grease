use crate::atom::AtomSpec;
use std::fmt::{self, Display};

/// A collection of atom specifications
#[derive(Debug, Clone)]
pub struct AtomSpecGroup {
    operator: OperatorKind,
    members:  Vec<MemberType>,
}

#[derive(Debug, Clone)]
enum MemberType {
    Item(Box<AtomSpec>),
    Group(Box<AtomSpecGroup>),
}

#[derive(Debug, Clone)]
enum OperatorKind {
    /// A group without an operator (implicit and)
    None,
    /// A group with an OR operator (||)
    Or,
    /// A group with an if-use operator ( use? )
    IfUse(String),
    /// A group with an if-not-use operator ( !use? )
    IfNotUse(String),
}

impl AtomSpecGroup {
    fn push(&mut self, i: MemberType) { self.members.push(i) }

    /// Add an [`AtomSpec`] to the group
    pub(crate) fn push_atomspec(&mut self, i: AtomSpec) {
        self.push(MemberType::Item(Box::new(i)))
    }

    /// Add a child [`AtomSpecGroup`] to the group
    pub(crate) fn push_group(&mut self, i: AtomSpecGroup) {
        self.push(MemberType::Group(Box::new(i)))
    }
}

impl Display for OperatorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OperatorKind::None => "".to_owned(),
                OperatorKind::Or => "||".to_owned(),
                OperatorKind::IfUse(u) => format!("{}?", u),
                OperatorKind::IfNotUse(u) => format!("!{}?", u),
            }
        )
    }
}

impl Display for AtomSpecGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.members.is_empty() {
            Ok(())
        } else {
            write!(
                f,
                "{operator}( {members} )",
                operator = match &self.operator {
                    OperatorKind::None => "".to_owned(),
                    _ => format!("{} ", &self.operator),
                },
                members = self
                    .members
                    .iter()
                    .map(|member| {
                        match member {
                            MemberType::Item(i) => i.to_string(),
                            MemberType::Group(i) => i.to_string(),
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        }
    }
}

#[test]
fn test_atom_spec_group() {
    assert_eq!(
        "",
        AtomSpecGroup { operator: OperatorKind::None, members: Vec::new() }
            .to_string()
    );
    assert_eq!(
        "( dev-lang/perl )",
        AtomSpecGroup {
            operator: OperatorKind::None,
            members:  vec!(MemberType::Item(Box::new(AtomSpec {
                category:     String::from("dev-lang"),
                package:      String::from("perl"),
                operator:     None,
                version:      None,
                revision:     None,
                slot:         None,
                slot_op:      None,
                required_use: None,
            }))),
        }
        .to_string()
    );
    assert_eq!(
        "|| ( dev-lang/perl )",
        AtomSpecGroup {
            operator: OperatorKind::Or,
            members:  vec!(MemberType::Item(Box::new(AtomSpec {
                category:     String::from("dev-lang"),
                package:      String::from("perl"),
                operator:     None,
                version:      None,
                revision:     None,
                slot:         None,
                slot_op:      None,
                required_use: None,
            }))),
        }
        .to_string()
    );
    assert_eq!(
        "foo? ( dev-lang/perl )",
        AtomSpecGroup {
            operator: OperatorKind::IfUse(String::from("foo")),
            members:  vec!(MemberType::Item(Box::new(AtomSpec {
                category:     String::from("dev-lang"),
                package:      String::from("perl"),
                operator:     None,
                version:      None,
                revision:     None,
                slot:         None,
                slot_op:      None,
                required_use: None,
            }))),
        }
        .to_string()
    );
    assert_eq!(
        "!foo? ( dev-lang/perl )",
        AtomSpecGroup {
            operator: OperatorKind::IfNotUse(String::from("foo")),
            members:  vec!(MemberType::Item(Box::new(AtomSpec {
                category:     String::from("dev-lang"),
                package:      String::from("perl"),
                operator:     None,
                version:      None,
                revision:     None,
                slot:         None,
                slot_op:      None,
                required_use: None,
            }))),
        }
        .to_string()
    );
    assert_eq!(
        "!foo? ( dev-lang/perl dev-perl/Moose )",
        AtomSpecGroup {
            operator: OperatorKind::IfNotUse(String::from("foo")),
            members:  vec!(
                MemberType::Item(Box::new(AtomSpec {
                    category:     String::from("dev-lang"),
                    package:      String::from("perl"),
                    operator:     None,
                    version:      None,
                    revision:     None,
                    slot:         None,
                    slot_op:      None,
                    required_use: None,
                })),
                MemberType::Item(Box::new(AtomSpec {
                    category:     String::from("dev-perl"),
                    package:      String::from("Moose"),
                    operator:     None,
                    version:      None,
                    revision:     None,
                    slot:         None,
                    slot_op:      None,
                    required_use: None,
                }))
            ),
        }
        .to_string()
    );
    assert_eq!(
        "!foo? ( dev-lang/perl dev-perl/Moose bar? ( dev-foo/bar ) )",
        AtomSpecGroup {
            operator: OperatorKind::IfNotUse(String::from("foo")),
            members:  vec!(
                MemberType::Item(Box::new(AtomSpec {
                    category:     String::from("dev-lang"),
                    package:      String::from("perl"),
                    operator:     None,
                    version:      None,
                    revision:     None,
                    slot:         None,
                    slot_op:      None,
                    required_use: None,
                })),
                MemberType::Item(Box::new(AtomSpec {
                    category:     String::from("dev-perl"),
                    package:      String::from("Moose"),
                    operator:     None,
                    version:      None,
                    revision:     None,
                    slot:         None,
                    slot_op:      None,
                    required_use: None,
                })),
                MemberType::Group(Box::new(AtomSpecGroup {
                    operator: OperatorKind::IfUse(String::from("bar")),
                    members:  vec!(MemberType::Item(Box::new(AtomSpec {
                        category:     String::from("dev-foo"),
                        package:      String::from("bar"),
                        operator:     None,
                        version:      None,
                        revision:     None,
                        slot:         None,
                        slot_op:      None,
                        required_use: None,
                    }))),
                }))
            ),
        }
        .to_string()
    );
}
