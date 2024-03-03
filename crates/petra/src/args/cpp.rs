use clap::Args;
use petra_backend::config::PetraCppConfiguration;
// Personal choice
#[allow(clippy::module_name_repetitions)]
#[derive(Args, Debug)]
pub struct CppBackendOpts {
    /// Use this namespace name
    #[arg(id = "cpp-namespace-name", long, require_equals = true)]
    namespace_name: Option<String>,
}
impl CppBackendOpts {
    pub fn get_used_fields_names(&self) -> Vec<&'static str> {
        let mut res = vec![];
        if self.namespace_name.is_some() {
            res.push("cpp-namespace_name");
        }
        res
    }
}
impl From<&CppBackendOpts> for PetraCppConfiguration {
    fn from(val: &CppBackendOpts) -> Self {
        Self::new(val.namespace_name.clone().map(Into::into))
    }
}
