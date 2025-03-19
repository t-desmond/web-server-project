use std::error::Error;

use cli::CliUploader;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let cli = CliUploader::from_args();
    
    cli.upload_file().await?;

    Ok(())
}

mod cli;