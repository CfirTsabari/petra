mod args;
mod backend_type;

use clap::Parser;
use petra_backend::{get_backend, BackendConfiguration};
use petra_frontend::Frontend;

/// # Panics
///
/// Will panic if the schema is not valid or if the stdin and stdout are not available
pub fn run() {
    let opts = args::PetraOpts::parse();
    opts.validate();
    let fe = Frontend::new();
    let mut be = get_backend(&opts.backend().into(), &opts);
    let intermediate: petra_core::Document = fe.parse(std::io::stdin()).unwrap();
    let be_config: BackendConfiguration = (&opts).into();
    be.format(&be_config, &intermediate, &mut std::io::stdout())
        .unwrap();
}
