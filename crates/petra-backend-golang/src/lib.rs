mod configuration;
use petra_backend_core::format;
use petra_backend_core::Name;
use petra_backend_core::SimpleLanguageBackend;

pub use configuration::PetraGolangConfiguration;

pub struct PetraGolangBackend {
    config: PetraGolangConfiguration,
}

impl PetraGolangBackend {
    #[must_use]
    pub const fn new(config: PetraGolangConfiguration) -> Self {
        Self { config }
    }
}

impl SimpleLanguageBackend for PetraGolangBackend {}
impl format::PetraFormatHeader for PetraGolangBackend {
    fn format(&self) -> Option<Vec<u8>> {
        Some(format!("package {}\n\n", self.config.get_package_name()).into_bytes())
    }
}
impl format::PetraFormatI64 for PetraGolangBackend {
    fn format(&self, name: &Name, data: i64) -> Vec<u8> {
        let name = name.to_pascal_case();
        let data = format!("const {name} = {data}\n");
        data.into_bytes()
    }
}
impl format::PetraFormatString for PetraGolangBackend {
    fn format(&self, name: &Name, data: &str) -> Vec<u8> {
        let name = name.to_pascal_case();
        let data = format!("const {name} = \"{data}\"\n");
        data.into_bytes()
    }
}
impl format::PetraFormatLineComment for PetraGolangBackend {
    fn format(&self, comment: &str) -> Vec<u8> {
        [b"// ", comment.as_bytes(), b"\n"].concat()
    }
}
impl format::PetraFormatMultiLineComment for PetraGolangBackend {
    fn format(&self, comment: &str) -> Vec<u8> {
        // this might be improved a bit in the future.
        let mut res: Vec<Vec<u8>> = vec![b"\n/*\n".to_vec()];
        for line in comment.lines().filter(|&x| !x.trim().is_empty()) {
            res.push(line.as_bytes().to_vec());
            res.push(b"\n".to_vec());
        }
        res.push(b"*/\n".to_vec());
        res.concat()
    }
}
