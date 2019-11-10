use std::path::Path;

use actix_web::{web, Error, HttpResponse};

use actix_web::client::{Client, SendRequestError};
use actix_web::error::{ErrorBadGateway, ErrorInternalServerError};

use crate::err::ImageProcessError;
use crate::MAX_IMAGE_SIZE;
use futures::Future;
use image::{self, load_from_memory};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct UrlFormData {
    pub url: String,
}

pub fn image_save(bytes: &[u8]) -> Result<String, ImageProcessError> {
    let file_name = format!("{}.png", Uuid::new_v4());
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
                    // let s = std::str::from_utf8(&bytes).expect("utf8 parse error)");
                    web::block(move || image_save(&bytes))
                        .from_err()
                        .and_then(|file_name| {
                            Ok(HttpResponse::Ok()
                                .content_type("text/html")
                                .body(file_name.to_string()))
                        })
                })
        })
}
