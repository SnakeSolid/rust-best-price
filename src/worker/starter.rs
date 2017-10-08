use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fs::File;
use std::io::Error as IoError;
use std::io::Read;
use std::io::Result as IoResult;
use std::path::Path;
use std::path::PathBuf;
use std::thread::Builder;
use std::thread::JoinHandle;
use std::thread;
use std::time::Duration;

use time;
use toml::de::Error as TomlError;
use toml;

use database::Database;
use logger::UnwrapLog;

use super::Config;
use super::ConfigError;
use super::PriceLoader;


#[derive(Debug, Clone)]
enum ReadConfigError {
    IoError { description: String },
    ParsingError {
        line: Option<usize>,
        column: Option<usize>,
    },
    InvalidConfig { error: ConfigError },
}


impl ReadConfigError {
    fn invalid_config(error: ConfigError) -> ReadConfigError {
        ReadConfigError::InvalidConfig { error }
    }
}


impl From<IoError> for ReadConfigError {
    fn from(error: IoError) -> ReadConfigError {
        ReadConfigError::IoError { description: error.description().into() }
    }
}


impl From<TomlError> for ReadConfigError {
    fn from(error: TomlError) -> ReadConfigError {
        ReadConfigError::ParsingError {
            line: error.line_col().map(|(line, _)| line),
            column: error.line_col().map(|(_, column)| column),
        }
    }
}


impl Display for ReadConfigError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &ReadConfigError::IoError { ref description } => write!(f, "IO error: {}", description),
            &ReadConfigError::ParsingError {
                line: Some(line),
                column: Some(column),
            } => write!(f, "Parsing error at line {} column {}", line, column),
            &ReadConfigError::ParsingError {
                line: Some(line),
                column: None,
            } => write!(f, "Parsing error at line {}", line),
            &ReadConfigError::ParsingError {
                line: None,
                column: Some(column),
            } => write!(f, "Parsing error at column {}", column),
            &ReadConfigError::ParsingError {
                line: None,
                column: None,
            } => write!(f, "Parsing error"),
            &ReadConfigError::InvalidConfig { ref error } => {
                write!(f, "Invalid configuration: {}", error)
            }
        }
    }
}


impl Error for ReadConfigError {
    fn description(&self) -> &str {
        match self {
            &ReadConfigError::IoError { .. } => "IO error",
            &ReadConfigError::ParsingError { .. } => "Parsing error",
            &ReadConfigError::InvalidConfig { .. } => "Invalid configuration",
        }
    }
}


fn read_config<P>(path: P) -> Result<Config, ReadConfigError>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    info!("Reading file {}.", path.display());

    let mut file = File::open(path)?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    info!("Parsing config from file {}.", path.display());

    let config: Config = toml::from_str(&content)?;

    info!("Validating config from file {}.", path.display());

    if let Some(error) = config.validate() {
        Err(ReadConfigError::invalid_config(error))
    } else {
        Ok(config)
    }
}


fn store_products(database: &Database, config: &Config, loader: &mut PriceLoader) -> () {
    info!("Procisseng productrs.");

    let mut shops = HashMap::new();

    for shop in &config.shops {
        shops.insert(shop.name.clone(), shop.clone());
    }

    for product in &config.products {
        let timestamp = time::get_time().sec;
        let shop_name = &product.shop_name;
        let shop = shops.get(shop_name).expect("Shop from product not found");
        let price = loader.load(
            &product.url,
            &shop.cookies,
            &shop.name_selector,
            &shop.price_selector,
            shop.price_factor.unwrap_or(1.0),
            shop.price_index.unwrap_or(0),
        );

        match price {
            Ok(price) => {
                let result = database.save_price(
                    shop_name,
                    &product.category,
                    &price.name,
                    timestamp,
                    price.price,
                );

                if let Err(error) = result {
                    warn!("Can not save product price: {}", error);
                }
            }
            Err(error) => warn!("Product parsing error: {}", error),
        }
    }
}


fn run(database: Database, config_path: PathBuf, period: time::Duration) {
    info!("Creating price loader.");

    let mut price_loader = PriceLoader::new().unwrap_log("Price loader creation error");

    info!("Strating price update loop.");

    loop {
        info!("Strating price update cycle.");

        let start_time = time::now();

        match read_config(&config_path) {
            Ok(config) => store_products(&database, &config, &mut price_loader),
            Err(error) => warn!("Error reading configuration: {}", error),
        }

        let complete_time = time::now();
        let update_duration = complete_time - start_time;

        if update_duration < period {
            let sleep_time = period.num_seconds() - update_duration.num_seconds();

            if sleep_time > 0 {
                info!("Sleeping to next update for {} seconds", sleep_time);

                thread::sleep(Duration::from_secs(sleep_time as u64));
            }
        } else {
            warn!("Period is too short to update all the products");
        }
    }
}


pub fn start_crawler<P>(database: Database, path: P, period: usize) -> IoResult<JoinHandle<()>>
where
    P: AsRef<Path>,
{
    info!("Starting background loader thread.");

    let config_path = path.as_ref().to_path_buf();
    let period = time::Duration::hours(period as i64);

    Builder::new()
        .name("crawler".into())
        .stack_size(512 * 1024)
        .spawn(move || run(database, config_path, period))
}
