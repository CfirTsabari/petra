pub mod format;
mod helper;
pub mod simple_language;

use crate::Result;
use petra_core::Document;
use std::io::Write;

// in the future we should change the translate function to accepts &mut to allow saving a state in the formatter

pub trait Backend<T>
where
    T: Write,
{
    /// # Errors
    ///
    /// Will return if unable to write string to writer.
    fn translate(&self, document: Document, writer: T) -> Result<()>;
}
