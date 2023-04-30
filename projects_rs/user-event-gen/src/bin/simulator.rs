use env_logger::Env;
use log::{debug, error, info, log_enabled, Level};
use user_event_gen; // - to _
fn main() {
    init_logger();
    user_event_gen::begin_simulation();
}

fn init_logger() {
    // set $RUST_LOG env variable accordingly https://docs.rs/env_logger/0.10.0/env_logger/
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
}
