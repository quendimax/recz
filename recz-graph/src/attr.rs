#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Attr {
    /// Marks a node from lazy/ungreedy iterations.
    Lazy,
}
