use petra_backend_core::Name;

const DEFAULT_NAMESPACE_NAME: &str = "Defs";

// Personal choice
#[allow(clippy::module_name_repetitions)]
pub struct PetraCppConfiguration {
    namespace_name: String,
}
impl PetraCppConfiguration {
    #[must_use]
    pub fn new(namespace_name: Option<Name>) -> Self {
        let namespace_name =
            namespace_name.map_or_else(|| DEFAULT_NAMESPACE_NAME.into(), |x| x.to_pascal_case());
        Self { namespace_name }
    }
    #[must_use]
    pub fn get_namespace_name(&self) -> &str {
        &self.namespace_name
    }
}
impl Default for PetraCppConfiguration {
    fn default() -> Self {
        Self::new(None)
    }
}
