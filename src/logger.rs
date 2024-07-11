use env_logger::{Builder, Target};
use std::io::Write;
use log::Level;
use colored::Colorize;

pub fn init_logger() {
    let mut builder = Builder::new();
    builder.format(|buf, record| {
        let level = record.level();
        let color = match level {
            Level::Error => colored::Color::Red,
            Level::Warn => colored::Color::Yellow,
            Level::Info => colored::Color::Green,
            Level::Debug => colored::Color::Blue,
            Level::Trace => colored::Color::White
        };

        writeln!(buf,
                 "{}: {}",
                 level.to_string().color(color).bold(), record.args()
        )
    });

    builder.filter(None, log::LevelFilter::Info);
    builder.target(Target::Stdout);
    builder.init();
}
