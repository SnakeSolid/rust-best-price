#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

extern crate argparse;
extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate iron;
extern crate kuchiki;
extern crate mount;
extern crate native_tls;
extern crate router;
extern crate serde_json;
extern crate sqlite;
extern crate staticfile;
extern crate tendril;
extern crate time;
extern crate tokio_core;
extern crate toml;
extern crate urlencoded;

mod backend;
mod database;
mod logger;
mod settings;
mod worker;

use backend::start_backend;
use database::Database;
use logger::UnwrapLog;
use settings::Settings;
use worker::start_crawler;


fn main() {
    if let Err(err) = logger::init() {
        panic!("Failed to initalize logger: {}", err);
    }

    let settings = Settings::from_args();
    let database = Database::connect(
        settings.database_path(),
        settings.create_database(),
        settings.force(),
    ).unwrap_log("Can't initialize database connection");

    if !settings.disable_crawler() {
        start_crawler(database.clone(), settings.config_path(), settings.period())
            .unwrap_log("Can't start background loader thread");
    }

    start_backend(database, settings.bind_address(), settings.bind_port())
        .unwrap_log("Can't start backend serwer");
}
