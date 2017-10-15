use std::error::Error;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::io::Error as IoError;
use std::path::PathBuf;
use std::sync::PoisonError;

use sqlite::Error as SqlError;


#[derive(Debug)]
pub enum DatabaseError {
    IsDirectoryError { path: PathBuf },
    SqlError { description: String },
    IoError { description: String },
    LockError,
    NoData,
}


impl DatabaseError {
    pub fn is_directory(path: PathBuf) -> DatabaseError {
        DatabaseError::IsDirectoryError { path }
    }

    pub fn no_data() -> DatabaseError {
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
