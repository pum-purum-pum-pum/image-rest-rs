use actix_web::http::header::{HeaderValue, CONTENT_TYPE};
use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpResponse};
use futures::Future;
use image::{self};
use std::fs;
use std::io::BufReader;
use std::path::Path;

pub fn image_preview() -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        println!("loading image");
        let path = Path::new("file.png");
        let img = image::load(
            BufReader::new(fs::File::open(path)?),
            image::ImageFormat::PNG,
        );
        img.and_then(|img| {
            // TODO block!
            let mut buffer = Vec::new();
            img.write_to(&mut buffer, image::ImageFormat::PNG)?;
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
