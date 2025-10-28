use env_logger::{Builder, Env};
use log::debug;

pub fn init_logger(verbose: bool) {
    let log_level = if verbose { "debug" } else { "info" };

    Builder::from_env(Env::default().default_filter_or(log_level)).init();

    debug!("Logger initialized with level: {}", log_level);
}
