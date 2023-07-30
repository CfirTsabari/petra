pub mod format;
mod helper;
pub mod simple_language;

use crate::BackendConfiguration;
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
    /// Will return if unable to write string to formatter.
    fn format(
        &mut self,
        config: &BackendConfiguration,
        document: &Document,
        writer: &mut T,
    ) -> Result<()>;
}
