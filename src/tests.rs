use crate::json_request::{image_json_save, ImageBase64, JsonImageResponse};
use crate::{index, url_form};
use actix_web::http::StatusCode;
use actix_web::test;
use actix_web::{http::header, web, App};
use bytes::Bytes;
use once_cell::sync::Lazy;

/// const useful for testing json request
static PIXEL_BASE64: Lazy<String> = Lazy::new(|| {
    let image_base64 = ImageBase64 {
      content: "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==".to_string()  
    };
    let image_base64 = serde_json::to_string(&image_base64).unwrap();
    image_base64
});

/// sanity check
#[test]
fn index_page() {
    let resp = test::block_on(index()).unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

/// sanity check
#[test]
fn url_page() {
    let resp = test::block_on(url_form()).unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[test]
fn json_request() {
    let mut app = test::init_service(
        App::new()
            .service(web::resource("/image_json").route(web::post().to_async(image_json_save))),
    );
    let req = test::TestRequest::post()
        .uri("/image_json")
        .header(header::CONTENT_TYPE, "application/json")
        .set_payload(Bytes::from_static(PIXEL_BASE64.as_bytes()))
        .to_request();
    let result = test::read_response(&mut app, req);
    let result = serde_json::from_slice::<JsonImageResponse>(&result).unwrap();
    assert_eq!(result.checksum, PIXEL_BASE64.len());
}
