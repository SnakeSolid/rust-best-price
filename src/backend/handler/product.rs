use std::collections::HashMap;
use std::error::Error;

use iron::Handler;
use iron::IronResult;
use iron::mime::Mime;
use iron::Request;
use iron::Response;
use iron::status;
use serde_json;

use database::Category;
use database::Database;
use database::DatabaseError;
use database::Product as DbProduct;
use database::ProductPrice;
use database::Shop;


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
        let last_iteration = check_error!(self.database.last_iteration());
        let mut best_products = Vec::new();

        if let Some(last_iteration) = last_iteration {
            let shops = check_error!(self.shops());
            let categories = check_error!(self.categories());
            let products = check_error!(self.products());
            let mut selected_products: HashMap<_, ProductPrice> = HashMap::new();

            for product_price in check_error!(
                self.database.product_prices(last_iteration, last_iteration)
            )
            {
                let product_id = product_price.product_id();
                let product = match products.get(&product_id) {
                    Some(product) => product,
                    None => {
                        warn!("Product {} not found", product_id);

                        continue;
                    }
                };
                let category_id = product.category_id();
                let insert_product = match selected_products.get(&category_id) {
                    Some(selected_price) if selected_price.price() < product_price.price() => false,
                    Some(_) => true,
                    None => true,
                };

                if insert_product {
                    selected_products.insert(category_id, product_price);
                }
            }

            for (category_id, product_price) in selected_products.drain() {
                let product_id = product_price.product_id();
                let product = match products.get(&product_id) {
                    Some(product) => product,
                    None => {
                        warn!("Product {} not found", product_id);

                        continue;
                    }
                };
                let category = match categories.get(&category_id) {
                    Some(category) => category,
                    None => {
                        warn!("Category {} not found", product_id);

                        continue;
                    }
                };
                let shop_id = product.shop_id();
                let shop = match shops.get(&shop_id) {
                    Some(shop) => shop,
                    None => {
                        warn!("Shop {} not found", product_id);

                        continue;
                    }
                };
                let product = ResponseProduct::new(
                    category_id,
                    category.name(),
                    product.name(),
                    product.url(),
                    shop.name(),
                    product_price.price(),
                    product_price.timestamp(),
                );

                best_products.push(product);
            }
        }

        let response = HandlerResponse::ok(best_products);
        let body = check_error!(serde_json::to_string(&response));

        Ok(Response::with((content_type, status::Ok, body)))
    }
}

impl ProductHandler {
    fn shops(&self) -> Result<HashMap<i64, Shop>, DatabaseError> {
        let mut result = HashMap::new();

        for shop in self.database.shops()? {
            result.insert(shop.id(), shop);
        }

        Ok(result)
    }

    fn categories(&self) -> Result<HashMap<i64, Category>, DatabaseError> {
        let mut result = HashMap::new();

        for category in self.database.categories()? {
            result.insert(category.id(), category);
        }

        Ok(result)
    }

    fn products(&self) -> Result<HashMap<i64, DbProduct>, DatabaseError> {
        let mut result = HashMap::new();

        for product in self.database.products()? {
            result.insert(product.id(), product);
        }

        Ok(result)
    }
}
