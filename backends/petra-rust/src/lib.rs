use petra_core as pc;
use petra_core::{Item, ItemData};
use std::io::Write;

pub struct PetraRustBackend {}

impl PetraRustBackend {
    pub fn new() -> Self {
        PetraRustBackend {}
    }
}
impl Default for PetraRustBackend {
    fn default() -> Self {
        Self::new()
    }
}

fn number(name: String, data: i64) -> Vec<u8> {
    let name = name.to_uppercase();
    let data = format!("pub const {}: i64 = {};\n", name, data);
    data.into_bytes()
}
fn string(name: String, data: String) -> Vec<u8> {
    let name = name.to_uppercase();
    let data = format!("pub static {}: &str = \"{}\";\n", name, data);
    data.into_bytes()
}

impl<T: Write> pc::PetraBackend<T> for PetraRustBackend {
    fn translate(self: Box<Self>, items: Vec<Item>, mut writer: T) -> Result<(), String> {
        for item in items.into_iter() {
            match item.data {
                ItemData::String(data) => writer
                    .write_all(&string(item.name, data))
                    .map_err(|e| e.to_string())?,
                ItemData::Integer(data) => writer
                    .write_all(&number(item.name, data))
                    .map_err(|e| e.to_string())?,
            }
        }
        Ok(())
    }
}
