/// Initialize the jantu tracing subscriber.
///
/// Reads the `JANTU_LOG` environment variable for filter directives
/// (e.g. `JANTU_LOG=debug`). Falls back to `warn` level if unset.
///
/// Requires the `logging` feature (which implies `std`).
pub fn init() {
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::try_from_env("JANTU_LOG").unwrap_or_else(|_| EnvFilter::new("warn"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}
