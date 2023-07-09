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
    fn number(name: &str, data: i64) -> Vec<u8> {
        let name = name.to_uppercase();
        let data = format!("pub const {name}: i64 = {data};\n");
        data.into_bytes()
    }
    fn string(name: &str, data: &str) -> Vec<u8> {
        let name = name.to_uppercase();
        let data = format!("pub static {name}: &str = \"{data}\";\n");
        data.into_bytes()
    }
}
