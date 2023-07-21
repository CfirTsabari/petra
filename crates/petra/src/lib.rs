mod args;
mod backend_type;

use clap::Parser;
use petra_backend::get_backend;
use petra_frontend::Frontend;

/// # Panics
///
/// Will panic if the schema is not valid or if the stdin and stdout are not available
pub fn run() {
    let opts = args::PetraOpts::parse();
    opts.validate();
    let fe = Frontend::new();
    let be = get_backend(&opts.backend().into(), &opts);
    let intermediate: petra_core::Document = fe.parse(std::io::stdin()).unwrap();
    be.translate(intermediate, std::io::stdout()).unwrap();
}
