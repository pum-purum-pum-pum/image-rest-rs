use crate::MINI_IMAGE_DIM;
use image::GenericImageView;
use serde_derive::{Deserialize, Serialize};
use actix_web::{HttpResponse};

#[derive(Deserialize, Serialize)]
pub struct UrlFormData {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonImageResponse {
    pub name: String,
    pub checksum: u64,
}

use image::{load_from_memory_with_format, DynamicImage, FilterType, ImageFormat, ImageResult};

pub fn minimize_image(image: &DynamicImage) -> ImageResult<DynamicImage> {
    let dimensions = image.dimensions();
    if dimensions.0 < MINI_IMAGE_DIM && dimensions.1 < MINI_IMAGE_DIM {
        return Ok(image.clone());
    }
    let image = image.resize(MINI_IMAGE_DIM, MINI_IMAGE_DIM, FilterType::Triangle);
    Ok(image)
}

pub fn mini_from_buf(buf: &[u8], format: ImageFormat) -> ImageResult<DynamicImage> {
    let image = load_from_memory_with_format(buf, format)?;
    minimize_image(&image)
}

pub fn index() -> HttpResponse {
    let html = include_str!("../html/multipart.html");
    HttpResponse::Ok().body(html)
}

pub fn url_form() -> HttpResponse {
    let html = include_str!("../html/upload_url.html");
    HttpResponse::Ok().body(html)
}

pub fn preview_form() -> HttpResponse {
    let html = include_str!("../html/preview.html");
    HttpResponse::Ok().body(html)
}