use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rust-cli-file-uploader", about = "Use an HTTP client (such as reqwest) to send the file to the upload endpoint on the server in this same workspace")]
pub struct CliUploader {
  #[structopt(short = "-f", long, parse(from_os_str))]
  pub file: PathBuf
}