use clap::{load_yaml, App as Clapp};
use actix_web::{middleware, web, App, HttpServer};

use image_preview_request::image_preview;
use json_request::image_json_save;
use multipart_request::upload;
use url_request::image_url_save;
use misc::{index, url_form, preview_form};
use std::fs;

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

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    let yaml = load_yaml!("cli.yml");
    let matches = Clapp::from_yaml(yaml).get_matches();
    let port = matches.value_of("port").unwrap_or("8000");
    let save_dir = matches.value_of("out").unwrap_or("images").to_string();
    fs::create_dir_all(save_dir.clone())?;
    HttpServer::new(move || {
        App::new()
            .data(save_dir.clone())
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
    .bind(format!("0.0.0.0:{}", port))?
    .run()
}
