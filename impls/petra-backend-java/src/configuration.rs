use petra_backend_core::Name;

const DEFAULT_CLASS_NAME: &str = "Defs";
const DEFAULT_PKG_NAME: &str = "model";

// Personal choice
#[allow(clippy::module_name_repetitions)]
pub struct PetraJavaConfiguration {
    class_name: String,
    pkg_name: String,
}
impl PetraJavaConfiguration {
    #[must_use]
    pub fn new(class_name: Option<Name>, pkg_name: Option<Name>) -> Self {
        let class_name: String =
            class_name.map_or_else(|| DEFAULT_CLASS_NAME.into(), |x| x.to_pascal_case());
        let pkg_name: String =
            pkg_name.map_or_else(|| DEFAULT_PKG_NAME.into(), |x| x.to_lower_snake());

        Self {
            class_name,
            pkg_name,
        }
    }
    #[must_use]
    pub fn get_class_name(&self) -> &str {
        &self.class_name
    }
    #[must_use]
    pub fn get_pkg_name(&self) -> &str {
        &self.pkg_name
    }
}
impl Default for PetraJavaConfiguration {
    fn default() -> Self {
        Self::new(None, None)
    }
}
