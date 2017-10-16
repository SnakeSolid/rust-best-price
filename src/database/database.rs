use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use sqlite::Connection;
use sqlite::State;
use sqlite::Value;
use sqlite;

use super::Category;
use super::DatabaseError;
use super::Product;
use super::ProductPrice;
use super::Shop;


#[derive(Clone)]
pub struct Database {
    connection: Arc<Mutex<Connection>>,
}


impl Debug for Database {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Database {{ .. }}")
    }
}


impl Database {
    pub fn connect(
        file_name: String,
        create: bool,
        force: bool,
    ) -> Result<Database, DatabaseError> {
        info!("Connecting to local database");

        let file_path: PathBuf = file_name.into();

        if file_path.exists() {
            if !file_path.is_file() {
                return Err(DatabaseError::is_directory(file_path));
            }

            if force {
                info!("Removing old database file {}", file_path.display());

                fs::remove_file(&file_path)?;
            }
        }

        let connection = sqlite::open(&file_path)?;

        if create {
            info!("Creating tables");

            connection.execute(include_str!("schema.sql"))?;

            info!("All tables created");
        }

        Ok(Database { connection: Arc::new(Mutex::new(connection)) })
    }

    pub fn save_price(
        &self,
        shop: &String,
        category: &String,
        product_url: &String,
        product_name: &String,
        iteration: i64,
        timestamp: i64,
        price: f64,
    ) -> Result<(), DatabaseError> {
        let mut connection = self.connection.lock()?;
        let shop_id = shop_id(&mut connection, shop)?;
        let category_id = category_id(&mut connection, category)?;
        let product_id = product_id(
            &mut connection,
            shop_id,
            category_id,
            product_url,
            product_name,
        )?;

        save_product_price(&mut connection, product_id, iteration, timestamp, price)?;

        Ok(())
    }

    pub fn last_iteration(&self) -> Result<Option<i64>, DatabaseError> {
        let mut connection = self.connection.lock()?;
        let last_iteration = get_last_iteration(&mut connection)?;

        Ok(last_iteration)
    }

    pub fn product_prices(
        &self,
        iteration_from: i64,
        iteration_to: i64,
    ) -> Result<Vec<ProductPrice>, DatabaseError> {
        let mut connection = self.connection.lock()?;
        let product_prices = get_product_prices(&mut connection, iteration_from, iteration_to)?;

        Ok(product_prices)
    }

    pub fn product(&self, id: i64) -> Result<Option<Product>, DatabaseError> {
        let mut connection = self.connection.lock()?;
        let product = get_product(&mut connection, id)?;

        Ok(product)
    }

    pub fn category(&self, id: i64) -> Result<Option<Category>, DatabaseError> {
        let mut connection = self.connection.lock()?;
        let category = get_category(&mut connection, id)?;

        Ok(category)
    }

    pub fn shop(&self, id: i64) -> Result<Option<Shop>, DatabaseError> {
        let mut connection = self.connection.lock()?;
        let shop = get_shop(&mut connection, id)?;

        Ok(shop)
    }

    pub fn products(&self) -> Result<Vec<Product>, DatabaseError> {
        let mut connection = self.connection.lock()?;
        let products = get_products(&mut connection)?;

        Ok(products)
    }

    pub fn categories(&self) -> Result<Vec<Category>, DatabaseError> {
        let mut connection = self.connection.lock()?;
        let categories = get_categories(&mut connection)?;

        Ok(categories)
    }

    pub fn shops(&self) -> Result<Vec<Shop>, DatabaseError> {
        let mut connection = self.connection.lock()?;
        let shops = get_shops(&mut connection)?;

        Ok(shops)
    }

    pub fn products_by_category(&self, category_id: i64) -> Result<Vec<Product>, DatabaseError> {
        let mut connection = self.connection.lock()?;
        let products = get_products_by_category(&mut connection, category_id)?;

        Ok(products)
    }

    pub fn product_prices_by_product(
        &self,
        product_id: i64,
    ) -> Result<Vec<ProductPrice>, DatabaseError> {
        let mut connection = self.connection.lock()?;
        let products = get_product_prices_by_product(&mut connection, product_id)?;

        Ok(products)
    }
}


fn get_product_prices_by_product(
    connection: &mut Connection,
    product_id: i64,
) -> Result<Vec<ProductPrice>, DatabaseError> {
    let mut statement = connection.prepare(
        "SELECT id, product_id, iteration, timestamp, price FROM product_price WHERE product_id = ?",
    )?;
    statement.bind(1, product_id)?;

    let mut result = Vec::new();

    while let State::Row = statement.next()? {
        let id = statement.read(0)?;
        let product_id = statement.read(1)?;
        let iteration = statement.read(2)?;
        let timestamp = statement.read(3)?;
        let price = statement.read(4)?;

        result.push(ProductPrice::new(
            id,
            product_id,
            iteration,
            timestamp,
            price,
        ));
    }

    Ok(result)
}

