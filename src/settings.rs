use ctor::ctor;
use tracing::Level;

#[ctor]
fn set_debug_level() {
    // If DEBUG_GRAPHINA is not set or set to false, disable logging. Otherwise, enable logging
    if std::env::var("DEBUG_GRAPHINA").map_or(true, |v| v == "0" || v == "false" || v.is_empty()) {
        // Disable logging
    } else {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
    }
}
