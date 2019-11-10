use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{error, web, Error, HttpResponse};
use futures::future::{err, Either};
use futures::{Future, Stream};
use std::cell::Cell;
use std::fs;
use std::io::Write;

pub fn save_file(field: Field) -> impl Future<Item = i64, Error = Error> {
    let file_path_string = match field
        .content_disposition()
        .and_then(|content| content.get_filename().map(|s| s.to_string()))
    {
        Some(filename) => filename.replace(' ', "_"),
        None => {
            return Either::A(err(error::ErrorInternalServerError(
                "Couldn't read the filename.",
            )))
        }
    };
    let file = match fs::File::create(file_path_string) {
        Ok(file) => file,
        Err(e) => return Either::A(err(error::ErrorInternalServerError(e))),
    };
    Either::B(
        field
            .fold((file, 0i64), move |(mut file, mut acc), bytes| {
                // fs operations are blocking, we have to execute writes
                // on threadpool
                web::block(move || {
                    file.write_all(bytes.as_ref()).map_err(|e| {
                        println!("file.write_all failed: {:?}", e);
                        MultipartError::Payload(error::PayloadError::Io(e))
                    })?;
                    acc += bytes.len() as i64;
                    Ok((file, acc))
                })
                .map_err(|e: error::BlockingError<MultipartError>| match e {
                    error::BlockingError::Error(e) => e,
                    error::BlockingError::Canceled => MultipartError::Incomplete,
                })
            })
            .map(|(_, acc)| acc)
            .from_err(),
    )
}

/// upload multipart data and responce with a future stream of sizes
pub fn upload(
    multipart: Multipart,
    counter: web::Data<Cell<usize>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    counter.set(counter.get() + 1);
    println!("{:?}", counter.get());

    multipart
        .map(|field| save_file(field).into_stream())
        .flatten()
        .collect()
        .map(|sizes| HttpResponse::Ok().json(sizes))
        .from_err()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_index() {}
}
