// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2025 Benjamin Grolleau and Angelo Tunney

use env_logger::{Builder, Env};
use log::debug;

pub fn init_logger(verbose: bool) {
    let log_level = if verbose { "debug" } else { "info" };

    Builder::from_env(Env::default().default_filter_or(log_level)).init();

    debug!("Logger initialized with level: {}", log_level);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes_logger_in_verbose_mode() {
        let _ = std::panic::catch_unwind(|| init_logger(true));
    }

    #[test]
    fn it_initializes_logger_in_non_verbose_mode() {
        let _ = std::panic::catch_unwind(|| init_logger(false));
    }
}
