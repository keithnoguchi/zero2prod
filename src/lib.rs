//! [zero2prod]
//!
//! [zero2prod]: https://github.com/LukeMathWalker/zero-to-production/

mod configuration;
mod routes;
mod startup;

pub use configuration::{get_config, DatabaseSettings};
pub use startup::run;
