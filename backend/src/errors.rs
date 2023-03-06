use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;

/// Represents an error occured in the Application
#[derive(Debug, Serialize)]
pub enum TutorError {
    /// Error from the database
    DBError(String),
    /// Error from Actix framework
    ActixError(String),
    /// Error from not found resource
    NotFound(String),
    /// Error for invalid input parameters.
    InvalidInput(String),
}

/// Represents an error message to be send as a response to the user or client.
#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    /// Error messsage
    error_message: String,
}

impl TutorError {
    fn error_response(&self) -> String {
        match self {
            TutorError::DBError(message) => {
                println!("A database error has occured: {message}");
                String::from("Database error")
            }
            TutorError::Actix(message) => {
                println!("An Actix server error has occured: {message}");
                String::from("Internal Server Error")
            }
            TutorError::NotFound(message) => {
                println!("A Not Found error has occured: {message}");
                message.into()
            }
            TutorError::InvalidInput(message) => {
                println!("Invalid parameters were given: {message}");
                message.into()
            }
        }
    }
}

impl actix_web::error::ResponseError for TutorError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            TutorError::DBError(_msg) | TutorError::ActixError(_msg) => {
                StatusCode::INTERNAL_SErVER_ERROR
            }
            TutorError::NotFound(_msg) => StatusCode::NOT_FOUND,
            TutorError::InvalidInput(_msg) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl std::fmt::Display for TutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DBError(msg) => write!(f, "TutorError::DBError : {msg}"),
            Self::ActixError(msg) => write!(f, "TutorError::ActixError : {msg}"),
            Self::NotFound(msg) => write!(f, "TutorError::NotFound : {msg}"),
            Self::InvalidInput(msg) => write!(f, "TutorError::InvalidInput : {msg}"),
        }
    }
}

impl From<actix_web::error::Error> for TutorError {
    fn from(value: actix_web::error::Error) -> Self {
        TutorError::ActixError(value.to_string())
    }
}

impl From<sqlx::error::Error> for TutorError {
    fn from(value: sqlx::error::Error) -> Self {
        TutorError::DBError(value.to_string())
    }
}
