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

use super::DatabaseError;
use super::IterationPrice;
use super::Product;
use super::ProductPrice;


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
        shop: &str,
        category: &str,
        product_url: &str,
        product_name: &str,
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

    pub fn iteration(&self) -> Result<Option<i64>, DatabaseError> {
        let mut connection = self.connection.lock()?;
        let iteration = get_iteration(&mut connection)?;

        Ok(iteration)
    }

    pub fn save_iteration(&self, iteration: i64) -> Result<(), DatabaseError> {
        let mut connection = self.connection.lock()?;
        update_iteration(&mut connection, iteration)?;

        Ok(())
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

    pub fn product_price_by_iteration(
        &self,
        iteration_from: i64,
        iteration_to: i64,
    ) -> Result<Vec<IterationPrice>, DatabaseError> {
        let mut connection = self.connection.lock()?;
        let product_prices =
            get_product_price_by_iteration(&mut connection, iteration_from, iteration_to)?;

        Ok(product_prices)
    }
}


fn get_product_price_by_iteration(
    connection: &mut Connection,
    iteration_from: i64,
    iteration_to: i64,
) -> Result<Vec<IterationPrice>, DatabaseError> {
    let mut statement = connection.prepare(
        r#"
SELECT
    p.category_id,
    c.name,
    p.name,
    p.url,
    s.name,
    pp.price,
    pp.timestamp
FROM product_price AS pp
    INNER JOIN product AS p ON ( p.id = pp.product_id )
    INNER JOIN category AS c ON ( c.id = p.category_id )
    INNER JOIN shop as s ON ( s.id = p.shop_id )
WHERE pp.iteration BETWEEN ? AND ?
"#,
    )?;
    statement.bind(1, iteration_from)?;
    statement.bind(2, iteration_to)?;

    let mut result = Vec::new();

    while let State::Row = statement.next()? {
        let category_id = statement.read(0)?;
        let category = statement.read(1)?;
        let product = statement.read(2)?;
        let url = statement.read(3)?;
        let shop = statement.read(4)?;
        let price = statement.read(5)?;
        let timestamp = statement.read(6)?;

        result.push(IterationPrice::new(
            category_id,
            category,
            product,
            url,
            shop,
            price,
            timestamp,
        ));
    }

    Ok(result)
}

fn get_product_prices_by_product(
    connection: &mut Connection,
    product_id: i64,
) -> Result<Vec<ProductPrice>, DatabaseError> {
    let mut statement = connection.prepare(
        "SELECT iteration, timestamp, price FROM product_price WHERE product_id = ?",
    )?;
    statement.bind(1, product_id)?;

    let mut result = Vec::new();

    while let State::Row = statement.next()? {
        let iteration = statement.read(0)?;
        let timestamp = statement.read(1)?;
        let price = statement.read(2)?;

        result.push(ProductPrice::new(iteration, timestamp, price));
    }

    Ok(result)
}

fn get_products_by_category(
    connection: &mut Connection,
    category_id: i64,
) -> Result<Vec<Product>, DatabaseError> {
    let mut statement = connection.prepare(
        "SELECT id, name FROM product WHERE category_id = ?",
    )?;
    statement.bind(1, category_id)?;

    let mut result = Vec::new();

    while let State::Row = statement.next()? {
        let id = statement.read(0)?;
        let name = statement.read(1)?;

        result.push(Product::new(id, name));
    }

    Ok(result)
}

fn get_iteration(connection: &mut Connection) -> Result<Option<i64>, DatabaseError> {
    let mut statement = connection.prepare(
        "SELECT iteration FROM iteration LIMIT 1",
    )?;

    if let State::Row = statement.next()? {
        let last_iteration = statement.read(0)?;

        Ok(Some(last_iteration))
    } else {
        Ok(None)
    }
}

fn update_iteration(connection: &mut Connection, iteration: i64) -> Result<(), DatabaseError> {
    let statement = connection.prepare("UPDATE iteration SET iteration = ?")?;
    let mut cursor = statement.cursor();
    cursor.bind(&[Value::Integer(iteration)])?;
    cursor.next()?;

    Ok(())
}

fn save_product_price(
    connection: &mut Connection,
    product_id: i64,
    iteration: i64,
    timestamp: i64,
    price: f64,
) -> Result<(), DatabaseError> {
    let statement = connection.prepare(
        r#"
INSERT INTO product_price ( product_id, iteration, timestamp, price )
VALUES ( ?, ?, ?, ? )
"#,
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

fn category_id(connection: &mut Connection, name: &str) -> Result<i64, DatabaseError> {
    let result;

    if let Some(id) = get_category_id(connection, name)? {
        result = id
    } else {
        save_category(connection, name)?;
        result = last_inserted_id(connection)?;
    }

    Ok(result)
}

fn shop_id(connection: &mut Connection, name: &str) -> Result<i64, DatabaseError> {
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
    url: &str,
    name: &str,
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

fn get_product_id(connection: &mut Connection, url: &str) -> Result<Option<i64>, DatabaseError> {
    let mut statement = connection.prepare("SELECT id FROM product WHERE url = ?")?;
    statement.bind(1, url)?;

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
    url: &str,
    name: &str,
) -> Result<(), DatabaseError> {
    let statement = connection.prepare(
        "INSERT INTO product ( shop_id, category_id, url, name ) VALUES ( ?, ?, ?, ? )",
    )?;
    let mut cursor = statement.cursor();
    cursor.bind(
        &[
            Value::Integer(shop_id),
            Value::Integer(category_id),
            Value::String(url.into()),
            Value::String(name.into()),
        ],
    )?;
    cursor.next()?;

    Ok(())
}

fn get_shop_id(connection: &mut Connection, name: &str) -> Result<Option<i64>, DatabaseError> {
    let mut statement = connection.prepare("SELECT id FROM shop WHERE name = ?")?;
    statement.bind(1, name)?;

    if let State::Row = statement.next()? {
        let id = statement.read(0)?;

        Ok(Some(id))
    } else {
        Ok(None)
    }
}

fn save_shop(connection: &mut Connection, name: &str) -> Result<(), DatabaseError> {
    let statement = connection.prepare("INSERT INTO shop ( name ) VALUES ( ? )")?;
    let mut cursor = statement.cursor();
    cursor.bind(&[Value::String(name.into())])?;
    cursor.next()?;

    Ok(())
}

fn get_category_id(connection: &mut Connection, name: &str) -> Result<Option<i64>, DatabaseError> {
    let mut statement = connection.prepare("SELECT id FROM category WHERE name = ?")?;
    statement.bind(1, name)?;

    if let State::Row = statement.next()? {
        let id = statement.read(0)?;

        Ok(Some(id))
    } else {
        Ok(None)
    }
}

fn save_category(connection: &mut Connection, name: &str) -> Result<(), DatabaseError> {
    let statement = connection.prepare(
        "INSERT INTO category ( name ) VALUES ( ? )",
    )?;
    let mut cursor = statement.cursor();
    cursor.bind(&[Value::String(name.into())])?;
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
