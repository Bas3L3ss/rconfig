#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SpannedKey {
    pub name: String,
    pub line: usize,
}
