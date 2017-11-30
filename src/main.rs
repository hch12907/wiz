#![feature(from_utf8_error_as_bytes)]

extern crate flate2;
extern crate reqwest;
#[macro_use] extern crate serde_derive;
extern crate sha2;
extern crate tar;
extern crate toml;

mod config;
mod download;
mod error;
mod hash;
mod installation;
mod list;
mod package;
mod repository;
mod state;
mod unpack;

fn main() {
    println!("Hello world!");
}