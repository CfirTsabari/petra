use clap::{AppSettings, Clap};
use petra_core::{Item, ItemData, PetraBackend};
use petra_python::PetraPythonBackend;
use std::io::{Read, Write};
use std::num::ParseIntError;
use std::str::FromStr;

// Define your enum
#[derive(Debug)]
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
            BackendType::Rust => panic!("backend {:?} isn't supported", self),
        }
    }
    fn get_values<'a>() -> &'a [&'a str] {
        &[
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
}
impl FromStr for BackendType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Python" | "python" | "py" => Ok(BackendType::Python),
            "GoLang" | "golang" | "go" => Ok(BackendType::GoLang),
            "TypeScript" | "typescript" | "ts" => Ok(BackendType::TypeScript),
            "JavaScript" | "javascript" | "js" => Ok(BackendType::JavaScript),
            "Rust" | "rust" | "rs" => Ok(BackendType::Rust),

            _ => Err("no match"),
        }
    }
}
/// Convert petra file into specific language representation.
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
struct PetraOpts {
    /// language backend
    #[clap(short, long, possible_values(BackendType::get_values()))]
    backend: BackendType,
    /// the location of the petra file
    #[clap(short, long)]
    file: Option<String>,
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
        for line in data.split('\n').filter(|line| line.is_empty()) {
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
    let opts: PetraOpts = PetraOpts::parse();
    let fe = Frontend::new();
    let be = opts.backend.get_backend_type();
    let intermediate = fe.parse(std::io::stdin()).unwrap();
    be.translate(intermediate, std::io::stdout()).unwrap();
}
