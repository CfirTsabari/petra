mod parser;

use petra_core::Document;
use std::io::Read;

pub use parser::PetraFrontendError;

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
    pub fn parse<R: Read>(&self, reader: R) -> Result<Document, PetraFrontendError> {
        parser::parse(reader)
    }
}
