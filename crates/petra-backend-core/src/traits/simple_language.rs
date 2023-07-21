use super::format::{
    PetraFormatHeader, PetraFormatI64, PetraFormatLineComment, PetraFormatMultiLineComment,
    PetraFormatString,
};
use super::helper::WriteHeader;
use crate::{Backend, Result};
use petra_core::{Document, TopItem, VarDeclaration, VarValue};
use std::io::Write;

// Personal choice
#[allow(clippy::module_name_repetitions)]
pub trait SimpleLanguageBackend:
    PetraFormatI64
    + PetraFormatLineComment
    + PetraFormatString
    + PetraFormatMultiLineComment
    + PetraFormatHeader
{
}

impl<T, B> Backend<T> for B
where
    T: Write,
    B: SimpleLanguageBackend,
{
    fn translate(&self, document: Document, mut writer: T) -> Result<()> {
        self.write_header(&mut writer)?;
        for item in document.items {
            match item {
                TopItem::Comment(comment) => {
                    writer.write_all(&PetraFormatLineComment::format(self, &comment))?;
                }
                TopItem::MultiLineComment(comment) => {
                    writer.write_all(&PetraFormatMultiLineComment::format(self, &comment))?;
                }
                TopItem::VarDeclaration(VarDeclaration {
                    name,
                    value: VarValue::Integer64(value),
                }) => writer.write_all(&PetraFormatI64::format(self, &name, value))?,
                TopItem::VarDeclaration(VarDeclaration {
                    name,
                    value: VarValue::String(value),
                }) => writer.write_all(&PetraFormatString::format(self, &name, &value))?,
            }
        }
        Ok(())
    }
}