fn get_products_by_category(
    connection: &mut Connection,
    category_id: i64,
) -> Result<Vec<Product>, DatabaseError> {
    let mut statement = connection.prepare(
        "SELECT id, shop_id, category_id, url, name FROM product WHERE category_id = ?",
    )?;
    statement.bind(1, category_id)?;

    let mut result = Vec::new();

    while let State::Row = statement.next()? {
        let id = statement.read(0)?;
        let shop_id = statement.read(1)?;
        let category_id = statement.read(2)?;
        let url = statement.read(3)?;
        let name = statement.read(4)?;

        result.push(Product::new(id, shop_id, category_id, url, name));
    }

    Ok(result)
}

fn get_shops(connection: &mut Connection) -> Result<Vec<Shop>, DatabaseError> {
    let mut statement = connection.prepare("SELECT id, name FROM shop")?;
    let mut result = Vec::new();

    while let State::Row = statement.next()? {
        let id = statement.read(0)?;
        let name = statement.read(1)?;

        result.push(Shop::new(id, name));
    }

    Ok(result)
}

fn get_categories(connection: &mut Connection) -> Result<Vec<Category>, DatabaseError> {
    let mut statement = connection.prepare("SELECT id, name FROM category")?;
    let mut result = Vec::new();

    while let State::Row = statement.next()? {
        let id = statement.read(0)?;
        let name = statement.read(1)?;

        result.push(Category::new(id, name));
    }

    Ok(result)
}

fn get_products(connection: &mut Connection) -> Result<Vec<Product>, DatabaseError> {
    let mut statement = connection.prepare(
        "SELECT id, shop_id, category_id, url, name FROM product",
    )?;
    let mut result = Vec::new();

    while let State::Row = statement.next()? {
        let id = statement.read(0)?;
        let shop_id = statement.read(1)?;
        let category_id = statement.read(2)?;
        let url = statement.read(3)?;
        let name = statement.read(4)?;

        result.push(Product::new(id, shop_id, category_id, url, name));
    }

    Ok(result)
}

fn get_shop(connection: &mut Connection, id: i64) -> Result<Option<Shop>, DatabaseError> {
    let mut statement = connection.prepare("SELECT id, name FROM shop WHERE id = ?")?;
    statement.bind(1, id)?;

    if let State::Row = statement.next()? {
        let id = statement.read(0)?;
        let name = statement.read(1)?;

        Ok(Some(Shop::new(id, name)))
    } else {
        Ok(None)
    }
}

fn get_category(connection: &mut Connection, id: i64) -> Result<Option<Category>, DatabaseError> {
    let mut statement = connection.prepare(
        "SELECT id, name FROM category WHERE id = ?",
    )?;
    statement.bind(1, id)?;

    if let State::Row = statement.next()? {
        let id = statement.read(0)?;
        let name = statement.read(1)?;

        Ok(Some(Category::new(id, name)))
    } else {
        Ok(None)
    }
}

fn get_product(connection: &mut Connection, id: i64) -> Result<Option<Product>, DatabaseError> {
    let mut statement = connection.prepare(
        "SELECT id, shop_id, category_id, url, name FROM product WHERE id = ?",
    )?;
    statement.bind(1, id)?;

    if let State::Row = statement.next()? {
        let id = statement.read(0)?;
        let shop_id = statement.read(1)?;
        let category_id = statement.read(2)?;
        let url = statement.read(3)?;
        let name = statement.read(4)?;

        Ok(Some(Product::new(id, shop_id, category_id, url, name)))
    } else {
        Ok(None)
    }
}

fn get_product_prices(
    connection: &mut Connection,
    iteration_from: i64,
    iteration_to: i64,
) -> Result<Vec<ProductPrice>, DatabaseError> {
    let mut statement = connection.prepare(
        "SELECT id, product_id, iteration, timestamp, price FROM product_price WHERE iteration BETWEEN ? AND ?",
    )?;
    statement.bind(1, iteration_from)?;
    statement.bind(2, iteration_to)?;

    let mut result = Vec::new();

    while let State::Row = statement.next()? {
        let id = statement.read(0)?;
        let product_id = statement.read(1)?;
        let iteration = statement.read(2)?;
        let timestamp = statement.read(3)?;
        let price = statement.read(4)?;

        result.push(ProductPrice::new(
            id,
            product_id,
            iteration,
            timestamp,
            price,
        ));
    }

    Ok(result)
}

