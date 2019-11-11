use crate::JsonImageResponse;
use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{error, web, Error, HttpResponse};
use futures::future::{err, Either};
use futures::{Future, Stream};
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

pub fn save_file(field: Field) -> impl Future<Item = (String, u64), Error = Error> {
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
    let file_path_string = format!("{}.{}", Uuid::new_v4(), extension);
    let file = match fs::File::create(file_path_string.clone()) {
        Ok(file) => file,
        Err(e) => return Either::A(err(error::ErrorInternalServerError(e))),
    };
    Either::B(
        field
            .fold(
                (file, file_path_string.clone(), 0u64),
                move |(mut file, filename, mut acc), bytes| {
                    web::block(move || {
                        file.write_all(bytes.as_ref())
                            .map_err(|e| MultipartError::Payload(error::PayloadError::Io(e)))?;
                        acc += bytes.len() as u64;
                        Ok((file, filename, acc))
                    })
                    .map_err(
                        |e: error::BlockingError<MultipartError>| match e {
                            error::BlockingError::Error(e) => e,
                            error::BlockingError::Canceled => MultipartError::Incomplete,
                        },
                    )
                },
            )
            .map(|(_, filename, acc)| (filename, acc))
            .from_err(),
    )
}

/// upload multipart data and responce with a future stream of sizes
pub fn upload(multipart: Multipart) -> impl Future<Item = HttpResponse, Error = Error> {
    multipart
        .map(|field| save_file(field).into_stream())
        .flatten()
        .collect()
        .map(|responces| {
            HttpResponse::Ok().json(
                responces
                    .iter()
                    .map(|(filename, acc)| JsonImageResponse {
                        name: filename.to_string(),
                        checksum: *acc,
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .from_err()
}
