use argparse::ArgumentParser;
use argparse::StoreOption;
use argparse::StoreTrue;


#[derive(Debug, Clone)]
pub struct Settings {
    bind_address: String,
    bind_port: u16,
    period: usize,
    config_path: String,
    database_path: String,
    create_database: bool,
    force: bool,
}


impl Settings {
    pub fn from_args() -> Settings {
        info!("Parsing setting from command line arguments");

        let mut bind_address: Option<String> = None;
        let mut bind_port: Option<u16> = None;
        let mut period: Option<usize> = None;
        let mut config_path: Option<String> = None;
        let mut database_path: Option<String> = None;
        let mut create_database: bool = false;
        let mut force: bool = false;

        {
            let mut ap = ArgumentParser::new();

            ap.set_description("PostgreSQL database test tool.");
            ap.refer(&mut bind_address).add_option(
                &["-b", "--bind"],
                StoreOption,
                "Address to bind on (default: localhost)",
            );
            ap.refer(&mut bind_port).add_option(
                &["-p", "--port"],
                StoreOption,
                "Port to listen (default: 8080)",
            );
            ap.refer(&mut period).add_option(
                &["-d", "--period"],
                StoreOption,
                "Update period in hours (default: 12)",
            );
            ap.refer(&mut config_path).add_option(
                &["-f", "--config"],
                StoreOption,
                "Path to local database (default: config.toml)",
            );
            ap.refer(&mut database_path).add_option(
                &["-d", "--database"],
                StoreOption,
                "Path to local database (default: local.sqlite)",
            );
            ap.refer(&mut create_database).add_option(
                &["-c", "--create"],
                StoreTrue,
                "Create local database (default: false)",
            );
            ap.refer(&mut force).add_option(
                &["-f", "--force"],
                StoreTrue,
                "Force rewrite of local database (default: false)",
            );
            ap.parse_args_or_exit();
        }

        let mut config = Self::default();

        if let Some(bind_address) = bind_address {
            config.bind_address = bind_address;
        }

        if let Some(bind_port) = bind_port {
            config.bind_port = bind_port;
        }

        if let Some(period) = period {
            config.period = period;
        }

        if let Some(config_path) = config_path {
            config.config_path = config_path;
        }

        if let Some(database_path) = database_path {
            config.database_path = database_path;
        }

        config.create_database |= create_database;
        config.force |= force;
        config
    }

    pub fn bind_address(&self) -> String {
        self.bind_address.clone()
    }

    pub fn bind_port(&self) -> u16 {
        self.bind_port
    }

    pub fn period(&self) -> usize {
        self.period
    }

    pub fn config_path(&self) -> String {
        self.config_path.clone()
    }

    pub fn database_path(&self) -> String {
        self.database_path.clone()
    }

    pub fn create_database(&self) -> bool {
        self.create_database
    }

    pub fn force(&self) -> bool {
        self.force
    }
}


impl Default for Settings {
    fn default() -> Self {
        Settings {
            bind_address: "localhost".into(),
            bind_port: 8080,
            period: 12,
            config_path: "config.toml".into(),
            database_path: "local.sqlite".into(),
            create_database: false,
            force: false,
        }
    }
}