use actix_web::{web, Error, HttpResponse};
use std::path::Path;

use crate::err::ImageProcessError;
use crate::misc::{minimize_image, JsonImageResponse};
use futures::{Future, Stream};
use image::{self, load_from_memory};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageBase64 {
    pub content: String,
    pub extension: String,
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
                let extension = obj.extension;
                let file_name = format!("{}.{}", Uuid::new_v4(), extension);
                let preview_file_name = format!("preview_{}.{}", Uuid::new_v4(), extension);
                let path = Path::new(&file_name);
                let preview_path = Path::new(&preview_file_name);
                let img = load_from_memory(&bytes).map_err(ImageProcessError::ImageError);
                img.and_then(|img| {
                    minimize_image(&img)
                        .map_err(ImageProcessError::ImageError)
                        .and_then(|mini_img| {
                            mini_img
                                .save(preview_path)
                                .map_err(ImageProcessError::IOError)
                        })
                        .and(img.save(path).map_err(ImageProcessError::IOError))
                })
                .map(|_| (file_name, body.len()))
            })
            .from_err()
        })
        .and_then(|(file_name, checksum)| {
            let response = JsonImageResponse {
                name: file_name,
                checksum: checksum as u64,
            };
            Ok(HttpResponse::Ok().json(response))
        })
}
