use std::error::Error;

use iron::Handler;
use iron::IronResult;
use iron::mime::Mime;
use iron::Request;
use iron::Response;
use iron::status;
use serde_json;

use database::Database;


pub struct PriceHandler {
    database: Database,
}


#[derive(Serialize)]
struct Product {
    category: String,
    product: String,
    url: String,
    shop: String,
    price: f64,
    updated: i64,
}


impl Product {
    fn new<S1, S2, S3, S4>(
        category: S1,
        product: S2,
        url: S3,
        shop: S4,
        price: f64,
        updated: i64,
    ) -> Product
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
    {
        Product {
            category: category.into(),
            product: product.into(),
            url: url.into(),
            shop: shop.into(),
            price: price,
            updated: updated,
        }
    }
}


impl PriceHandler {
    pub fn new(database: Database) -> PriceHandler {
        PriceHandler { database }
    }
}


impl Handler for PriceHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let content_type: Mime = "application/json".parse().unwrap();
        let mut products = Vec::new();
        products.push(Product::new(
            "cpu",
            "486DX",
            "http://nowhere.com/",
            "MegaShop",
            10.0,
            12345,
        ));
        products.push(Product::new(
            "memory",
            "1 Mib",
            "http://nowhere.com/",
            "MegaShop",
            10.0,
            12345,
        ));
        products.push(Product::new(
            "motherboard",
            "ATX compatible",
            "http://nowhere.com/",
            "MegaShop",
            10.0,
            12345,
        ));

        let body = match serde_json::to_string(&products) {
            Ok(string) => string,
            Err(err) => return Ok(Response::with((status::Ok, err.description()))),
        };

        Ok(Response::with((content_type, status::Ok, body)))
    }
}
