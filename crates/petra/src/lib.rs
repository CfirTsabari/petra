mod backend_type;
use backend_type::BackendType;
use clap::Parser;
use petra_backend::get_backend;
use petra_frontend::Frontend;

/// Convert petra file into specific language representation.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct PetraOpts {
    /// language backend
    #[arg(short, long)]
    backend: BackendType,
}

/// # Panics
///
/// Will panic if the schema is not valid or if the stdin and stdout are not available
pub fn run() {
    let opts = PetraOpts::parse();
    let fe = Frontend::new();
    let be = get_backend(&opts.backend.into());
    let intermediate = fe.parse(std::io::stdin()).unwrap();
    be.translate(intermediate.items, std::io::stdout()).unwrap();
}
