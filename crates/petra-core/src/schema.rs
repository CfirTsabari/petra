#[derive(Debug, Clone, Default)]
pub struct Document {
    pub items: Vec<TopItem>,
}
impl Document {
    #[must_use]
    pub const fn new() -> Self {
        Self { items: vec![] }
    }
}

#[derive(Debug, Clone)]
pub struct VarDeclaration {
    pub name: String,
    pub value: VarValue,
}

impl VarDeclaration {
    #[must_use]
    pub const fn new(name: String, value: VarValue) -> Self {
        Self { name, value }
    }
}
#[derive(Debug, Clone)]
pub enum VarValue {
    String(String),
    Integer64(i64),
}
#[derive(Debug, Clone)]
pub enum TopItem {
    Comment(String),
    MultiLineComment(String),
    VarDeclaration(VarDeclaration),
}
