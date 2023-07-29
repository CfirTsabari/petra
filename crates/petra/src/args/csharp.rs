use clap::Args;
use petra_backend::config::PetraCSharpConfiguration;

#[derive(Args, Debug)]
pub struct CSharpBackendOpts {
    /// Use this namespace name
    #[arg(id = "c#-namespace", long, require_equals = true)]
    namespace_name: Option<String>,
    /// Use this class name
    #[arg(id = "c#-class-name", long, require_equals = true)]
    class_name: Option<String>,
}
impl CSharpBackendOpts {
    pub fn get_used_fields_names(&self) -> Vec<&'static str> {
        let mut res = vec![];
        if self.namespace_name.is_some() {
            res.push("c#-namespace");
        }
        if self.class_name.is_some() {
            res.push("c#-class-name");
        }
        res
    }
}
impl From<&CSharpBackendOpts> for PetraCSharpConfiguration {
    fn from(val: &CSharpBackendOpts) -> Self {
        Self::new(
            val.namespace_name.clone().map(Into::into),
            val.class_name.clone().map(Into::into),
        )
    }
}
