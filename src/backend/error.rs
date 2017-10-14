use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::error::Error;

use iron::error::HttpError;


#[derive(Debug)]
pub enum BackendError {
    IronError { description: String },
}


impl Display for BackendError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            &BackendError::IronError { ref description } => {
                write!(f, "Iron error: {}", description)
            }
        }
    }
}


impl Error for BackendError {
    fn description(&self) -> &str {
        match self {
            &BackendError::IronError { ref description } => &description,
        }
    }
}


impl From<HttpError> for BackendError {
    fn from(error: HttpError) -> BackendError {
        BackendError::IronError { description: error.description().into() }
    }
}
