use std::io::{stderr, Write};
use log::{self, Level, LevelFilter, Log, Metadata, Record};
use term_painter::ToStyle;
use term_painter::Color::*;

struct Logger;

impl Log for Logger {
    fn log(&self, record: &Record) {
        match record.level() {
            Level::Error => {
                let _ = writeln!(
                    stderr(),
                    "{}{}",
                    Red.bold().paint("error: "),
                    Red.paint(record.args())
                );
            }
            Level::Warn => {
                println!(
                    "{}{}",
                    Yellow.bold().paint("warn: "),
                    Yellow.paint(record.args())
                );
            }
            Level::Debug => {
                println!(
                    "{}{}",
                    Blue.bold().paint("debug: "),
                    Blue.paint(record.args())
                );
            }
            _ => println!("{}", record.args()),
        }
    }

    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn flush(&self) {}
}

pub fn init() {
    log::set_boxed_logger(|max_level| {
        max_level.set(LevelFilter::Debug);
        Box::new(Logger)
    }).unwrap();
}
