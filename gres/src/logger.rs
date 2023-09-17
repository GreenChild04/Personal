use soulog::*;

pub struct GresLogger;

impl Logger for GresLogger {
    fn new() -> Self { Self }
    fn hollow(&self) -> Self { Self::new() }

    fn crash<T>(&mut self) -> T {
        std::process::exit(1)
    }

    fn verbose(&mut self, log: Log) {
        println!("{}", colour_format!(blue("["), cyan(log.origin), blue("] "), none(log.message)));
    }

    fn error(&mut self, log: Log) -> ErrorResponse {
        let message = match log.log_type {
            LogType::Failure => colour_format![blue("["), red(log.origin), blue("] "), red("Failure"), blue(": "), none(log.message)],
            LogType::Fatal => colour_format![blue("["), red(log.origin), blue("] "), red("Fatal"), blue(": "), none(log.message)],
            _ => panic!("meta error: invalid error log type '{:?}'", log.log_type),
        }; println!("{message}");

        ErrorResponse::Crash
    }

    fn vital(&mut self, log: Log) {
        let message = match log.log_type {
            LogType::Inconvenience => colour_format![blue("["), yellow(log.origin), blue("] "), yellow("Inconvenience"), blue(": "), none(log.message)],
            LogType::Warning => colour_format![blue("["), yellow(log.origin), blue("] "), yellow("Warning"), blue(": "), none(log.message)],
            LogType::Result => colour_format![blue("["), green("Result"), blue("] "), green(log.origin), blue(": "), none(log.message)],
            LogType::Log => colour_format!(blue("["), green(log.origin), blue("] "), none(log.message)),
            _ => panic!("meta error: invalid error log type '{:?}'", log.log_type),
        }; println!("{message}");
    }
}
