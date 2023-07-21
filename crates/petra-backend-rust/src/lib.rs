use petra_backend_core::format;
use petra_backend_core::Name;
use petra_backend_core::SimpleLanguageBackend;

pub struct PetraRustBackend {}

impl PetraRustBackend {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for PetraRustBackend {
    fn default() -> Self {
        Self::new()
    }
}
impl SimpleLanguageBackend for PetraRustBackend {}
impl format::PetraFormatHeader for PetraRustBackend {
    fn format(&self) -> Option<Vec<u8>> {
        None
    }
}
impl format::PetraFormatI64 for PetraRustBackend {
    fn format(&self, name: &Name, data: i64) -> Vec<u8> {
        let name = name.to_upper_snake();
        let data = format!("pub const {name}: i64 = {data};\n");
        data.into_bytes()
    }
}
impl format::PetraFormatString for PetraRustBackend {
    fn format(&self, name: &Name, data: &str) -> Vec<u8> {
        let name = name.to_upper_snake();
        let data = format!("pub const {name}: &str = \"{data}\";\n");
        data.into_bytes()
    }
}
impl format::PetraFormatLineComment for PetraRustBackend {
    fn format(&self, comment: &str) -> Vec<u8> {
        [b"// ", comment.as_bytes(), b"\n"].concat()
    }
}
impl format::PetraFormatMultiLineComment for PetraRustBackend {
    fn format(&self, comment: &str) -> Vec<u8> {
        [b"/*", comment.as_bytes(), b"*/\n"].concat()
    }
}
