use petra_backend_core::format;
use petra_backend_core::Name;
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
impl format::PetraFormatHeader for PetraPythonBackend {
    fn format(&self) -> Option<Vec<u8>> {
        None
    }
}
impl format::PetraFormatI64 for PetraPythonBackend {
    fn format(&self, name: &Name, data: i64) -> Vec<u8> {
        let name = name.to_upper_snake();
        let data = format!("{name} = {data}\n");
        data.into_bytes()
    }
}
impl format::PetraFormatString for PetraPythonBackend {
    fn format(&self, name: &Name, data: &str) -> Vec<u8> {
        let name = name.to_upper_snake();
        let data = format!("{name} = \"{data}\"\n");
        data.into_bytes()
    }
}
impl format::PetraFormatLineComment for PetraPythonBackend {
    fn format(&self, comment: &str) -> Vec<u8> {
        [b"# ", comment.as_bytes(), b"\n"].concat()
    }
}
impl format::PetraFormatMultiLineComment for PetraPythonBackend {
    fn format(&self, comment: &str) -> Vec<u8> {
        let res: Vec<Vec<u8>> = comment
            .lines()
            .map(|line: &str| {
                if line.trim().is_empty() {
                    b"#\n".to_vec()
                } else {
                    [b"# ", line.as_bytes(), b"\n"].concat()
                }
            })
            .collect();
        res.concat()
    }
}

impl SimpleLanguageBackend for PetraPythonBackend {}
