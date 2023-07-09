use petra_backend_core::SimpleLanguageBackend;

pub struct PetraPythonBackend {}

impl PetraPythonBackend {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for PetraPythonBackend {
    fn default() -> Self {
        Self::new()
    }
}
impl SimpleLanguageBackend for PetraPythonBackend {
    fn number(name: &str, data: i64) -> Vec<u8> {
        let name = name.to_uppercase();
        let data = format!("{name} = {data}\n");
        data.into_bytes()
    }
    fn string(name: &str, data: &str) -> Vec<u8> {
        let name = name.to_uppercase();
        let data = format!("{name} = \"{data}\"\n");
        data.into_bytes()
    }
}
