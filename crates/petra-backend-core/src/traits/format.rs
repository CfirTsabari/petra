use petra_core::Name;
pub trait PetraFormatString {
    fn format(&self, name: &Name, data: &str) -> Vec<u8>;
}
pub trait PetraFormatI64 {
    fn format(&self, name: &Name, data: i64) -> Vec<u8>;
}
pub trait PetraFormatLineComment {
    fn format(&self, comment: &str) -> Vec<u8>;
}
pub trait PetraFormatMultiLineComment {
    fn format(&self, comment: &str) -> Vec<u8>;
}
pub trait PetraFormatHeader {
    fn format(&self) -> Option<Vec<u8>>;
}
