use cli::CliUploader;
use structopt::StructOpt;

fn main() {
    let cli = CliUploader::from_args();
    println!("{:#?}!", cli);
}

mod cli;