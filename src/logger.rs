use std::io::{stderr, Write};
use log::{self, LogLevel, LogLevelFilter, Log, LogMetadata, LogRecord};
use term_painter::ToStyle;
use term_painter::Color::*;

struct Logger;

impl Log for Logger {
    fn log(&self, record: &LogRecord) {
        match record.level() {
            LogLevel::Error => {
                let _ = writeln!(
                    stderr(),
                    "{}{}",
                    Red.bold().paint("error: "),
                    Red.paint(record.args())
                );
            }
            LogLevel::Warn => {
                println!(
                    "{}{}",
                    Yellow.bold().paint("warn: "),
                    Yellow.paint(record.args())
                );
            }
            LogLevel::Debug => {
                println!(
                    "{}{}",
                    Blue.bold().paint("debug: "),
                    Blue.paint(record.args())
                );
            }
            _ => println!("{}", record.args()),
        }
    }

    fn enabled(&self, _: &LogMetadata) -> bool {
        true
    }
}

pub fn init() {
    log::set_logger(|max_level| {
        max_level.set(LogLevelFilter::Debug);
        Box::new(Logger)
    }).unwrap();
}
