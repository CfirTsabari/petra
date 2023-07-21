use super::backend_type::BackendType;
use clap::{Args, Parser};
use petra_backend::config::{PetraConfiguration, PetraGolangConfiguration};
use petra_core::Name;

// In the future we might need to start working with clap builder, this will allow better flexibility for example iterate through all golang params.

/// Convert petra file into specific language representation.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct PetraOpts {
    #[command(flatten)]
    golang_opts: Option<GoLangBackendOpts>,
    /// language backend
    #[arg(short, long)]
    backend: BackendType,
}
impl PetraOpts {
    // In the future we will have multiple stuff checking for each backend
    #[allow(clippy::collapsible_if)]
    pub fn validate(&self) {
        if self.backend != BackendType::GoLang {
            if self
                .golang_opts
                .as_ref()
                .and_then(|x| x.package_name.as_ref())
                .is_some()
            {
                eprintln!("Warning: provided option \"go-package-name\" is not relevant for the backend type {:?}",self.backend);
            }
        }
    }
}
#[derive(Args, Debug)]
pub struct GoLangBackendOpts {
    /// Use this package name
    #[arg(id = "go-package-name", long, require_equals = true)]
    package_name: Option<String>,
}
impl PetraOpts {
    pub const fn backend(&self) -> BackendType {
        self.backend
    }
}
impl From<&PetraOpts> for PetraConfiguration {
    fn from(val: &PetraOpts) -> Self {
        let mut res = Self::new();
        if let Some(golang_opts) = val.golang_opts.as_ref() {
            res.set_golang(golang_opts.into());
        }
        res
    }
}
impl From<&GoLangBackendOpts> for PetraGolangConfiguration {
    fn from(val: &GoLangBackendOpts) -> Self {
        Self::new(val.package_name.as_ref().map(|x| Name::new(x.clone())))
    }
}
