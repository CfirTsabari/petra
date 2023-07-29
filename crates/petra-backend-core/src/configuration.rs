// Personal choice
#[allow(clippy::module_name_repetitions)]
pub struct BackendConfiguration {
    create_auto_generated_comment: bool,
}

impl BackendConfiguration {
    #[must_use]
    pub const fn new(create_auto_generated_comment: bool) -> Self {
        Self {
            create_auto_generated_comment,
        }
    }
    #[must_use]
    pub const fn create_auto_generated_comment(&self) -> bool {
        self.create_auto_generated_comment
    }
}
impl Default for BackendConfiguration {
    fn default() -> Self {
        Self::new(true)
    }
}
