use std::io::Write;
use std::net::TcpStream;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing_appender::rolling;

/// Interface commune
pub trait Logger {
    fn init(&self, level: String);
}

/// Logger texte lisible
pub struct StdoutLogger;
impl Logger for StdoutLogger {
    fn init(&self, level: String) {
        let filter = EnvFilter::try_new(level.as_str()).unwrap_or_else(|_| EnvFilter::new("info"));
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer().pretty())
            .init();
        println!("🟢 Logger Stdout activé");
    }
}

/// Logger JSON structuré
pub struct JsonLogger;
impl Logger for JsonLogger {
    fn init(&self, level: String) {
        let filter = EnvFilter::try_new(level.as_str()).unwrap_or_else(|_| EnvFilter::new("info"));
        let file_appender = rolling::daily("logs", "api.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        tracing_subscriber::registry()
            .with(filter)
            .with(
                fmt::layer()
                    .json()
                    .with_writer(non_blocking)
                    .with_current_span(true)
                    .with_span_list(true),
            )
            .init();
        println!("🟢 Logger JSON activé");
    }
}

/// Logger désactivé
pub struct NoopLogger;
impl Logger for NoopLogger {
    fn init(&self, level: String) {
        println!("🟡 Logger désactivé");
    }
}


/// Logger Logstash (envoi JSON via TCP sur port 5044)
pub struct LogstashLogger;
impl Logger for LogstashLogger {
    fn init(&self, level: String) {
        let filter = EnvFilter::try_new(level.as_str()).unwrap_or_else(|_| EnvFilter::new("info"));
        tracing_subscriber::registry()
            .with(filter)
            .with(
                fmt::layer()
                    .json()
                    .with_writer(|| LogstashWriter::default())
                    .with_current_span(true)
                    .with_span_list(true),
            )
            .init();
        println!("🟢 Logger Logstash activé (tcp://localhost:5044)");
    }
}

/// Writer personnalisé pour envoyer les logs à Logstash
#[derive(Default)]
struct LogstashWriter;

impl Write for LogstashWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match TcpStream::connect("127.0.0.1:5044") {
            Ok(mut stream) => {
                stream.write_all(buf)?;
                stream.write_all(b"\n")?;
                Ok(buf.len())
            }
            Err(err) => {
                eprintln!("⚠️  Connexion à Logstash échouée : {err}");
                Ok(0)
            }
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
