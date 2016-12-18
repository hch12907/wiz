use std::error::Error;
use std::fs;
use std::io::stdin;
use std::path::Path;

use backend::download;
use package::find;
use package::pkg::Package;
use paths::{ self, PathAppend };  

fn select_package(vec: &Vec<Package>) -> u32 {
    println!("There are multiple packages with similar name");

    for i in 0..vec.len() {
        println!("#{}: {}", i, &vec[i].name);
    }
    
    let mut choice = String::new();
    stdin().read_line(&mut choice).ok().expect("Invalid input");
    let choice = choice.trim().parse::<u32>().unwrap();

    return if choice as usize > vec.len() {
        choice
    } else {
        println!("Invalid input");
        select_package(vec)
    }
}

fn install_package(package: &Package, base_path: &Path) -> Result<(), String> {
    try!(download::download_file_while(url, base_path, |all, current|{
        println!("Downloaded {} bytes, out of {} bytes.", current, all);
    });
}

pub fn install(input: &str, base_path: &Path) -> Result<(), String> {
    let package_list_dir = get!(fs::read_dir(base_path.to_path_buf().append(paths::PACKAGE_LIST_DIR)), "");
    for list_path in package_list_dir {
        let result = find::select_package(input, list_path.unwrap().path().as_path(), select_package); 
        if result.is_ok() {
            let result = result.unwrap();
            return install_package(&result, base_path);
        }
    }
    Err("Specified package not found".to_string())
}