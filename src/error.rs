use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Multipart error: {0}")]
    Multipart(#[from] actix_multipart::MultipartError),
    
    #[error("Invalid file format")]
    InvalidFileFormat,
    
    #[error("File not found")]
    FileNotFound,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Io(_) => HttpResponse::InternalServerError().body("Internal server error"),
            AppError::Multipart(_) => HttpResponse::BadRequest().body("Invalid request format"),
            AppError::InvalidFileFormat => HttpResponse::BadRequest().body("Invalid file format"),
            AppError::FileNotFound => HttpResponse::NotFound().body("File not found"),
        }
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
