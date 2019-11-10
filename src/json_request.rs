use actix_web::{error, web, Error, HttpResponse};
use std::io::{Error as IOError, ErrorKind};
use std::path::Path;

use crate::err::ImageProcessError;
use futures::{Future, Stream};
use image::{self, load_from_memory};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageBase64 {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonImageResponse {
    pub name: String,
    pub checksum: usize,
}

pub fn image_json_save(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {
    // payload is a stream of Bytes objects
    payload
        .from_err()
        .concat2()
        .and_then(|body| {
            // body is loaded, now we can deserialize serde-json
            web::block(move || {
                let obj = serde_json::from_slice::<ImageBase64>(&body)?;
                let bytes = base64::decode(&obj.content).map_err(ImageProcessError::DecodeError)?;
                let file_name = format!("{}.png", Uuid::new_v4());
                let path = Path::new(&file_name);
                load_from_memory(&bytes)
                    .map_err(ImageProcessError::ImageError)
                    .and_then(|img| img.save(path).map_err(ImageProcessError::IOError))
                    .map(|_| (file_name, body.len()))
            })
            .map_err(|e: error::BlockingError<ImageProcessError>| match e {
                error::BlockingError::Error(e) => e,
                error::BlockingError::Canceled => ImageProcessError::IOError(IOError::new(
                    ErrorKind::Other,
                    "saving operation canceled",
                )),
            })
            .from_err()
        })
        .and_then(|(file_name, checksum)| {
            let response = JsonImageResponse {
                name: file_name,
                checksum,
            };
            Ok(HttpResponse::Ok().json(response))
        })
}
