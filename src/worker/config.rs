use std::collections::HashMap;
use std::collections::HashSet;
use std::default::Default;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub shops: Vec<ShopConfig>,
    pub products: Vec<ProductConfig>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShopConfig {
    pub name: String,
    pub name_selector: String,
    pub price_selector: String,
    pub price_factor: Option<f64>,
    pub price_index: Option<usize>,
    pub cookies: Option<HashMap<String, String>>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductConfig {
    pub shop_name: String,
    pub category: String,
    pub url: String,
}


#[derive(Debug, Clone)]
pub enum ConfigError {
    DuplicateShopName { shop_name: String },
    NoSuchShopExists { shop_name: String },
}


impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &ConfigError::DuplicateShopName { ref shop_name } => {
                write!(f, "Duplicate shop name: {}", shop_name)
            }
            &ConfigError::NoSuchShopExists { ref shop_name } => {
                write!(
                    f,
                    "Shop `{}` exists in product, but not found in shops",
                    shop_name
                )
            }
        }
    }
}


impl Default for Config {
    fn default() -> Config {
        Config {
            shops: Vec::new(),
            products: Vec::new(),
        }
    }
}


impl ConfigError {
    fn duplicate_shop_name<S>(shop_name: S) -> ConfigError
    where
        S: Into<String>,
    {
        ConfigError::DuplicateShopName { shop_name: shop_name.into() }
    }

    fn no_such_shop_exists<S>(shop_name: S) -> ConfigError
    where
        S: Into<String>,
    {
        ConfigError::NoSuchShopExists { shop_name: shop_name.into() }
    }
}


impl Config {
    pub fn validate(&self) -> Option<ConfigError> {
        let mut shop_names = HashSet::new();

        for shop in &self.shops {
            let shop_name = &shop.name;

            if shop_names.contains(shop_name) {
                return Some(ConfigError::duplicate_shop_name(shop_name.clone()));
            }

            shop_names.insert(shop_name.clone());
        }

        for product in &self.products {
            let shop_name = &product.shop_name;

            if !shop_names.contains(shop_name) {
                return Some(ConfigError::no_such_shop_exists(shop_name.clone()));
            }
        }

        None
    }
}
