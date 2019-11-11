use actix_multipart::MultipartError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::ResponseError;
use base64::DecodeError;
use image::ImageError;
use serde_json::error::Error as JsonError;
use std::fmt;

/// errors related to image loading and saving (possible to replace with dynamic error)
#[derive(Debug)]
pub enum ImageProcessError {
    ImageError(image::ImageError),
    IOError(std::io::Error),
    DecodeError(DecodeError),
    JsonError(JsonError),
}

#[derive(Debug)]
pub enum SavingMultipartError {
    Multipart(MultipartError),
    Io(std::io::Error),
    Image(ImageError),
}

impl fmt::Display for SavingMultipartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{:?}", self)
    }
}

impl ResponseError for SavingMultipartError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::new(StatusCode::BAD_REQUEST)
    }
}

impl From<MultipartError> for SavingMultipartError {
    fn from(item: MultipartError) -> Self {
        SavingMultipartError::Multipart(item)
    }
}

impl fmt::Display for ImageProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{:?}", self)
    }
}

impl From<image::ImageError> for ImageProcessError {
    fn from(item: image::ImageError) -> Self {
        ImageProcessError::ImageError(item)
    }
}

impl From<std::io::Error> for ImageProcessError {
    fn from(item: std::io::Error) -> Self {
        ImageProcessError::IOError(item)
    }
}

impl From<DecodeError> for ImageProcessError {
    fn from(item: DecodeError) -> Self {
        ImageProcessError::DecodeError(item)
    }
}

impl From<JsonError> for ImageProcessError {
    fn from(item: JsonError) -> Self {
        ImageProcessError::JsonError(item)
    }
}

impl ResponseError for ImageProcessError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::new(StatusCode::BAD_REQUEST)
    }
}
