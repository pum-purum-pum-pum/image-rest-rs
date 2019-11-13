use std::path::Path;

use actix_web::{web, Error, HttpResponse};

use actix_web::client::{Client, SendRequestError};
use actix_web::error::{ErrorBadGateway, ErrorInternalServerError};

use crate::err::ImageProcessError;
use crate::misc::UrlFormData;
use crate::MAX_IMAGE_SIZE;
use futures::Future;
use image::{self, load_from_memory};
use std::ffi::OsStr;
use uuid::Uuid;

pub fn image_save(bytes: &[u8], extension: &str) -> Result<String, ImageProcessError> {
    let file_name = format!("{}.{}", Uuid::new_v4(), extension);
    let path = Path::new(&file_name);
    load_from_memory(&bytes)
        .map_err(ImageProcessError::ImageError)
        .and_then(|file| file.save(path).map_err(ImageProcessError::IOError))
        .map(|_| file_name)
}

pub fn image_url_save(
    param: web::Form<UrlFormData>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let client = Client::default();
    let extension = Path::new(&param.url)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("png")
        .to_string();
    client
        .get(&param.url)
        .header("User-Agent", "Actix-web")
        .send()
        .map_err(|err| match err {
            SendRequestError::Connect(error) => {
                ErrorBadGateway(format!("Unable to connect to httpbin: {}", error))
            }
            error => ErrorInternalServerError(error),
        })
        .and_then(|mut response| {
            response
                .body()
                .limit(MAX_IMAGE_SIZE)
                .from_err()
                .and_then(move |bytes| {
                    web::block(move || image_save(&bytes, &extension))
                        .from_err()
                        .and_then(|file_name| {
                            Ok(HttpResponse::Ok()
                                .content_type("text/html")
                                .body(file_name.to_string()))
                        })
                })
        })
}
