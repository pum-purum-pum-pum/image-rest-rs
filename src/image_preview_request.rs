use actix_web::http::header::{HeaderValue, CONTENT_TYPE};
use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpResponse};
use futures::Future;
use image::ImageFormat;
use image::{self};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::io::BufReader;
use std::io::{Error as IoError, ErrorKind};
use std::path::Path;

#[derive(Deserialize, Serialize)]
pub struct FileNameFormData {
    pub name: String,
}

pub fn image_preview(
    param: web::Form<FileNameFormData>,
    save_dir: web::Data<String>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let in_path = Path::new(&param.name);
        let file_name_error = IoError::new(ErrorKind::Other, "file name error");
        let invalid_char = IoError::new(ErrorKind::Other, "invalied character in filename");
        // for safety convert input to filename
        let image_path = format!(
            "{}/{}",
            save_dir.to_string(),
            in_path
                .file_name()
                .ok_or(file_name_error)?
                .to_str()
                .ok_or(invalid_char)?
        );
        let path = Path::new(&image_path);
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
