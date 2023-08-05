use clap::Args;
use petra_backend::config::PetraJavaConfiguration;

// Personal choice
#[allow(clippy::module_name_repetitions)]
#[derive(Args, Debug)]
pub struct JavaBackendOpts {
    /// Use this class name
    #[arg(id = "java-class-name", long, require_equals = true)]
    class_name: Option<String>,
    #[arg(id = "java-pkg-name", long, require_equals = true)]
    pkg_name: Option<String>,
}
impl JavaBackendOpts {
    pub fn get_used_fields_names(&self) -> Vec<&'static str> {
        let mut res = vec![];
        if self.class_name.is_some() {
            res.push("java-class-name");
        }
        if self.pkg_name.is_some() {
            res.push("java-pkg-name");
        }
        res
    }
}
impl From<&JavaBackendOpts> for PetraJavaConfiguration {
    fn from(val: &JavaBackendOpts) -> Self {
        Self::new(
            val.class_name.clone().map(Into::into),
            val.pkg_name.clone().map(Into::into),
        )
    }
}
