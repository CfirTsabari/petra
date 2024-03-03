pub mod config;

pub use petra_backend_core::BackendConfiguration;

use petra_backend_core::Backend;
#[cfg(feature = "lang_cpp")]
use petra_backend_cpp::PetraCppBackend;
#[cfg(feature = "lang_csharp")]
use petra_backend_csharp::PetraCSharpBackend;
#[cfg(feature = "lang_golang")]
use petra_backend_golang::PetraGolangBackend;
#[cfg(feature = "lang_java")]
use petra_backend_java::PetraJavaBackend;
#[cfg(feature = "lang_js")]
use petra_backend_js::PetraJsBackend;
#[cfg(feature = "lang_python")]
use petra_backend_python::PetraPythonBackend;
#[cfg(feature = "lang_rust")]
use petra_backend_rust::PetraRustBackend;
#[cfg(feature = "lang_ts")]
use petra_backend_ts::PetraTsBackend;

use std::io::Write;

// Define your enum
#[derive(Debug, Clone, Copy)]
pub enum BackendType {
    #[cfg(feature = "lang_python")]
    Python,
    #[cfg(feature = "lang_golang")]
    GoLang,
    #[cfg(feature = "lang_ts")]
    TypeScript,
    #[cfg(feature = "lang_js")]
    JavaScript,
    #[cfg(feature = "lang_rust")]
    Rust,
    #[cfg(feature = "lang_csharp")]
    CSharp,
    #[cfg(feature = "lang_java")]
    Java,
    #[cfg(feature = "lang_cpp")]
    Cpp,
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
        #[cfg(feature = "lang_python")]
        BackendType::Python => Box::new(PetraPythonBackend::new()),
        #[cfg(feature = "lang_golang")]
        BackendType::GoLang => Box::new(PetraGolangBackend::new(configuration.golang())),
        #[cfg(feature = "lang_csharp")]
        BackendType::CSharp => Box::new(PetraCSharpBackend::new(configuration.csharp())),
        #[cfg(feature = "lang_js")]
        BackendType::JavaScript => Box::new(PetraJsBackend::new()),
        #[cfg(feature = "lang_ts")]
        BackendType::TypeScript => Box::new(PetraTsBackend::new()),
        #[cfg(feature = "lang_rust")]
        BackendType::Rust => Box::new(PetraRustBackend::new()),
        #[cfg(feature = "lang_java")]
        BackendType::Java => Box::new(PetraJavaBackend::new(configuration.java())),
        #[cfg(feature = "lang_cpp")]
        BackendType::Cpp => Box::new(PetraCppBackend::new(configuration.cpp())),
    }
}
