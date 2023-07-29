pub use petra_backend_core::BackendConfiguration;
pub use petra_backend_csharp::PetraCSharpConfiguration;
pub use petra_backend_golang::PetraGolangConfiguration;
#[derive(Default)]
pub struct PetraConfiguration {
    golang: Option<PetraGolangConfiguration>,
    csharp: Option<PetraCSharpConfiguration>,
}
impl PetraConfiguration {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_golang(&mut self, golang: PetraGolangConfiguration) {
        self.golang = Some(golang);
    }
    pub fn set_csharp(&mut self, csharp: PetraCSharpConfiguration) {
        self.csharp = Some(csharp);
    }

    #[must_use]
    pub fn golang(self) -> PetraGolangConfiguration {
        self.golang.unwrap_or_default()
    }
    #[must_use]
    pub fn csharp(self) -> PetraCSharpConfiguration {
        self.csharp.unwrap_or_default()
    }
}
