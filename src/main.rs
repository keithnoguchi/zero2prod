//! [zero2prod]
//!
//! [zero2prod]: https://github.com/LukeMathWalker/zero-to-production/

use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
