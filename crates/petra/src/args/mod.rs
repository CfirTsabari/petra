#[cfg(feature = "lang_cpp")]
mod cpp;
#[cfg(feature = "lang_csharp")]
mod csharp;
#[cfg(feature = "lang_golang")]
mod golang;
#[cfg(feature = "lang_java")]
mod java;

use super::backend_type::BackendType;
use clap::Parser;
use petra_backend::config::BackendConfiguration;
use petra_backend::config::PetraConfiguration;

// In the future we might need to start working with clap builder, this will allow better flexibility for example iterate through all golang params.

/// Convert petra file into specific language representation.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct PetraOpts {
    /// language backend
    #[arg(short, long)]
    backend: BackendType,
    /// disable generating the auto generated comment
    #[arg(long)]
    no_gen_comment: bool,
    #[cfg(feature = "lang_golang")]
    #[command(flatten)]
    golang_opts: Option<golang::GoLangBackendOpts>,
    #[cfg(feature = "lang_csharp")]
    #[command(flatten)]
    csharp_opts: Option<csharp::CSharpBackendOpts>,
    #[cfg(feature = "lang_java")]
    #[command(flatten)]
    java_opts: Option<java::JavaBackendOpts>,
    #[cfg(feature = "lang_cpp")]
    #[command(flatten)]
    cpp_opts: Option<cpp::CppBackendOpts>,
}
impl PetraOpts {
    pub fn validate(&self) {
        #[cfg(feature = "lang_golang")]
        if let (Some(golang_opts), true) = (
            self.golang_opts.as_ref(),
            self.backend != BackendType::GoLang,
        ) {
            self.eprint_irrelevant_fields(&golang_opts.get_used_fields_names());
        }
        #[cfg(feature = "lang_csharp")]
        if let (Some(csharp_opts), true) = (
            self.csharp_opts.as_ref(),
            self.backend != BackendType::CSharp,
        ) {
            self.eprint_irrelevant_fields(&csharp_opts.get_used_fields_names());
        }
        #[cfg(feature = "lang_java")]
        if let (Some(java_opts), true) =
            (self.java_opts.as_ref(), self.backend != BackendType::Java)
        {
            self.eprint_irrelevant_fields(&java_opts.get_used_fields_names());
        }
        #[cfg(feature = "lang_cpp")]
        if let (Some(cpp_opts), true) = (self.cpp_opts.as_ref(), self.backend != BackendType::Cpp) {
            self.eprint_irrelevant_fields(&cpp_opts.get_used_fields_names());
        }
    }
    fn eprint_irrelevant_fields(&self, used_fields: &[&str]) {
        match used_fields.len() {
            0 => {}
            1 => eprintln!(
                "Warning: provided option {} is not relevant for the backend type {:?}",
                used_fields.first().copied().unwrap_or_default(),
                self.backend
            ),
            _ => eprintln!(
                "Warning: provided options {:?} are not relevant for the backend type {:?}",
                used_fields, self.backend
            ),
        }
    }
}
impl PetraOpts {
    pub const fn backend(&self) -> BackendType {
        self.backend
    }
}
impl From<&PetraOpts> for PetraConfiguration {
    fn from(val: &PetraOpts) -> Self {
        let mut res = Self::new();
        #[cfg(feature = "lang_csharp")]
        if let Some(csharp_opts) = val.csharp_opts.as_ref() {
            res.set_csharp(csharp_opts.into());
        }
        #[cfg(feature = "lang_golang")]
        if let Some(golang_opts) = val.golang_opts.as_ref() {
            res.set_golang(golang_opts.into());
        }
        #[cfg(feature = "lang_java")]
        if let Some(java_opts) = val.java_opts.as_ref() {
            res.set_java(java_opts.into());
        }
        #[cfg(feature = "lang_cpp")]
        if let Some(cpp_opts) = val.cpp_opts.as_ref() {
            res.set_cpp(cpp_opts.into());
        }

        res
    }
}
impl From<&PetraOpts> for BackendConfiguration {
    fn from(val: &PetraOpts) -> Self {
        Self::new(!val.no_gen_comment)
    }
}
