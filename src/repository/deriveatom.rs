//! Traits for deriving repository objects from atom objects

/// A trait for declaring that `<self>` can produce a `::repository::<thing>`
/// by using a `::atom::<thing>`
pub trait DeriveAtom<Atom, Target> {
    /// Yield a kind of `Target` by consuming a kind of `Atom`
    fn derive(&self, a: Atom) -> Target;
}
