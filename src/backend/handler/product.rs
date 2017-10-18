use std::collections::HashMap;
use std::error::Error;

use iron::Handler;
use iron::IronResult;
use iron::mime::Mime;
use iron::Request;
use iron::Response;
use iron::status;
use serde_json;

use database::Database;
use database::IterationPrice;


pub struct ProductHandler {
    database: Database,
}


#[derive(Serialize)]
struct HandlerResponse {
    ok: bool,
    products: Vec<ResponseProduct>,
}


#[derive(Serialize)]
struct ResponseProduct {
    category_id: i64,
    category: String,
    product: String,
    url: String,
    shop: String,
    price: f64,
    updated: i64,
}


impl HandlerResponse {
    fn ok(products: Vec<ResponseProduct>) -> HandlerResponse {
        HandlerResponse {
            ok: true,
            products: products,
        }
    }
}


impl ResponseProduct {
    fn new<S1, S2, S3, S4>(
        category_id: i64,
        category: S1,
        product: S2,
        url: S3,
        shop: S4,
        price: f64,
        updated: i64,
    ) -> ResponseProduct
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
    {
        ResponseProduct {
            category_id: category_id,
            category: category.into(),
            product: product.into(),
            url: url.into(),
            shop: shop.into(),
            price: price,
            updated: updated,
        }
    }
}


impl ProductHandler {
    pub fn new(database: Database) -> ProductHandler {
        ProductHandler { database }
    }
}


impl Handler for ProductHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let content_type: Mime = check_text!("application/json".parse(), "MIME type parsing error");
        let iteration = check_error!(self.database.iteration());
        let mut best_products = Vec::new();

        if let Some(iteration) = iteration {
            let mut products_by_category: HashMap<_, IterationPrice> = HashMap::new();

            for product_price in check_error!(self.database.product_price_by_iteration(
                iteration,
                iteration,
            ))
            {
                let category_id = product_price.category_id();

                let insert_product = match products_by_category.get(&category_id) {
                    Some(selected_price) if selected_price.price() < product_price.price() => false,
                    Some(_) => true,
                    None => true,
                };

                if insert_product {
                    products_by_category.insert(category_id, product_price);
                }
            }

            for (category_id, product_price) in products_by_category.drain() {
                best_products.push(ResponseProduct::new(
                    category_id,
                    product_price.category(),
                    product_price.product(),
                    product_price.url(),
                    product_price.shop(),
                    product_price.price(),
                    product_price.timestamp(),
                ));
            }
        }

        let response = HandlerResponse::ok(best_products);
        let body = check_error!(serde_json::to_string(&response));

        Ok(Response::with((content_type, status::Ok, body)))
    }
}
