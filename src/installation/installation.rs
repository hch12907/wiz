use error::PackageError;

use installation::Essentials;
pub fn install_package(name: &str, forced: bool, essential: Essentials) -> Result<(), PackageError> {
    println!("name: {}, forced: {}, config: {:?}", name, forced, essential.config);
    Ok(())
}