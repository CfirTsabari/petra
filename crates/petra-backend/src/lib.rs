use petra_backend_core::Backend;
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
}

/// # Panics
///
/// Will panic if backend type isn't supported
#[must_use]
pub fn get_backend<T: Write>(backend_type: &BackendType) -> Box<dyn Backend<T>> {
    match backend_type {
        BackendType::Python => Box::new(PetraPythonBackend::new()),
        BackendType::GoLang | BackendType::JavaScript | BackendType::TypeScript => {
            panic!("backend {backend_type:?} isn't supported")
        }
        BackendType::Rust => Box::new(PetraRustBackend::new()),
    }
}
