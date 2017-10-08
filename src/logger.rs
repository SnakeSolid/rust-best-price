use std::env;
use std::error::Error;
use std::fmt::Display;

use log::LogRecord;
use log::LogLevelFilter;
use log::SetLoggerError;
use env_logger::LogBuilder;
use time;


fn format(record: &LogRecord) -> String {
    let t = time::now();

    if let Ok(time_string) = time::strftime("%Y-%m-%d %H:%M:%S", &t) {
        format!("{}: {} - {}", time_string, record.level(), record.args())
    } else {
        format!(
            "????-??-?? ??:??:??: {} - {}",
            record.level(),
            record.args()
        )
    }
}


pub fn init() -> Result<(), SetLoggerError> {
    let mut builder = LogBuilder::new();
    builder.format(format);
    builder.filter(None, LogLevelFilter::Info);

    if let Ok(rust_log) = env::var("RUST_LOG") {
        builder.parse(&rust_log);
    }

    builder.init()
}


pub trait UnwrapLog<T> {
    fn unwrap_log<D>(self, description: D) -> T
    where
        D: Display;
}


pub trait ExpectLog<T> {
    fn expect_log<D>(self, description: D) -> T
    where
        D: Display;
}


impl<T, E> UnwrapLog<T> for Result<T, E>
where
    E: Error + Display,
{
    fn unwrap_log<D>(self, description: D) -> T
    where
        D: Display,
    {
        match self {
            Ok(value) => value,
            Err(error) => {
                error!("{}: {}", description, error);
                panic!("{}: {}", description, error);
            }
        }
    }
}


impl<T, E> ExpectLog<T> for Result<T, E> {
    fn expect_log<D>(self, description: D) -> T
    where
        D: Display,
    {
        match self {
            Ok(value) => value,
            Err(_) => {
                error!("{}", description);
                panic!("{}", description);
            }
        }
    }
}
