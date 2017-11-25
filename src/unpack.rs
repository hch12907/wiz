use error::PackageError;
use std::fs::{ File, metadata };
use std::path::Path;
use tar::Archive;

fn unpack_tar(input: &Path, target: &Path) -> Result<u64, PackageError> {
    let file = File::open(input)?;
    let mut archive = Archive::new(file);
    archive.unpack(target)?;
    let target_file = metadata(target)?;
    Ok(target_file.len())
}

fn unpack_gzip() {

}