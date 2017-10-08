use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fs;
use std::io::Error as IoError;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::PoisonError;

use sqlite::Connection;
use sqlite::Error as SqlError;
use sqlite::State;
use sqlite::Value;
use sqlite;


#[derive(Clone)]
pub struct Database {
    connection: Arc<Mutex<Connection>>,
}


#[derive(Debug, Clone)]
pub enum DatabaseError {
    IsDirectoryError { path: PathBuf },
    SqlError { description: String },
    IoError { description: String },
    LockError,
    NoData,
}


impl DatabaseError {
    fn is_directory(path: PathBuf) -> DatabaseError {
        DatabaseError::IsDirectoryError { path }
    }

    fn no_data() -> DatabaseError {
        DatabaseError::NoData
    }
}


impl From<IoError> for DatabaseError {
    fn from(error: IoError) -> DatabaseError {
        DatabaseError::IoError { description: error.description().into() }
    }
}


impl From<SqlError> for DatabaseError {
    fn from(error: SqlError) -> DatabaseError {
        DatabaseError::SqlError { description: error.description().into() }
    }
}


impl<T> From<PoisonError<T>> for DatabaseError {
    fn from(_: PoisonError<T>) -> DatabaseError {
        DatabaseError::LockError
    }
}


impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            &DatabaseError::IsDirectoryError { ref path } => {
                write!(f, "Database path is directory: {}", path.display())
            }
            &DatabaseError::SqlError { ref description } => write!(f, "SQL error: {}", description),
            &DatabaseError::IoError { ref description } => write!(f, "IO error: {}", description),
            &DatabaseError::LockError => write!(f, "Mutex lock error"),
            &DatabaseError::NoData => write!(f, "No data"),
        }
    }
}


impl Error for DatabaseError {
    fn description(&self) -> &str {
        match self {
            &DatabaseError::IsDirectoryError { .. } => "Database path is directory",
            &DatabaseError::SqlError { .. } => "SQL error",
            &DatabaseError::IoError { .. } => "IO error",
            &DatabaseError::LockError => "Mutex lock error",
            &DatabaseError::NoData => "No data",
        }
    }
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
        product: &String,
        timestamp: i64,
        price: f64,
    ) -> Result<(), DatabaseError> {
        let mut connection = self.connection.lock()?;
        let shop_id = Database::shop_id(&mut connection, shop)?;
        let category_id = Database::category_id(&mut connection, category)?;
        let product_id = Database::product_id(&mut connection, shop_id, category_id, product)?;

        Database::save_product_price(&mut connection, product_id, timestamp, price)?;

        Ok(())
    }

    fn save_product_price(
        connection: &mut Connection,
        product_id: i64,
        timestamp: i64,
        price: f64,
    ) -> Result<(), DatabaseError> {
        let statement = connection.prepare(
            "INSERT INTO product_price ( product_id, timestamp, price ) VALUES ( ?, ?, ? )",
        )?;
        let mut cursor = statement.cursor();
        cursor.bind(
            &[
                Value::Integer(product_id),
                Value::Integer(timestamp),
                Value::Float(price),
            ],
        )?;
        cursor.next()?;

        Ok(())
    }

    fn category_id(connection: &mut Connection, name: &String) -> Result<i64, DatabaseError> {
        let result;

        if let Some(id) = Database::get_category_id(connection, name)? {
            result = id
        } else {
            Database::save_category(connection, name)?;
            result = Database::last_inserted_id(connection)?;
        }

        Ok(result)
    }

    fn shop_id(connection: &mut Connection, name: &String) -> Result<i64, DatabaseError> {
        let result;

        if let Some(id) = Database::get_shop_id(connection, name)? {
            result = id
        } else {
            Database::save_shop(connection, name)?;
            result = Database::last_inserted_id(connection)?;
        }

        Ok(result)
    }

    pub fn product_id(
        connection: &mut Connection,
        shop_id: i64,
        category_id: i64,
        name: &String,
    ) -> Result<i64, DatabaseError> {
        let result;

        if let Some(id) = Database::get_product_id(connection, name)? {
            result = id
        } else {
            Database::save_product(connection, shop_id, category_id, name)?;
            result = Database::last_inserted_id(connection)?;
        }

        Ok(result)
    }

    fn get_product_id(
        connection: &mut Connection,
        name: &String,
    ) -> Result<Option<i64>, DatabaseError> {
        let mut statement = connection.prepare("SELECT id FROM product WHERE name = ?")?;
        statement.bind(1, name.as_str())?;

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
        name: &String,
    ) -> Result<(), DatabaseError> {
        let statement = connection.prepare(
            "INSERT INTO product ( shop_id, category_id, name ) VALUES ( ?, ?, ? )",
        )?;
        let mut cursor = statement.cursor();
        cursor.bind(
            &[
                Value::Integer(shop_id),
                Value::Integer(category_id),
                Value::String(name.clone()),
            ],
        )?;
        cursor.next()?;

        Ok(())
    }

    fn get_shop_id(
        connection: &mut Connection,
        name: &String,
    ) -> Result<Option<i64>, DatabaseError> {
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
}
