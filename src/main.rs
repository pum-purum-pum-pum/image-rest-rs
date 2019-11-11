use clap::{load_yaml, App as Clapp};

use std::cell::Cell;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use image::GenericImageView;
use image_preview_request::image_preview;
use json_request::image_json_save;
use multipart_request::upload;
use serde_derive::{Deserialize, Serialize};
use url_request::image_url_save;

mod err;
mod image_preview_request;
mod json_request;
mod multipart_request;
#[cfg(test)]
mod tests;
mod url_request;

pub const MAX_SIZE: usize = 262_144; // max payload size is 256k
pub const MAX_IMAGE_SIZE: usize = 2 * 1024 * 1024;
pub const MINI_IMAGE_DIM: u32 = 100;

#[derive(Deserialize, Serialize)]
pub struct UrlFormData {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonImageResponse {
    pub name: String,
    pub checksum: u64,
}

pub fn minimize_image(image: &DynamicImage) -> ImageResult<DynamicImage> {
    let dimensions = image.dimensions();
    if dimensions.0 < MINI_IMAGE_DIM && dimensions.0 < MINI_IMAGE_DIM {
        return Ok(image.clone());
    }
    let image = image.resize(MINI_IMAGE_DIM, MINI_IMAGE_DIM, FilterType::Triangle);
    Ok(image)
}

use image::{load_from_memory_with_format, DynamicImage, FilterType, ImageFormat, ImageResult};
pub fn mini_from_buf(buf: &[u8], format: ImageFormat) -> ImageResult<DynamicImage> {
    let image = load_from_memory_with_format(buf, format)?;
    minimize_image(&image)
}

pub fn index() -> HttpResponse {
    let html = include_str!("../html/multipart.html");
    HttpResponse::Ok().body(html)
}

fn url_form() -> HttpResponse {
    let html = include_str!("../html/upload_url.html");
    HttpResponse::Ok().body(html)
}

fn preview_form() -> HttpResponse {
    let html = include_str!("../html/preview.html");
    HttpResponse::Ok().body(html)
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    // env_logger::init();
    let yaml = load_yaml!("cli.yml");
    let matches = Clapp::from_yaml(yaml).get_matches();
    let port = matches.value_of("port").unwrap_or("8080");
    HttpServer::new(|| {
        App::new()
            .data(Cell::new(0usize))
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/")
                    .route(web::get().to(index))
                    .route(web::post().to_async(upload)),
            )
            .service(web::resource("/image_json").route(web::post().to_async(image_json_save)))
            .service(
                web::resource("/image_url")
                    .route(web::get().to(url_form))
                    .route(web::post().to_async(image_url_save)),
            )
            .service(
                web::resource("/image_preview")
                    .route(web::get().to(preview_form))
                    .route(web::post().to_async(image_preview)),
            )
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
}
