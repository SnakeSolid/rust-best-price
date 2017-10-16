use std::error::Error;

use iron::Handler;
use iron::IronResult;
use iron::mime::Mime;
use iron::Plugin;
use iron::Request;
use iron::Response;
use iron::status;
use serde_json;
use urlencoded::UrlEncodedQuery;

use database::Database;


pub struct PriceHandler {
    database: Database,
}


#[derive(Serialize)]
struct HandlerResponse {
    ok: bool,
    products: Option<Vec<ResponseProductPrice>>,
    message: Option<String>,
}


#[derive(Serialize)]
struct ResponseProductPrice {
    product: String,
    prices: Vec<ResponsePrice>,
}


#[derive(Serialize)]
struct ResponsePrice {
    timestamp: i64,
    price: f64,
}


impl HandlerResponse {
    fn ok(products: Vec<ResponseProductPrice>) -> HandlerResponse {
        HandlerResponse {
            ok: true,
            products: Some(products),
            message: None,
        }
    }

    fn err<S>(message: S) -> HandlerResponse
    where
        S: Into<String>,
    {
        HandlerResponse {
            ok: false,
            products: None,
            message: Some(message.into()),
        }
    }
}


impl ResponseProductPrice {
    fn new<S, V>(product: S, prices: V) -> ResponseProductPrice
    where
        S: Into<String>,
        V: Into<Vec<ResponsePrice>>,
    {
        ResponseProductPrice {
            product: product.into(),
            prices: prices.into(),
        }
    }
}


impl ResponsePrice {
    fn new(timestamp: i64, price: f64) -> ResponsePrice {
        ResponsePrice { timestamp, price }
    }
}


impl PriceHandler {
    pub fn new(database: Database) -> PriceHandler {
        PriceHandler { database }
    }
}


impl Handler for PriceHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let content_type: Mime = check_text!("application/json".parse(), "MIME type parsing error");
        let params = check_params!(request, content_type);
        let category_id = check_value!(content_type, params, "category");
        let mut product_prices = Vec::new();

        for product in check_error!(self.database.products_by_category(category_id)).into_iter() {
            let mut prices = Vec::new();

            for product_price in check_error!(
                self.database.product_prices_by_product(product.id())
            ).into_iter()
            {
                prices.push(ResponsePrice::new(
                    product_price.timestamp(),
                    product_price.price(),
                ));
            }

            product_prices.push(ResponseProductPrice::new(product.name(), prices));
        }

        let response = HandlerResponse::ok(product_prices);
        let body = check_error!(serde_json::to_string(&response));

        Ok(Response::with((content_type, status::Ok, body)))
    }
}
