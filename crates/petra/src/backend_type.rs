use clap::ValueEnum;
use petra_backend::BackendType as CoreBackendType;
// Define your enum
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[value(rename_all = "PascalCase")]
pub enum BackendType {
    #[cfg(feature = "python")]
    #[value(aliases(["python","py"]))]
    Python,
    #[cfg(feature = "golang")]
    #[value(aliases(["golang","go"]))]
    GoLang,
    #[value(aliases(["typescript","ts"]))]
    TypeScript,
    #[value(aliases(["javascript","js"]))]
    JavaScript,
    #[cfg(feature = "rust")]
    #[value(aliases(["rust","rs"]))]
    Rust,
    #[cfg(feature = "csharp")]
    #[value(aliases(["csharp","C#","c#"]))]
    CSharp,
}
impl From<BackendType> for CoreBackendType {
    fn from(val: BackendType) -> Self {
        match val {
            #[cfg(feature = "python")]
            BackendType::Python => Self::Python,
            #[cfg(feature = "golang")]
            BackendType::GoLang => Self::GoLang,
            BackendType::TypeScript => Self::TypeScript,
            BackendType::JavaScript => Self::JavaScript,
            #[cfg(feature = "rust")]
            BackendType::Rust => Self::Rust,
            #[cfg(feature = "csharp")]
            BackendType::CSharp => Self::CSharp,
        }
    }
}
