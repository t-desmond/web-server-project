use reqwest::{multipart, Client, Url};
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;
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
}

impl CliUploader {
    pub async fn upload_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();

        let url = Url::parse(self.url.as_str())?;

        for file in &self.files {
            let file_content = fs::read(&file)?;
            let c = file.clone();
            let path = c.display().to_string();
            let resource_name = file
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            println!("Resource Name: {}, {}", resource_name, path);
    
            let part = multipart::Part::bytes(file_content).file_name(resource_name.clone());
    
            let form = reqwest::multipart::Form::new()
                .text("resourceName", resource_name)
                .part("fileupload", part);
    
            let response = client.post(url.clone()).multipart(form).send().await?;
    
            if response.status().is_success() {
                println!("file uploaded.. status: {}", response.status())
            } else {
                println!("failed to upload file. \n{}", response.status())
            }
        }

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
                println!("file sent succesfully")
            }
        }

        Ok(())
    }
}
