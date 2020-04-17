use log::{LevelFilter, Metadata, Record};

#[derive(Debug)]
struct SimpleLogger;

impl log::Log for SimpleLogger {
	fn enabled(&self, _metadata: &Metadata) -> bool {
		true
	}

	fn log(&self, record: &Record) {
		if self.enabled(record.metadata()) {
			println!("{}: {}", record.level(), record.args());
		}
	}

	fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() {
	log::set_logger(&LOGGER)
		.map(|()| log::set_max_level(LevelFilter::Trace))
		.expect("Failed to set a logger.");
}
