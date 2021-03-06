use std::io::Write;

pub struct Item {
    pub name: String,
    pub data: ItemData,
}

impl Item {
    pub fn new(name: String, data: ItemData) -> Self {
        Item { name, data }
    }
}
pub enum ItemData {
    String(String),
    Integer(i64),
}
pub trait PetraBackend<T>
where
    T: Write,
{
    fn translate(self: Box<Self>, items: Vec<Item>, writer: T) -> Result<(), String>;
}
