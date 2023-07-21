use super::format::{PetraFormatHeader, PetraFormatLineComment};
use crate::{defs, Result};
use std::io::Write;

pub trait WriteHeader<T>
where
    T: Write,
{
    fn write_header(&self, writer: &mut T) -> Result<()>;
}
impl<T, B> WriteHeader<T> for B
where
    T: Write,
    B: PetraFormatLineComment + PetraFormatHeader,
{
    fn write_header(&self, writer: &mut T) -> Result<()> {
        writer.write_all(&PetraFormatLineComment::format(
            self,
            defs::GENERATED_MESSAGE,
        ))?;
        writer.write_all(b"\n")?;
        if let Some(header) = PetraFormatHeader::format(self) {
            writer.write_all(&header)?;
        }
        Ok(())
    }
}
