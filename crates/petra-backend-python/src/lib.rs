use petra_backend_core::to_upper_snake;
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
    fn number64(name: &str, data: i64) -> Vec<u8> {
        let name = to_upper_snake(name);
        let data = format!("{name} = {data}\n");
        data.into_bytes()
    }
    fn string(name: &str, data: &str) -> Vec<u8> {
        let name = to_upper_snake(name);
        let data = format!("{name} = \"{data}\"\n");
        data.into_bytes()
    }

    fn line_comment(comment: &str) -> Vec<u8> {
        [b"# ", comment.as_bytes(), b"\n"].concat()
    }

    fn multi_line_comment(comment: &str) -> Vec<u8> {
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
