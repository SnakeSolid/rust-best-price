use std::error::Error;
use std::num::ParseIntError;

use hyper::Error as HyperError;
use hyper::error::UriError;


#[derive(Debug, Clone)]
pub struct Product {
    name: String,
    price: f64,
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


impl Product {
    pub fn new(name: String, price: f64) -> Product {
        Product { name, price }
    }
}
