use std::io::Write;

pub struct MultiCommentFormatter {
    allow_empty: bool,
}
impl MultiCommentFormatter {
    #[must_use]
    pub const fn new(allow_empty: bool) -> Self {
        Self { allow_empty }
    }
    /// # Errors
    ///
    /// Will return if unable to write string to Writer.
    pub fn format<T: Write>(
        &self,
        comment: &str,
        current_indentation: &str,
        writer: &mut T,
    ) -> std::io::Result<()> {
        writeln!(writer, "{current_indentation}/*")?;
        for line in comment.lines() {
            if line.trim().is_empty() {
                if self.allow_empty {
                    writeln!(writer)?;
                }
            } else {
                writeln!(writer, "{current_indentation}{line}")?;
            }
        }
        writeln!(writer, "{current_indentation}*/")
    }
}
