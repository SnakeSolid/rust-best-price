use std::error::Error;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::num::ParseIntError;

use hyper::Error as HyperError;
use hyper::error::UriError;


#[derive(Debug, Clone)]
pub struct Product {
    pub name: String,
    pub price: f64,
}


#[derive(Debug, Clone)]
pub enum ProductError {
    InvaliudSchema,
    InvaliudUri,
    IoError { description: String },
    NameElementNotExists,
    PriceElementNotExists,
    NameNotFound,
    PriceNotFound,
    ParsePriceError,
}


impl ProductError {
    #[inline]
    pub fn invalid_schema() -> ProductError {
        ProductError::InvaliudSchema
    }

    #[inline]
    pub fn name_not_exists(_: ()) -> ProductError {
        ProductError::NameElementNotExists
    }

    #[inline]
    pub fn price_not_exists(_: ()) -> ProductError {
        ProductError::PriceElementNotExists
    }

    #[inline]
    pub fn name_not_found() -> ProductError {
        ProductError::NameNotFound
    }

    #[inline]
    pub fn price_not_found() -> ProductError {
        ProductError::PriceNotFound
    }
}


impl From<UriError> for ProductError {
    fn from(_: UriError) -> ProductError {
        ProductError::InvaliudUri
    }
}


impl From<HyperError> for ProductError {
    fn from(error: HyperError) -> ProductError {
        ProductError::IoError { description: error.description().into() }
    }
}


impl From<ParseIntError> for ProductError {
    fn from(_: ParseIntError) -> ProductError {
        ProductError::ParsePriceError
    }
}


impl Display for ProductError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            &ProductError::InvaliudSchema => write!(f, "Invalid schema"),
            &ProductError::InvaliudUri => write!(f, "Invalid URI"),
            &ProductError::IoError { ref description } => write!(f, "IO error: {}", description),
            &ProductError::NameElementNotExists => write!(f, "DOM node for name does not exists"),
            &ProductError::PriceElementNotExists => write!(f, "DOM node for price does not exists"),
            &ProductError::NameNotFound => write!(f, "Name not found on a page"),
            &ProductError::PriceNotFound => write!(f, "Price not found on a page"),
            &ProductError::ParsePriceError => write!(f, "Price has non numeric format"),
        }
    }
}


impl Error for ProductError {
    fn description(&self) -> &str {
        match self {
            &ProductError::InvaliudSchema => "",
            &ProductError::InvaliudUri => "",
            &ProductError::IoError { .. } => "",
            &ProductError::NameElementNotExists => "",
            &ProductError::PriceElementNotExists => "",
            &ProductError::NameNotFound => "",
            &ProductError::PriceNotFound => "",
            &ProductError::ParsePriceError => "",
        }
    }
}


impl Product {
    pub fn new(name: String, price: f64) -> Product {
        Product { name, price }
    }
}
