use crate::atom::AtomSpec;

/// A collection of atom specifications
#[derive(Debug,Clone)]
pub struct AtomSpecGroup {
    operator: OperatorKind,
    members: Vec<MemberType>
}

#[derive(Debug,Clone)]
enum MemberType {
    Item(Box<AtomSpec>),
    Group(Box<AtomSpecGroup>),
}

#[derive(Debug,Clone)]
enum OperatorKind {
    /// A group without an operator (implicit and)
    None,
    /// A group with an OR operator (||)
    Or,
    /// A group with an if-use operator ( use? )
    IfUse(String),
    /// A group with an if-not-use operator ( !use? )
    IfNotUse(String)
}

impl AtomSpecGroup {

    fn push(&mut self, i: MemberType) {
        self.members.push(i)
    }
    /// Add an [`AtomSpec`] to the group
    pub(crate) fn push_atomspec(&mut self, i: AtomSpec) {
        self.push(MemberType::Item(Box::new(i)))
    }
    /// Add a child [`AtomSpecGroup`] to the group
    pub(crate) fn push_group(&mut self, i: AtomSpecGroup) {
        self.push(MemberType::Group(Box::new(i)))
    }
}

