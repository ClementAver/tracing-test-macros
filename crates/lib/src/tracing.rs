use std::sync::Once;
use tracing::Level;

static INIT: Once = Once::new();

pub fn init_tracing() {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .compact()
            .with_max_level(Level::TRACE)
            .without_time()
            .with_line_number(true)
            .try_init()
            .expect("could not init env filter")
    });
}
