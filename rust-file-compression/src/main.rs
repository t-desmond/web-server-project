mod compression;
use std::{error::Error, path::Path};

use compression::CompressFile;
fn main()  -> Result<(), Box<dyn Error>> {
    let file = CompressFile::new(Path::new("/Users/gis/Projects/coding/rust/class-exrcises/web-dev/web-server-project/rust-file-compression/n.txt"));

    file.compress()?;
    
    Ok(())
}
