use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct CliUploader {
  #[structopt(short, long, parse(from_os_str))]
  pub file: PathBuf
}