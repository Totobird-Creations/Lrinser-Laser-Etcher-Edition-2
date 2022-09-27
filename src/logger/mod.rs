use std::cmp;
use chrono;
use log;
use colored::Colorize;

type       TimeZone           = chrono::Utc;
static mut LOGGER    : Logger = Logger {start : None};

pub struct Logger {
    start : Option<chrono::DateTime<TimeZone>>
}
impl Logger {
    pub fn init() -> Result<(), log::SetLoggerError> {
        unsafe {
            LOGGER.start = Some(TimeZone::now());
            return log::set_logger(&LOGGER)
                .map(|()| log::set_max_level(Logger::get_max_level().to_level_filter()));
        }
    }
    fn get_max_level() -> log::Level {
        return log::Level::Trace;
    }
    fn level_width() -> usize {
        let mut len = 0;
        for level in log::Level::iter() {
            len = cmp::max(len, level.to_string().len());
        }
        return len;
    }
    fn colourise(level : log::Level, text : String) -> colored::ColoredString {
        return match (level) {
            log::Level::Error => text.bright_white().on_bright_red().bold(),
            log::Level::Warn  => text.yellow(),
            log::Level::Info  => text.bright_cyan(),
            log::Level::Debug => text.normal(),
            log::Level::Trace => text.bright_black()
        }
    }
    pub fn get_time() -> chrono::DateTime<TimeZone> {
        return TimeZone::now();
    }
    pub fn format_time(time : chrono::DateTime<chrono::Utc>) -> String {
        return time.format("%Y-%m-%d %H-%M-%S.%f").to_string();
    }
}
impl log::Log for Logger {
    fn enabled(&self, metadata : &log::Metadata) -> bool {
        return metadata.level() <= Logger::get_max_level();
    }
    fn log(&self, record : &log::Record) {
        if (self.enabled(record.metadata())) {
            println!(" [ {time} ][ {level:level_width$} ] {message}",
                time        = Logger::format_time(Logger::get_time()).to_string().green().dimmed(),
                level       = Logger::colourise(record.level(), record.level().to_string()).bold(),
                level_width = Logger::level_width(),
                message     = Logger::colourise(record.level(), record.args().to_string())
            );
        }
    }
    fn flush(&self) {}
}

#[macro_export]
macro_rules! error {
    ($($args:tt),*) => {{
        let text = format!($($args),*);
        log::error!("{}", text);
        std::process::exit(1);
    }}
}
