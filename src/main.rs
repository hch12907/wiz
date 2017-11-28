extern crate reqwest;
#[macro_use] extern crate serde_derive;
extern crate tar;
extern crate toml;

mod config;
mod download;
mod error;
mod hash;
mod package;
mod repository;
mod state;
mod unpack;

fn main() {
    println!("Hello world!");
}