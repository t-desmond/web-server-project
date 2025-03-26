use chrono::Local;
use pbr::ProgressBar;
use reqwest::{multipart, Client, Url};
use rust_cli_file_uploader::compress_flate2;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
#[structopt(
    name = "rust-cli-file-uploader",
    about = "Use an HTTP client to send the file to the upload endpoint on the server in this same workspace"
)]
pub struct CliUploader {
    #[structopt(short = "-f", long, parse(from_os_str))]
    pub files: Vec<PathBuf>,
    #[structopt(short = "-u", long)]
    pub url: Url,
    #[structopt(short = "-m", long)]
    pub method: Option<String>,
    #[structopt(short = "-l", long)]
    pub level: Option<u32>
}

impl CliUploader {
    pub async fn upload_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();

        let url = Url::parse(self.url.as_str())?;

        let mut progress = ProgressBar::new(self.files.len() as u64);
        for file in &self.files {
            thread::sleep(Duration::from_secs(2));

            let compressed_data = if &self.method.clone().unwrap_or("flate2".to_string()) == "flate2" {
                compress_flate2(&file, self.level)?
            } else {
                println!("{:?} not implemented, going with default compression method", self.method.clone().unwrap());
                compress_flate2(&file, None)?
            };
            
            let resource_name = format!(
                "{}_{}",
                Local::now().format("%Y-%m-%d-%T").to_string(),
                file.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
            );
            
            let part = multipart::Part::bytes(compressed_data).file_name(resource_name.clone());
            
            let form = reqwest::multipart::Form::new()
            .text("resourceName", resource_name.clone())
            .part("fileupload", part);
        
        let response = client.post(url.clone()).multipart(form).send().await?;
        progress.format("╢▌▌░╟");
        progress.inc();

            if response.status().is_client_error() {
                println!("failed to upload file: {}", resource_name);
            }
        }
        progress.finish_print("done");

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn upload_file_1(&self) -> Result<(), Box<dyn std::error::Error>> {
        for file_ in &self.files {
            let mut file = File::open(&file_)?;

            let mut file_data = Vec::new();

            file.read_to_end(&mut file_data)?;

            let file_name = file_.file_name().unwrap().to_string_lossy().to_string();

            let form = multipart::Form::new().part(
                "fileupload",
                multipart::Part::bytes(file_data).file_name(file_name),
            );

            let response = Client::new()
                .post(self.url.as_str())
                .multipart(form)
                .send()
                .await?;

            if response.status().is_success() {
                println!("file uploaded.. status: {}", response.status())
            } else {
                println!("failed to upload file. \n{}", response.status())
            }
        }

        Ok(())
    }
}
