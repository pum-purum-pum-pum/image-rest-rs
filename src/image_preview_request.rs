use actix_web::http::header::{HeaderValue, CONTENT_TYPE};
use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpResponse};
use futures::Future;
use image::ImageFormat;
use image::{self};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Serialize)]
pub struct FileNameFormData {
    pub name: String,
}

pub fn image_preview(
    param: web::Form<FileNameFormData>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let path = Path::new(&param.name);
        let image_format = ImageFormat::from_path(path)?;
        let img = image::load(BufReader::new(fs::File::open(path)?), image_format);
        img.and_then(|img| {
            // TODO block!
            let mut buffer = Vec::new();
            img.write_to(&mut buffer, image_format)?;
            Ok(buffer)
        })
    })
    .map(|buffer| {
        let mut response = HttpResponse::with_body(StatusCode::OK, buffer.into());
        response
            .headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static("image/jpeg"));
        response
    })
    .from_err()
}