use std::env;
pub use crate::utils::logger::{Logger, StdoutLogger, JsonLogger, NoopLogger, LogstashLogger};
pub struct LoggerFactory;

impl LoggerFactory {
    pub fn init_from_env(level: String, outpout: String) {
        let logger: Box<dyn Logger> = match outpout.as_str() {
            "stdout" => Box::new(StdoutLogger),
            "json" => Box::new(JsonLogger),
            "logstash" => Box::new(LogstashLogger),
            "none" => Box::new(NoopLogger),
            _ => Box::new(StdoutLogger),
        };
        logger.init(level);
    }
}
