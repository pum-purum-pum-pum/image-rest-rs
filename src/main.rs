use clap::{load_yaml, App as Clapp};

use std::cell::Cell;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use image_preview_request::image_preview;
use json_request::image_json_save;
use multipart_request::upload;
use url_request::image_url_save;

mod err;
mod image_preview_request;
mod json_request;
mod misc;
mod multipart_request;
#[cfg(test)]
mod tests;
mod url_request;

pub const MAX_SIZE: usize = 262_144; // max payload size is 256k
pub const MAX_IMAGE_SIZE: usize = 2 * 1024 * 1024;
pub const MINI_IMAGE_DIM: u32 = 100;

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
