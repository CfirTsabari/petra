mod csharp;
mod golang;

use super::backend_type::BackendType;
use clap::Parser;
use petra_backend::config::BackendConfiguration;
use petra_backend::config::PetraConfiguration;

use csharp::CSharpBackendOpts;
use golang::GoLangBackendOpts;

// In the future we might need to start working with clap builder, this will allow better flexibility for example iterate through all golang params.

/// Convert petra file into specific language representation.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct PetraOpts {
    #[command(flatten)]
    golang_opts: Option<GoLangBackendOpts>,
    #[command(flatten)]
    csharp_opts: Option<CSharpBackendOpts>,
    /// language backend
    #[arg(short, long)]
    backend: BackendType,
    /// language backend
    #[arg(long)]
    no_gen_comment: bool,
}
impl PetraOpts {
    pub fn validate(&self) {
        if let (Some(golang_opts), true) = (
            self.golang_opts.as_ref(),
            self.backend != BackendType::GoLang,
        ) {
            self.eprint_irrelevant_fields(&golang_opts.get_used_fields_names());
        }
        if let (Some(csharp_opts), true) = (
            self.csharp_opts.as_ref(),
            self.backend != BackendType::CSharp,
        ) {
            self.eprint_irrelevant_fields(&csharp_opts.get_used_fields_names());
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
        if let Some(csharp_opts) = val.csharp_opts.as_ref() {
            res.set_csharp(csharp_opts.into());
        }
        if let Some(golang_opts) = val.golang_opts.as_ref() {
            res.set_golang(golang_opts.into());
        }

        res
    }
}
impl From<&PetraOpts> for BackendConfiguration {
    fn from(val: &PetraOpts) -> Self {
        Self::new(!val.no_gen_comment)
    }
}
