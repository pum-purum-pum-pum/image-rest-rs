return filename from multipart

replace url form data with path form data for preview

save image also async?

check images url?

test!

future should return size as result item

fix:
	replace html with forms or save in file (and include bytes?)
	100 x 100 preview
check injections

actix logging

replace fold with concat2?

try .from_err() instead of converting errors directly



https://cdn.pixabay.com/photo/2016/03/02/13/59/bird-1232416_960_720.png
https://cdn.pixabay.com/photo/2017/01/03/02/07/vine-1948358_960_720.png




// #[test]
// fn multipart() {
//     let mut app = test::init_service(
//         App::new()
//             .service(web::resource("/").route(web::post().to_async(upload))),
//     );
//     let req = test::TestRequest::post()
//         .uri("/")
//         .header(header::CONTENT_TYPE, "application/json")
//         // .set_payload(Bytes::from_static(PIXEL_BASE64.as_bytes()))
//         .to_request();
//     let result = test::read_response(&mut app, req);
//     let result = serde_json::from_slice::<JsonImageResponse>(&result).unwrap();
//     assert_eq!(result.checksum, PIXEL_BASE64.len());
// }

// #[test]
// fn url_test() {
    // HttpServer::new(|| {
    //     App::new()
    //         .data(Cell::new(0usize))
    //         .wrap(middleware::Logger::default())
    //         .service(
    //             web::resource("/image_preivew")
    //                 .route(web::get().to(url_form))
    //                 .route(web::post().to_async(image_preview))
    //         )
    //         .service(
    //             web::resource("/image_url")
    //                 .route(web::get().to(url_form))
    //                 .route(web::post().to_async(image_url_save)),
    //         )
    // })
    // .bind(format!("127.0.0.1:8080")).unwrap()
    // .start();
    // let client = Client::default();
    // use futures::Future;
    // client
    //     .get("127.0.0.1:8080/image_preivew")
    //     .header("User-Agent", "Actix-web")
    //     .send_form(&UrlFormData{url: "test.png".to_string()}).poll().unwrap();


    // let mut app = test::init_service(
    //     App::new()
    //         .service(web::resource("/image_preivew").route(web::post().to_async(image_preview)))
    //         .service(web::resource("/image_url").route(web::post().to_async(image_url_save))),
    // );
    // let form = UrlFormData {
    //     url: "test.png".to_string()
    // };
    // let req = test::TestRequest::post()
    //     .uri("/image_preivew")
    //     .header(header::CONTENT_TYPE, "multipart/form-data")
    //     .set_form(&form)
    //     .to_request();
    // let result = test::read_response(&mut app, req);
    // let loaded_img = load_from_memory(&result).unwrap();
    // let path = Path::new(&form.url);
    // let img = image::load(
    //     BufReader::new(fs::File::open(path).unwrap()),
    //     image::ImageFormat::PNG,
    // ).unwrap();
    // assert_eq!(convert_image(img).unwrap(), convert_image(loaded_img).unwrap());
// }