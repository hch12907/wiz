#![feature(from_utf8_error_as_bytes)]

extern crate flate2;
extern crate reqwest;
#[macro_use] extern crate serde_derive;
extern crate sha2;
extern crate tar;
extern crate toml;

mod config;
mod error;
mod installation;
mod package;
mod repository;
mod state;
mod utils;

fn main() {
    println!("Hello world!");
}