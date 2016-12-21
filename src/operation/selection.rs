use std::io::stdin;

use package::pkg::Package;

pub fn select_package(vec: &Vec<Package>) -> u32 {
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
