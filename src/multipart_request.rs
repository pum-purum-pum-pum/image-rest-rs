use crate::err::SavingMultipartError;
use crate::misc::mini_from_buf;
use crate::misc::JsonImageResponse;
use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{error, web, Error, HttpResponse};
use futures::future::{err, Either};
use futures::{Future, Stream};
use image::{guess_format, ImageFormat};
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

/// upload multipart data and responce with a future stream of sizes
pub fn upload(multipart: Multipart) -> impl Future<Item = HttpResponse, Error = Error> {
    multipart
        .map(|field| save_file(field).into_stream())
        .flatten()
        .collect()
        .map(|responces| HttpResponse::Ok().json(responces.iter().collect::<Vec<_>>()))
        .from_err()
}

// error conversion functions
fn blocking_multipart_convert(e: error::BlockingError<MultipartError>) -> MultipartError {
    match e {
        error::BlockingError::Error(e) => e,
        error::BlockingError::Canceled => MultipartError::Incomplete,
    }
}

fn blocking_savemultipart_convert(
    e: error::BlockingError<SavingMultipartError>,
) -> SavingMultipartError {
    match e {
        error::BlockingError::Error(e) => e,
        error::BlockingError::Canceled => {
            SavingMultipartError::Multipart(MultipartError::Incomplete)
        }
    }
}

/// create and save mini preview image
fn save_mini(filename: &str, image_buffer: Vec<u8>) -> Result<(), SavingMultipartError> {
    let format = guess_format(&image_buffer).unwrap_or(ImageFormat::PNG);
    let mini_image = mini_from_buf(&image_buffer, format).map_err(SavingMultipartError::Image);
    let mini_filename = format!("preview_{}", filename);
    let path = Path::new(&mini_filename);
    mini_image.and_then(|image| image.save(path).map_err(SavingMultipartError::Io))?;
    Ok(())
}

pub fn save_file(field: Field) -> impl Future<Item = JsonImageResponse, Error = Error> {
    let extension = match field
        .content_disposition()
        .and_then(|content| content.get_filename().map(|s| s.to_string()))
    {
        Some(filename) => Path::new(&filename)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("png")
            .to_string(),
        None => {
            return Either::A(err(error::ErrorInternalServerError(
                "Couldn't read the filename.",
            )))
        }
    };
    let image_buffer = vec![];
    let file_path_string = format!("{}.{}", Uuid::new_v4(), extension);
    let file = match fs::File::create(file_path_string.clone()) {
        Ok(file) => file,
        Err(e) => return Either::A(err(error::ErrorInternalServerError(e))),
    };
    Either::B(
        field
            .fold(
                // stream to file and image_buffer
                (file, file_path_string.clone(), image_buffer, 0u64),
                move |(mut file, filename, mut image_buffer, mut acc), bytes| {
                    web::block(move || {
                        file.write_all(bytes.as_ref())
                            .map_err(|e| MultipartError::Payload(error::PayloadError::Io(e)))?;
                        image_buffer.extend_from_slice(bytes.as_ref());
                        acc += bytes.len() as u64;
                        Ok((file, filename, image_buffer, acc))
                    })
                    .map_err(blocking_multipart_convert)
                },
            )
            // using this error type as enum of multipart and image processing errors
            .from_err()
            .and_then(|(_file, filename, image_buffer, acc)| {
                web::block(move || {
                    save_mini(&filename, image_buffer)?;
                    Ok(JsonImageResponse {
                        name: filename,
                        checksum: acc,
                    })
                })
                .map_err(blocking_savemultipart_convert)
            })
            .from_err(),
    )
}
