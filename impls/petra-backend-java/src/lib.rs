mod configuration;

pub use configuration::PetraJavaConfiguration;

use petra_backend_core::{format, MultiCommentFormatter, Name, SimpleLanguageBackend};
use std::io::Write;

// move to common place
const SPACE_2: &str = "  ";
#[derive(SimpleLanguageBackend, format::PetraFormatAutoGeneratedComment)]
pub struct PetraJavaBackend {
    config: PetraJavaConfiguration,
    current_indentation: String,
    comment_writer: MultiCommentFormatter,
}

impl PetraJavaBackend {
    #[must_use]
    pub const fn new(config: PetraJavaConfiguration) -> Self {
        Self {
            config,
            current_indentation: String::new(),
            comment_writer: MultiCommentFormatter::new(true),
        }
    }
}

impl format::PetraFormatHeader for PetraJavaBackend {
    fn format<T: Write>(&mut self, writer: &mut T) -> std::io::Result<()> {
        self.current_indentation = SPACE_2.into();
        writeln!(writer, "package {};\n", self.config.get_pkg_name())?;
        writeln!(
            writer,
            "/** {} hold const values. */",
            self.config.get_class_name()
        )?;
        writeln!(writer, "public class {} {{", self.config.get_class_name())
    }
}

impl format::PetraFormatFooter for PetraJavaBackend {
    fn format<T: Write>(&mut self, writer: &mut T) -> std::io::Result<()> {
        self.current_indentation = String::new();
        writeln!(writer, "}}")
    }
}

impl format::PetraFormatI64 for PetraJavaBackend {
    fn format<T: Write>(&mut self, name: &Name, data: i64, writer: &mut T) -> std::io::Result<()> {
        let name: String = name.to_pascal_case();
        writeln!(
            writer,
            "{}public static final long {name} = {data}L;",
            self.current_indentation
        )
    }
}
impl format::PetraFormatString for PetraJavaBackend {
    fn format<T: Write>(&mut self, name: &Name, data: &str, writer: &mut T) -> std::io::Result<()> {
        let name = name.to_pascal_case();
        writeln!(
            writer,
            "{}public static final String {name} = \"{data}\";",
            self.current_indentation
        )
    }
}
impl format::PetraFormatLineComment for PetraJavaBackend {
    fn format<T: Write>(&mut self, comment: &str, writer: &mut T) -> std::io::Result<()> {
        writeln!(writer, "{}// {comment}", self.current_indentation)
    }
}
impl format::PetraFormatMultiLineComment for PetraJavaBackend {
    fn format<T: Write>(&mut self, comment: &str, writer: &mut T) -> std::io::Result<()> {
        self.comment_writer
            .format(comment, self.current_indentation.as_str(), writer)
    }
}
