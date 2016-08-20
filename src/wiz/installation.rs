use std::fs::File;

fn find_package(name: &str) -> String
{
    
}

fn retrieve_package(url: &str) -> Path
{
    /*
      This function retrieves the package, and
      puts them in a temporary folder.
    */
}

fn verify_package(package: &File, provided: &str)
{
    /*
      This function verifies the package(CRC32 IEEE)
      retrieved by retrieve_package, and returns boolean indicating 
      that the file is not corrupted.
    */
}

fn copy_package()
{
    /*
      This function retrieves the package, and put them
      to specified locations accordingly.
    */
}

fn process_package()
{
    /*
      This function finalises the installation of
      the package, such as setting PATH and such.
    */
}

fn install()
{
    
}