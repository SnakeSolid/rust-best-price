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
extern crate native_tls;
extern crate sqlite;
extern crate tendril;
extern crate time;
extern crate tokio_core;
extern crate toml;

mod database;
mod logger;
mod settings;
mod worker;

use iron::Iron;
use iron::Request;
use iron::Response;
use iron::status;

use database::Database;
use logger::ExpectLog;
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
    ).unwrap_log("Can not initialize database connection");
    let loader = start_crawler(database.clone(), settings.config_path(), settings.period())
        .unwrap_log("Can not start background loader thread");
    let bind_address = settings.bind_address();
    let bind_port = settings.bind_port();

    info!("Starting WEB server on {}:{}", bind_address, bind_port);

    Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello World!")))
    }).http((bind_address.as_str(), bind_port))
        .unwrap_log("Failed to start WEB server");

    info!("Joining background loader thread.");

    loader.join().expect_log("Loader joining error");
}
