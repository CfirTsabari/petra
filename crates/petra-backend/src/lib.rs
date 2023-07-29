pub mod config;

use petra_backend_core::Backend;
use petra_backend_csharp::PetraCSharpBackend;
use petra_backend_golang::PetraGolangBackend;
use petra_backend_python::PetraPythonBackend;
use petra_backend_rust::PetraRustBackend;
use std::io::Write;

// Define your enum
#[derive(Debug, Clone, Copy)]
pub enum BackendType {
    Python,
    GoLang,
    TypeScript,
    JavaScript,
    Rust,
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
        BackendType::Python => Box::new(PetraPythonBackend::new()),
        BackendType::GoLang => Box::new(PetraGolangBackend::new(configuration.golang())),
        BackendType::CSharp => Box::new(PetraCSharpBackend::new(configuration.csharp())),
        BackendType::JavaScript | BackendType::TypeScript => {
            panic!("backend {backend_type:?} isn't supported")
        }
        BackendType::Rust => Box::new(PetraRustBackend::new()),
    }
}
