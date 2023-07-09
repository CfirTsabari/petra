use clap::Parser;
use petra_core::{Item, ItemData, PetraBackend};
use petra_python::PetraPythonBackend;
use petra_rust::PetraRustBackend;
use std::io::{Read, Write};
use std::num::ParseIntError;

// Define your enum
#[derive(Debug, Clone)]
enum BackendType {
    Python,
    GoLang,
    TypeScript,
    JavaScript,
    Rust,
}
impl BackendType {
    fn get_backend_type<T: Write>(&self) -> Box<dyn PetraBackend<T>> {
        match self {
            BackendType::Python => Box::new(PetraPythonBackend::new()),
            BackendType::GoLang => panic!("backend {:?} isn't supported", self),
            BackendType::TypeScript => panic!("backend {:?} isn't supported", self),
            BackendType::JavaScript => panic!("backend {:?} isn't supported", self),
            BackendType::Rust => Box::new(PetraRustBackend::new()),
        }
    }
    fn get_values<'a>() -> Vec<&'a str> {
        vec![
            // Python
            "Python",
            "python",
            "py",
            // GoLang
            "GoLang",
            "golang",
            "go",
            // TypeScript
            "TypeScript",
            "typescript",
            "ts",
            // JavaScript
            "JavaScript",
            "javascript",
            "js",
            // Rust
            "Rust",
            "rust",
            "rs",
        ]
    }
    fn from_str(s: &str) -> Self {
        match s {
            "Python" | "python" | "py" => BackendType::Python,
            "GoLang" | "golang" | "go" => BackendType::GoLang,
            "TypeScript" | "typescript" | "ts" => BackendType::TypeScript,
            "JavaScript" | "javascript" | "js" => BackendType::JavaScript,
            "Rust" | "rust" | "rs" => BackendType::Rust,

            _ => panic!("no match"),
        }
    }
}
/// Convert petra file into specific language representation.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct PetraOpts {
    /// language backend
    #[arg(short, long, value_parser(BackendType::get_values()))]
    backend: String,
}
struct Frontend {}
impl Frontend {
    fn new() -> Self {
        Frontend {}
    }
    fn parse<R: Read>(&self, mut reader: R) -> Result<Vec<Item>, String> {
        let mut res = vec![];
        let mut data = String::new();
        reader
            .read_to_string(&mut data)
            .map_err(|e| e.to_string())?;

        for line in data.split('\n').filter(|line| !line.is_empty()) {
            let params: Vec<&str> = line.split(':').collect();
            if params.len() != 3 {
                return Err("need to have three params".into());
            }
            let (field_type, name, value) = (params[0], params[1], params[2]);
            let new_item = match field_type {
                "string" => Item::new(name.into(), ItemData::String(value.into())),
                "number" => {
                    let num: i64 = value.parse().map_err(|e: ParseIntError| e.to_string())?;
                    Item::new(name.into(), ItemData::Integer(num))
                }
                _ => return Err(format!("undefined field type :{}", field_type)),
            };

            res.push(new_item);
        }
        Ok(res)
    }
}
pub fn run() {
    let opts = PetraOpts::parse();
    let fe = Frontend::new();
    let backend_type = BackendType::from_str(&opts.backend);
    let be = backend_type.get_backend_type();
    let intermediate = fe.parse(std::io::stdin()).unwrap();
    be.translate(intermediate, std::io::stdout()).unwrap();
}
