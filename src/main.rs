#[macro_use]
extern crate clap;
use clap::{App, Arg};

arg_enum! {
    #[derive(Debug)]
    enum Method {
        Install,
        Remove
    }
}

fn main() {
    let arg_matches = App::new("wiz")
        .version("0.1.0")
                          
        .arg(Arg::with_name("method")
             .help("Specify what to do with the specified package")
             .index(1)
             .possible_values(&Method::variants())
             .required(true))
                                        
        .arg(Arg::with_name("package")
             .help("Specify package")
             .index(2)
             .required(true))
        
        .get_matches();

    let method = value_t!(arg_matches.value_of("method"), Method).unwrap();
    let package = arg_matches.value_of("package").unwrap_or("invalid");

    match method {
        Method::Install => println!("install: {}", package),
        Method::Remove => println!("remove: {}", package)
    }
}