fn get_last_iteration(connection: &mut Connection) -> Result<Option<i64>, DatabaseError> {
    let mut statement = connection.prepare(
        "SELECT MAX(iteration) FROM product_price",
    )?;

    if let State::Row = statement.next()? {
        let last_iteration = statement.read(0)?;

        Ok(Some(last_iteration))
    } else {
        Ok(None)
    }
}

fn save_product_price(
    connection: &mut Connection,
    product_id: i64,
    iteration: i64,
    timestamp: i64,
    price: f64,
) -> Result<(), DatabaseError> {
    let statement = connection.prepare(
        "INSERT INTO product_price ( product_id, iteration, timestamp, price ) VALUES ( ?, ?, ?, ? )",
    )?;
    let mut cursor = statement.cursor();
    cursor.bind(
        &[
            Value::Integer(product_id),
            Value::Integer(iteration),
            Value::Integer(timestamp),
            Value::Float(price),
        ],
    )?;
    cursor.next()?;

    Ok(())
}

fn category_id(connection: &mut Connection, name: &String) -> Result<i64, DatabaseError> {
    let result;

    if let Some(id) = get_category_id(connection, name)? {
        result = id
    } else {
        save_category(connection, name)?;
        result = last_inserted_id(connection)?;
    }

    Ok(result)
}

fn shop_id(connection: &mut Connection, name: &String) -> Result<i64, DatabaseError> {
    let result;

    if let Some(id) = get_shop_id(connection, name)? {
        result = id
    } else {
        save_shop(connection, name)?;
        result = last_inserted_id(connection)?;
    }

    Ok(result)
}

pub fn product_id(
    connection: &mut Connection,
    shop_id: i64,
    category_id: i64,
    url: &String,
    name: &String,
) -> Result<i64, DatabaseError> {
    let result;

    if let Some(id) = get_product_id(connection, url)? {
        result = id
    } else {
        save_product(connection, shop_id, category_id, url, name)?;
        result = last_inserted_id(connection)?;
    }

    Ok(result)
}

fn get_product_id(connection: &mut Connection, url: &String) -> Result<Option<i64>, DatabaseError> {
    let mut statement = connection.prepare("SELECT id FROM product WHERE url = ?")?;
    statement.bind(1, url.as_str())?;

    if let State::Row = statement.next()? {
        let id = statement.read(0)?;

        Ok(Some(id))
    } else {
        Ok(None)
    }
}

fn save_product(
    connection: &mut Connection,
    shop_id: i64,
    category_id: i64,
    url: &String,
    name: &String,
) -> Result<(), DatabaseError> {
    let statement = connection.prepare(
        "INSERT INTO product ( shop_id, category_id, url, name ) VALUES ( ?, ?, ?, ? )",
    )?;
    let mut cursor = statement.cursor();
    cursor.bind(
        &[
            Value::Integer(shop_id),
            Value::Integer(category_id),
            Value::String(url.clone()),
            Value::String(name.clone()),
        ],
    )?;
    cursor.next()?;

    Ok(())
}

fn get_shop_id(connection: &mut Connection, name: &String) -> Result<Option<i64>, DatabaseError> {
    let mut statement = connection.prepare("SELECT id FROM shop WHERE name = ?")?;
    statement.bind(1, name.as_str())?;

    if let State::Row = statement.next()? {
        let id = statement.read(0)?;

        Ok(Some(id))
    } else {
        Ok(None)
    }
}

fn save_shop(connection: &mut Connection, name: &String) -> Result<(), DatabaseError> {
    let statement = connection.prepare("INSERT INTO shop ( name ) VALUES ( ? )")?;
    let mut cursor = statement.cursor();
    cursor.bind(&[Value::String(name.clone())])?;
    cursor.next()?;

    Ok(())
}

fn get_category_id(
    connection: &mut Connection,
    name: &String,
) -> Result<Option<i64>, DatabaseError> {
    let mut statement = connection.prepare("SELECT id FROM category WHERE name = ?")?;
    statement.bind(1, name.as_str())?;

    if let State::Row = statement.next()? {
        let id = statement.read(0)?;

        Ok(Some(id))
    } else {
        Ok(None)
    }
}

fn save_category(connection: &mut Connection, name: &String) -> Result<(), DatabaseError> {
    let statement = connection.prepare(
        "INSERT INTO category ( name ) VALUES ( ? )",
    )?;
    let mut cursor = statement.cursor();
    cursor.bind(&[Value::String(name.clone())])?;
    cursor.next()?;

    Ok(())
}

fn last_inserted_id(connection: &mut Connection) -> Result<i64, DatabaseError> {
    let mut query_id = connection.prepare("SELECT last_insert_rowid()")?;

    if let State::Row = query_id.next()? {
        Ok(query_id.read(0)?)
    } else {
        Err(DatabaseError::no_data())
    }
}
