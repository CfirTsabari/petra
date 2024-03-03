use clap::ValueEnum;
use petra_backend::BackendType as CoreBackendType;
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[value(rename_all = "PascalCase")]
pub enum BackendType {
    #[cfg(feature = "lang_python")]
    #[value(aliases(["python","py"]),help("aliases: python, py."))]
    Python,
    #[cfg(feature = "lang_golang")]
    #[value(aliases(["golang","go"]),help("aliases: golang, go."))]
    GoLang,
    #[cfg(feature = "lang_ts")]
    #[value(aliases(["typescript","ts"]),help("aliases: typescript, ts."))]
    TypeScript,
    #[cfg(feature = "lang_js")]
    #[value(aliases(["javascript","js"]),help("aliases: javascript, js."))]
    JavaScript,
    #[cfg(feature = "lang_rust")]
    #[value(aliases(["rust","rs"]),help("aliases: rust, rs."))]
    Rust,
    #[cfg(feature = "lang_csharp")]
    #[value(aliases(["csharp","C#","c#","cs"]),help("aliases: csharp, C#, c#, cs."))]
    CSharp,
    #[cfg(feature = "lang_java")]
    #[value(aliases(["java"]),help("aliases: java."))]
    Java,
    #[cfg(feature = "lang_cpp")]
    #[value(aliases(["cpp","cplusplus","c++"]),help("aliases: cpp, cplusplus, c++."))]
    Cpp,
}
impl From<BackendType> for CoreBackendType {
    fn from(val: BackendType) -> Self {
        match val {
            #[cfg(feature = "lang_python")]
            BackendType::Python => Self::Python,
            #[cfg(feature = "lang_golang")]
            BackendType::GoLang => Self::GoLang,
            #[cfg(feature = "lang_ts")]
            BackendType::TypeScript => Self::TypeScript,
            #[cfg(feature = "lang_js")]
            BackendType::JavaScript => Self::JavaScript,
            #[cfg(feature = "lang_rust")]
            BackendType::Rust => Self::Rust,
            #[cfg(feature = "lang_csharp")]
            BackendType::CSharp => Self::CSharp,
            #[cfg(feature = "lang_java")]
            BackendType::Java => Self::Java,
            #[cfg(feature = "lang_cpp")]
            BackendType::Cpp => Self::Cpp,
        }
    }
}
