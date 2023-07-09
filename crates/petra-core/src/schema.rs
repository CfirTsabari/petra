#[derive(Debug, Clone, Default)]
pub struct Document {
    pub items: Vec<Item>,
}
impl Document {
    #[must_use]
    pub const fn new() -> Self {
        Self { items: vec![] }
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub data: ItemData,
}

impl Item {
    #[must_use]
    pub const fn new(name: String, data: ItemData) -> Self {
        Self { name, data }
    }
}
#[derive(Debug, Clone)]
pub enum ItemData {
    String(String),
    Integer(i64),
}
