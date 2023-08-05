mod configuration;
mod defs;
mod error;
mod traits;
mod utils;

use error::Result;

pub use configuration::BackendConfiguration;
pub use error::PetraBackendError;
pub use petra_backend_core_derive::{self, SimpleLanguageBackend};
pub use petra_core::Name;
pub use traits::format;
pub use traits::simple_language::SimpleLanguageBackend;
pub use traits::Backend;
pub use utils::MultiCommentFormatter;
