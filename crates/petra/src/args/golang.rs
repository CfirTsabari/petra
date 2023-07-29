use clap::Args;
use petra_backend::config::PetraGolangConfiguration;

#[derive(Args, Debug)]
pub struct GoLangBackendOpts {
    /// Use this package name
    #[arg(id = "go-package-name", long, require_equals = true)]
    package_name: Option<String>,
}
impl GoLangBackendOpts {
    pub fn get_used_fields_names(&self) -> Vec<&'static str> {
        let mut res = vec![];
        if self.package_name.is_some() {
            res.push("go-package-name");
        }
        res
    }
}
impl From<&GoLangBackendOpts> for PetraGolangConfiguration {
    fn from(val: &GoLangBackendOpts) -> Self {
        Self::new(val.package_name.clone().map(Into::into))
    }
}
