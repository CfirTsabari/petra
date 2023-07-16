mod utils;

use petra_core::{Document, TopItem, VarDeclaration, VarValue};
use std::io::Write;
use thiserror::Error;

pub use utils::to_upper_snake;
pub trait Backend<T>
where
    T: Write,
{
    /// # Errors
    ///
    /// Will return if unable to write string to writer.
    fn translate(self: Box<Self>, document: Document, writer: T) -> Result<(), PetraBackendError>;
}

pub trait SimpleLanguageBackend {
    fn string(name: &str, data: &str) -> Vec<u8>;
    fn number64(name: &str, data: i64) -> Vec<u8>;
    fn line_comment(comment: &str) -> Vec<u8>;
    fn multi_line_comment(comment: &str) -> Vec<u8>;
}
impl<T, B> Backend<T> for B
where
    T: Write,
    B: SimpleLanguageBackend,
{
    fn translate(
        self: Box<Self>,
        document: Document,
        mut writer: T,
    ) -> Result<(), PetraBackendError> {
        for item in document.items {
            match item {
                TopItem::Comment(comment) => writer.write_all(&B::line_comment(&comment))?,
                TopItem::MultiLineComment(comment) => {
                    writer.write_all(&B::multi_line_comment(&comment))?;
                }
                TopItem::VarDeclaration(VarDeclaration {
                    name,
                    value: VarValue::Integer64(value),
                }) => writer.write_all(&B::number64(&name, value))?,
                TopItem::VarDeclaration(VarDeclaration {
                    name,
                    value: VarValue::String(value),
                }) => writer.write_all(&B::string(&name, &value))?,
            }
        }
        Ok(())
    }
}
#[derive(Error, Debug)]
pub enum PetraBackendError {
    #[error("failed writing output")]
    OutputError(#[from] std::io::Error),
    #[error("backend is not supported:{0}")]
    BackendNotSupported(String),
}
