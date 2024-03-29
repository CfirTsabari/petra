mod configuration;

pub use configuration::PetraCSharpConfiguration;

use petra_backend_core::{format, MultiCommentFormatter, Name, SimpleLanguageBackend};
use std::io::Write;

const SPACE_4: &str = "    ";
#[derive(SimpleLanguageBackend)]
pub struct PetraCSharpBackend {
    config: PetraCSharpConfiguration,
    current_indentation: String,
    comment_writer: MultiCommentFormatter,
}

impl PetraCSharpBackend {
    #[must_use]
    pub const fn new(config: PetraCSharpConfiguration) -> Self {
        Self {
            config,
            current_indentation: String::new(),
            comment_writer: MultiCommentFormatter::new(true),
        }
    }
}

impl format::PetraFormatHeader for PetraCSharpBackend {
    fn format<T: Write>(&mut self, writer: &mut T) -> std::io::Result<()> {
        self.current_indentation = SPACE_4.into();
        writeln!(writer, "namespace {};", self.config.get_namespace_name())?;
        writeln!(writer)?;
        writeln!(
            writer,
            "public static class {}",
            self.config.get_class_name()
        )?;
        writeln!(writer, "{{")
    }
}
impl format::PetraFormatAutoGeneratedComment for PetraCSharpBackend {
    fn format<T: Write>(&mut self, writer: &mut T) -> std::io::Result<()> {
        write!(writer, "{}", configuration::AUTO_GENERATED_COMMENT)
    }
}

impl format::PetraFormatFooter for PetraCSharpBackend {
    fn format<T: Write>(&mut self, writer: &mut T) -> std::io::Result<()> {
        self.current_indentation = String::new();
        writeln!(writer, "}}")
    }
}

impl format::PetraFormatI64 for PetraCSharpBackend {
    fn format<T: Write>(&mut self, name: &Name, data: i64, writer: &mut T) -> std::io::Result<()> {
        let name: String = name.to_pascal_case();
        writeln!(
            writer,
            "{}public const long {name} = {data};",
            self.current_indentation
        )
    }
}
impl format::PetraFormatString for PetraCSharpBackend {
    fn format<T: Write>(&mut self, name: &Name, data: &str, writer: &mut T) -> std::io::Result<()> {
        let name = name.to_pascal_case();
        writeln!(
            writer,
            "{}public const string {name} = \"{data}\";",
            self.current_indentation
        )
    }
}
impl format::PetraFormatLineComment for PetraCSharpBackend {
    fn format<T: Write>(&mut self, comment: &str, writer: &mut T) -> std::io::Result<()> {
        writeln!(writer, "{}// {comment}", self.current_indentation)
    }
}
impl format::PetraFormatMultiLineComment for PetraCSharpBackend {
    fn format<T: Write>(&mut self, comment: &str, writer: &mut T) -> std::io::Result<()> {
        self.comment_writer
            .format(comment, self.current_indentation.as_str(), writer)
    }
}
