//! Internal Format

#[derive(Debug, Clone)]
pub enum Relation<L, R> {
    Increases(L, R),
    Decreases(L, R),
}

