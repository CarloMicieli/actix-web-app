use slog::{o, Drain, Logger};
use slog_async;
use slog_envlogger;
use slog_term;
use std::env;

pub struct Config {}

impl Config {
    pub fn configure_log() -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
        let console_drain = slog_envlogger::new(console_drain);
        let console_drain = slog_async::Async::new(console_drain).build().fuse();
        slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
    }

    pub fn bind_address_from_env() -> String {
        let app_host = env::var("APP_HOST").unwrap_or("0.0.0.0".to_owned());
        let port_number = env::var("APP_PORT").unwrap_or("5000".to_owned());
        format!("{}:{}", &app_host, &port_number)
    }
}