use petra_core::{Document, Item, ItemData};
use std::io::Read;
use std::num::ParseIntError;

#[derive(Default)]
pub struct Frontend {}
impl Frontend {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
    /// # Errors
    ///
    /// Will return error if reader doesn't represent a valid schema, or unable to read strings from reader.
    pub fn parse<R: Read>(&self, mut reader: R) -> Result<Document, String> {
        let mut res = Document::new();
        let mut data = String::new();
        reader
            .read_to_string(&mut data)
            .map_err(|e| e.to_string())?;

        for line in data.split('\n').filter(|line| !line.is_empty()) {
            let params: Vec<&str> = line.split(':').collect();
            if params.len() != 3 {
                return Err("need to have three params".into());
            }
            let (field_type, name, value) = (params[0], params[1], params[2]);
            let new_item = match field_type {
                "string" => Item::new(name.into(), ItemData::String(value.into())),
                "number" => {
                    let num: i64 = value.parse().map_err(|e: ParseIntError| e.to_string())?;
                    Item::new(name.into(), ItemData::Integer(num))
                }
                _ => return Err(format!("undefined field type :{field_type}")),
            };

            res.items.push(new_item);
        }
        Ok(res)
    }
}
