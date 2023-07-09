use petra_core::{Item, ItemData};
use std::io::Write;

pub trait Backend<T>
where
    T: Write,
{
    /// # Errors
    ///
    /// Will return if unable to write string to writer.
    fn translate(self: Box<Self>, items: Vec<Item>, writer: T) -> Result<(), String>;
}

pub trait SimpleLanguageBackend {
    fn string(name: &str, data: &str) -> Vec<u8>;
    fn number(name: &str, data: i64) -> Vec<u8>;
}
impl<T, B> Backend<T> for B
where
    T: Write,
    B: SimpleLanguageBackend,
{
    fn translate(self: Box<Self>, items: Vec<Item>, mut writer: T) -> Result<(), String> {
        for item in items {
            match item.data {
                ItemData::String(data) => writer
                    .write_all(&B::string(&item.name, &data))
                    .map_err(|e| e.to_string())?,
                ItemData::Integer(data) => writer
                    .write_all(&B::number(&item.name, data))
                    .map_err(|e| e.to_string())?,
            }
        }
        Ok(())
    }
}
