use petra_backend_core::to_upper_snake;
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
impl SimpleLanguageBackend for PetraRustBackend {
    fn string(name: &str, data: &str) -> Vec<u8> {
        let name = to_upper_snake(name);
        let data = format!("pub const {name}: &str = \"{data}\";\n");
        data.into_bytes()
    }

    fn number64(name: &str, data: i64) -> Vec<u8> {
        let name = to_upper_snake(name);
        let data = format!("pub const {name}: i64 = {data};\n");
        data.into_bytes()
    }

    fn line_comment(comment: &str) -> Vec<u8> {
        [b"// ", comment.as_bytes(), b"\n"].concat()
    }

    fn multi_line_comment(comment: &str) -> Vec<u8> {
        [b"/*", comment.as_bytes(), b"*/\n"].concat()
    }
}
