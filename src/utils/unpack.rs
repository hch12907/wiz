use error::PackageError;
use flate2::bufread::GzDecoder;
use std::fs::{ File, metadata };
use std::io::{ BufReader, BufWriter, Read, Write };
use std::path::Path;
use tar::Archive;

fn unpack_tar<P: AsRef<Path>>(input: P, target: P) -> Result<u64, PackageError> {
    let file = File::open(input)?;
    let mut archive = Archive::new(file);
    archive.unpack(target.as_ref())?;
    let target_file = metadata(target)?;
    Ok(target_file.len())
}

fn unpack_gzip<P: AsRef<Path>>(input: P, target: P) -> Result<u64, PackageError> {
    let file = File::open(input)?;
    let decoder = GzDecoder::new(BufReader::new(file))?;
    let target = File::create(target)?;
    let mut target = BufWriter::new(target);
    let mut bytes = 0;
    for byte in decoder.bytes() { 
        target.write(&[byte?])?;
        bytes += 1;
    }
    Ok(bytes)
}