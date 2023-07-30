pub mod config;

pub use petra_backend_core::BackendConfiguration;

use petra_backend_core::Backend;
#[cfg(feature = "csharp")]
use petra_backend_csharp::PetraCSharpBackend;
#[cfg(feature = "golang")]
use petra_backend_golang::PetraGolangBackend;
#[cfg(feature = "python")]
use petra_backend_python::PetraPythonBackend;
#[cfg(feature = "rust")]
use petra_backend_rust::PetraRustBackend;

use std::io::Write;

// Define your enum
#[derive(Debug, Clone, Copy)]
pub enum BackendType {
    #[cfg(feature = "python")]
    Python,
    #[cfg(feature = "golang")]
    GoLang,
    TypeScript,
    JavaScript,
    #[cfg(feature = "rust")]
    Rust,
    #[cfg(feature = "csharp")]
    CSharp,
}

/// # Panics
///
/// Will panic if backend type isn't supported
#[must_use]
pub fn get_backend<T: Write, C: Into<config::PetraConfiguration>>(
    backend_type: &BackendType,
    configuration: C,
) -> Box<dyn Backend<T>> {
    let configuration: config::PetraConfiguration = configuration.into();
    match backend_type {
        #[cfg(feature = "python")]
        BackendType::Python => Box::new(PetraPythonBackend::new()),
        #[cfg(feature = "golang")]
        BackendType::GoLang => Box::new(PetraGolangBackend::new(configuration.golang())),
        #[cfg(feature = "csharp")]
        BackendType::CSharp => Box::new(PetraCSharpBackend::new(configuration.csharp())),
        BackendType::JavaScript | BackendType::TypeScript => {
            panic!("backend {backend_type:?} isn't supported")
        }
        #[cfg(feature = "rust")]
        BackendType::Rust => Box::new(PetraRustBackend::new()),
    }
}
