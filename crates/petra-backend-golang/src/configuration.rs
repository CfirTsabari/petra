use petra_backend_core::Name;

const DEFAULT_PACKAGE_NAME: &str = "defs";

// Personal choice
#[allow(clippy::module_name_repetitions)]
pub struct PetraGolangConfiguration {
    package_name: String,
}
impl PetraGolangConfiguration {
    #[must_use]
    pub fn new(package_name: Option<Name>) -> Self {
        let package_name =
            package_name.map_or_else(|| DEFAULT_PACKAGE_NAME.into(), |x| x.to_lower_snake());
        Self { package_name }
    }
    #[must_use]
    pub fn get_package_name(&self) -> &str {
        &self.package_name
    }
}
impl Default for PetraGolangConfiguration {
    fn default() -> Self {
        Self::new(None)
    }
}
