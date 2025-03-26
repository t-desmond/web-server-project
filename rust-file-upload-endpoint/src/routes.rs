use axum::{extract::Multipart, response::Html};
use rust_file_upload_endpoint::compress_flate2;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};
use tera::{Context, Tera};

pub async fn index() -> Html<String> {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error initializing Tera: {}", e);
            return Html("Error loading templates".to_string());
        }
    };

    let context = Context::new();
    match tera.render("index.html", &context) {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            eprintln!("Error rendering template: {}", e);
            Html("Error rendering template".to_string())
        }
    }
}

pub async fn upload(mut multipart: Multipart) {
    while let Some(field) = match multipart.next_field().await {
        Err(_) => None,
        Ok(value) => value,
    } {
        if field.name().unwrap_or_default() != "fileupload" {
            continue;
        }

        let file_name = match field.file_name() {
            Some(name) => name.to_string(), 
            None => {
                eprintln!("failed to retrieve file name:");
                return;
            }
        };

        println!("Got file {}", file_name);

        let create_dir = |path: &str| {
            if fs::create_dir_all(path).is_err() {
                eprintln!("Failed to create directory {}", path);
            }
        };

        create_dir("files");
        create_dir("compressed");

        let file_path = format!("files/{}", file_name);

        let data = match field.bytes().await {
            Ok(data) => data,
            Err(e) => {
                eprintln!("failed to read bytes: {}", e);
                return;
            }
        };

        let create_and_write_file = |file_path: &str, data: &[u8]| -> Result<(), std::io::Error> {
            let mut file_handle = File::create(file_path)?;
            file_handle.write_all(data)?;
            Ok(())
        };

        if let Err(e) = create_and_write_file(&file_path, &data) {
            eprintln!("failed to write to destination file: {}", e);
            return;
        }

        if let Ok(compressed_bytes) = compress_flate2(Path::new(&file_path), None) {
            let compressed_file_path = format!("compressed/{}.gz", file_name);
            if let Err(e) = create_and_write_file(&compressed_file_path, &compressed_bytes) {
                eprintln!("failed to write compressed bytes: {}", e);
                return;
            }
        } else {
            println!("No compression applied for {}", file_name);
        }

    }
}
