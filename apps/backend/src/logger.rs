use std::default;

use colored::Colorize;
use log::{Level, Metadata, Record};

pub struct Logger;

impl log::Log for Logger {
	fn enabled(&self, metadata: &Metadata) -> bool {
		metadata.level() <= Level::Info
	}

	fn log(&self, record: &Record) {
		if self.enabled(record.metadata()) {
			let level = record.level();
			let level_str = level.to_string();
			let level_style = match level {
				Level::Error => level_str.red(),
				Level::Warn => level_str.yellow(),
				Level::Info => level_str.blue(),
				Level::Debug => level_str.dimmed(),
				Level::Trace => level_str.white(),
			}
			.bold();

			let args_str = match level {
				Level::Debug => record.args().to_string().dimmed(),
				_ => record.args().to_string().bright_white(),
			};

			println!("{} {}", level_style, args_str);
		}
	}

	fn flush(&self) {}
}
