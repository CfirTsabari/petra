pub use petra_backend_core::BackendConfiguration;
#[cfg(feature = "lang_cpp")]
pub use petra_backend_cpp::PetraCppConfiguration;
#[cfg(feature = "lang_csharp")]
pub use petra_backend_csharp::PetraCSharpConfiguration;
#[cfg(feature = "lang_golang")]
pub use petra_backend_golang::PetraGolangConfiguration;
#[cfg(feature = "lang_java")]
pub use petra_backend_java::PetraJavaConfiguration;

#[derive(Default)]
pub struct PetraConfiguration {
    #[cfg(feature = "lang_golang")]
    golang: Option<PetraGolangConfiguration>,
    #[cfg(feature = "lang_csharp")]
    csharp: Option<PetraCSharpConfiguration>,
    #[cfg(feature = "lang_java")]
    java: Option<PetraJavaConfiguration>,
    #[cfg(feature = "lang_cpp")]
    cpp: Option<PetraCppConfiguration>,
}
impl PetraConfiguration {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
#[cfg(feature = "lang_golang")]
impl PetraConfiguration {
    pub fn set_golang(&mut self, golang: PetraGolangConfiguration) {
        self.golang = Some(golang);
    }

    #[must_use]
    pub fn golang(self) -> PetraGolangConfiguration {
        self.golang.unwrap_or_default()
    }
}
#[cfg(feature = "lang_csharp")]
impl PetraConfiguration {
    #[must_use]
    pub fn csharp(self) -> PetraCSharpConfiguration {
        self.csharp.unwrap_or_default()
    }
    pub fn set_csharp(&mut self, csharp: PetraCSharpConfiguration) {
        self.csharp = Some(csharp);
    }
}
#[cfg(feature = "lang_java")]
impl PetraConfiguration {
    pub fn set_java(&mut self, java: PetraJavaConfiguration) {
        self.java = Some(java);
    }

    #[must_use]
    pub fn java(self) -> PetraJavaConfiguration {
        self.java.unwrap_or_default()
    }
}
#[cfg(feature = "lang_cpp")]
impl PetraConfiguration {
    pub fn set_cpp(&mut self, cpp: PetraCppConfiguration) {
        self.cpp = Some(cpp);
    }

    #[must_use]
    pub fn cpp(self) -> PetraCppConfiguration {
        self.cpp.unwrap_or_default()
    }
}
