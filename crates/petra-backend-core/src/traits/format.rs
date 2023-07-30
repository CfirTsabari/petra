pub use petra_backend_core_derive::{
    PetraFormatAutoGeneratedComment, PetraFormatFooter, PetraFormatHeader,
};
use petra_core::Name;
use std::io::Write;

pub trait PetraFormatString {
    /// # Errors
    ///
    /// Will return if unable to write string to Writer.
    fn format<T: Write>(&mut self, name: &Name, data: &str, writer: &mut T) -> std::io::Result<()>;
}
pub trait PetraFormatI64 {
    /// # Errors
    ///
    /// Will return if unable to write string to Writer.
    fn format<T: Write>(&mut self, name: &Name, data: i64, writer: &mut T) -> std::io::Result<()>;
}
pub trait PetraFormatLineComment {
    /// # Errors
    ///
    /// Will return if unable to write string to Writer.
    fn format<T: Write>(&mut self, comment: &str, writer: &mut T) -> std::io::Result<()>;
}
pub trait PetraFormatMultiLineComment {
    /// # Errors
    ///
    /// Will return if unable to write string to Writer.
    fn format<T: Write>(&mut self, comment: &str, writer: &mut T) -> std::io::Result<()>;
}
pub trait PetraFormatHeader {
    /// # Errors
    ///
    /// Will return if unable to write string to Writer.
    fn format<T: Write>(&mut self, writer: &mut T) -> std::io::Result<()>;
}

pub trait PetraFormatFooter {
    /// # Errors
    ///
    /// Will return if unable to write string to Writer.
    fn format<T: Write>(&mut self, writer: &mut T) -> std::io::Result<()>;
}
pub trait PetraFormatAutoGeneratedComment {
    /// # Errors
    ///
    /// Will return if unable to write string to Writer.
    fn format<T: Write>(&mut self, writer: &mut T) -> std::io::Result<()>;
}
