#[macro_use]
extern crate clap;
extern crate yaml_rust;

mod error;

use clap::{App, Arg};
use yaml_rust::{YamlLoader, Yaml};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use error::{Error, Result};

fn main() {
    let config = match load_config() {
        Ok(config) => config,
        Err(err) => {
            println!("{:?}", err);
            return
        }
    };

    println!("{:?}", config["name"].as_str().unwrap());
}

fn load_config() -> Result<Yaml> {
    let file_string = try!(load_yaml());
    let yamls = try!(YamlLoader::load_from_str(&file_string));
    Ok(yamls[0].to_owned())
}

fn load_yaml() -> Result<String> {
    let mut file = try!(File::open("/Users/mikeastock/.tmuxinator/dev.yml"));
    let mut file_string = String::new();
    try!(file.read_to_string(&mut file_string));
    Ok(file_string)
}
