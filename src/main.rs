#![feature(from_utf8_error_as_bytes)]

extern crate clap;
extern crate flate2;
extern crate reqwest;
#[macro_use] extern crate serde_derive;
extern crate sha2;
extern crate tar;
extern crate toml;

mod app;
mod cache;
mod config;
mod error;
mod installation;
mod package;
mod repository;
mod utils;

use std::path::Path;

fn main() {
    // Get the configs.
    let config = Path::new("~/.config/wiz");
    let config = config::Config::
        read_from(config)
        .map(|x| x.fill_with_default())
        .unwrap();

    // Get the packages which are installed or are going to be installed.
    let cache = Path::new("~/.config/wiz");
    let cache = cache::Cache::
        read_from(cache)
        .unwrap();

    // Get the repositories (package lists).
    let repositories = Path::new("~/.config/wiz");
    let repositories = repository::RepositoryList::
        read_from(repositories)
        .unwrap();

    println!("{}", app::run()
        .subcommand_matches("install")
        .map_or("no subcommands specified", |s|
            s.value_of("package_name")
                .unwrap_or("no arguments specified")
        )    
    );
}