pub use petra_backend_golang::PetraGolangConfiguration;
pub struct PetraConfiguration {
    golang: Option<PetraGolangConfiguration>,
}
impl PetraConfiguration {
    #[must_use]
    pub const fn new() -> Self {
        Self { golang: None }
    }
    pub fn set_golang(&mut self, golang: PetraGolangConfiguration) {
        self.golang = Some(golang);
    }
    #[must_use]
    pub fn golang(self) -> PetraGolangConfiguration {
        self.golang.unwrap_or_default()
    }
}
impl Default for PetraConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
