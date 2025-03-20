use axum::{
    extract::{multipart::Field, Multipart},
    response::Html,
};
use std::{
    fs::{self, File},
    io::Write,
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
            Some(f) => f,
            None => {
                eprintln!("failed to retrieve file name:");
                return;
            }
        };

        println!("Got file {}", file_name);

        if fs::create_dir_all("files").is_ok() {
        } else {
            println!("Failed to create destination directory");
        }

        let file_path = format!("files/{}", file_name);

        let data = match field.bytes().await {
            Ok(data) => data,
            Err(_) => {
                eprintln!("failed to read bytes");
                return;
            }
        };

        let mut file_handle = match File::create(file_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("failed to create file {}", e);
                return;
            }
        };

        match file_handle.write_all(&data) {
            Ok(_) => {}
            Err(_) => {
                println!("failed to wirte to destination file")
            }
        }
    }
}
