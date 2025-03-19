mod compression;
use std::error::Error;

use compression::CompressFile;
fn main()  -> Result<(), Box<dyn Error>> {
    let file = CompressFile::new("n.txt");

    file.compress()?;
    
    Ok(())
}
