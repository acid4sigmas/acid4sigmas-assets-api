use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

mod fs_helper;
mod upload;

use crate::upload::upload_file;

const TARGET_FOLDER: &str = "/";

#[derive(Serialize)]
struct FolderContent {
    folder_name: String,
    path: String,
    folder_content: Vec<FolderItem>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum FolderItem {
    File {
        name: String,
        path: String,
        folder: bool,
    },
    Folder {
        name: String,
        path: String,
        folder: bool,
        folder_content: Vec<FolderItem>,
    },
}

async fn list_directory(path: web::Path<String>) -> HttpResponse {
    let base_path = Path::new(TARGET_FOLDER).join(&*path);

    if !base_path.exists() {
        return HttpResponse::NotFound().finish();
    }

    let mut folder_items = vec![];

    if let Ok(entries) = fs::read_dir(&base_path) {
        for entry in entries.filter_map(Result::ok) {
            let entry_name = entry.file_name().into_string().unwrap();
            let entry_path = entry.path();
            let relative_path = entry_path
                .strip_prefix(TARGET_FOLDER)
                .unwrap()
                .display()
                .to_string();

            if entry_path.is_dir() {
                let folder_content = list_directory_entries(&entry_path);
                folder_items.push(FolderItem::Folder {
                    name: entry_name,
                    path: relative_path,
                    folder: true,
                    folder_content,
                });
            } else {
                folder_items.push(FolderItem::File {
                    name: entry_name,
                    path: relative_path,
                    folder: false,
                });
            }
        }
    }

    HttpResponse::Ok().json(FolderContent {
        folder_name: path.to_string(),
        path: path.clone(),
        folder_content: folder_items,
    })
}

fn list_directory_entries(path: &Path) -> Vec<FolderItem> {
    let mut items = vec![];

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(Result::ok) {
            let entry_name = entry.file_name().into_string().unwrap();
            let entry_path = entry.path();
            let relative_path = entry_path
                .strip_prefix(TARGET_FOLDER)
                .unwrap()
                .display()
                .to_string();

            if entry_path.is_dir() {
                let folder_content = list_directory_entries(&entry_path);
                items.push(FolderItem::Folder {
                    name: entry_name,
                    path: relative_path,
                    folder: true,
                    folder_content,
                });
            } else {
                items.push(FolderItem::File {
                    name: entry_name,
                    path: relative_path,
                    folder: false,
                });
            }
        }
    }

    items
}

#[get("/{path:.*}")]
async fn get_folder(path: web::Path<String>) -> HttpResponse {
    list_directory(path).await
}

#[get("/files/{path:.*}")]
async fn get_file(path: web::Path<String>) -> HttpResponse {
    let file_path = Path::new(TARGET_FOLDER).join(&*path);

    if file_path.exists() {
        return HttpResponse::Ok().body(fs::read(file_path).unwrap());
    }

    HttpResponse::NotFound().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(get_file)
            .service(get_folder)
            .service(upload_file)
    })
    .bind("127.0.0.1:8087")?
    .run()
    .await
}
