use axum::{extract::Multipart, response::Html};
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
    while let Some(field) = multipart
        .next_field()
        .await
        .expect("failed to extract field")
    {
        if field.name().unwrap() != "fileupload" {
            continue;
        }

        let file_name = field.file_name().unwrap();

        println!("Got file {}", file_name);

        if fs::create_dir_all("files").is_ok() {
        } else {
            println!("Failed to create destination directory");
        }

        let file_path = format!("files/{}", file_name);

        let data = field.bytes().await.unwrap();

        let mut file_handle = File::create(file_path).expect("failed to open file handle");

        file_handle.write_all(&data).expect("failed to write data");
    }
}
