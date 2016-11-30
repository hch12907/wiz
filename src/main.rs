#[macro_use]
extern crate clap;
extern crate reqwest;
extern crate rustc_serialize;

use clap::{ App, Arg, SubCommand };

#[macro_use]
mod macros;
mod backend;

fn main() {
    let possible_operation = [ "install", "remove", "update", "upgrade" ];
    let arg_matches = App::new("wiz")
        .version("0.1.0")
                          
        .subcommand(SubCommand::with_name("operation")
            .about("Specify what to do with the packages")
            .arg(Arg::with_name("package")
                .help("The specified package")
                .index(1)
                .required(true)))
        
        .get_matches();

    /*let method = value_t!(arg_matches.value_of("method"), Method).unwrap();
    let package = arg_matches.value_of("package").unwrap_or("invalid");

    match method {
        Method::Install => { 
            println!("Installing package: {}", package);
            match package::installation::install_package(package, Path::new("wiz")) {
                Ok(_) => println!("Install complete"),
                Err(why) => println!("Install failed.\nReason: {}", why)
            }
        },
        Method::Remove => { 
            println!("Uninstalling package: {}", package);
            match package::uninstallation::uninstall(package, Path::new("wiz")) {
                Ok(_) => println!("Uninstall complete"),
                Err(why) => println!("Uninstall failed.\nReason: {}", why)
            }
        }
    }*/
}
