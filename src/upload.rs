use actix_multipart::Multipart;
use actix_web::post;
use actix_web::web;
use actix_web::web::Bytes;
use actix_web::web::BytesMut;
use actix_web::HttpResponse;
use futures_util::stream::StreamExt;
use futures_util::TryStreamExt;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::TARGET_FOLDER;

#[post("/upload")]
pub async fn upload_file(
    query: web::Query<std::collections::HashMap<String, String>>,
    mut payload: Multipart,
) -> HttpResponse {
    let file_path_str = match query.get("path") {
        Some(path) => path,
        None => return HttpResponse::NotFound().body("no file path found"),
    };

    let target_path = Path::new(TARGET_FOLDER).join(file_path_str);

    if let Some(parent) = target_path.parent() {
        if !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to create directories: {}", e));
            }
        }
    }

    let mut file_content = BytesMut::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        // Get the field name (in case there are multiple fields, for now just handle one)

        // Read the field data (file content)
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            file_content.extend_from_slice(&data); // Now works because file_content is BytesMut
        }

        // Now save the file content to the specified path
        if let Err(e) = crate::fs_helper::save_file(&target_path, file_content).await {
            return HttpResponse::InternalServerError().body(format!("Failed to save file: {}", e));
        }

        let filename = field
            .content_disposition()
            .unwrap() // improve error handling here
            .get_filename()
            .unwrap_or("unknown");

        return HttpResponse::Ok().body(format!("File '{}' uploaded successfully", filename));
    }

    HttpResponse::BadRequest().body("No file uploaded.")
}
