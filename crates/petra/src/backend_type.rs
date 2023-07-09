use clap::ValueEnum;
use petra_backend::BackendType as CoreBackendType;
// Define your enum
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[value(rename_all = "PascalCase")]
pub enum BackendType {
    #[value(aliases(["python","py"]))]
    Python,
    #[value(aliases(["golang","go"]))]
    GoLang,
    #[value(aliases(["typescript","ts"]))]
    TypeScript,
    #[value(aliases(["javascript","js"]))]
    JavaScript,
    #[value(aliases(["rust","rs"]))]
    Rust,
}
impl From<BackendType> for CoreBackendType {
    fn from(val: BackendType) -> Self {
        match val {
            BackendType::Python => Self::Python,
            BackendType::GoLang => Self::GoLang,
            BackendType::TypeScript => Self::TypeScript,
            BackendType::JavaScript => Self::JavaScript,
            BackendType::Rust => Self::Rust,
        }
    }
}
