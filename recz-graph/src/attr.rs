#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Attr {
    /// Marks a node from lazy/ungreedy iterations.
    Lazy,

    /// The branch ID of disjunction HIR instance.
    BranchId(String),
}
