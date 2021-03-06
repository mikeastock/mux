#[macro_use]
extern crate clap;
extern crate yaml_rust;

mod error;
mod project;

use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::{YamlLoader, Yaml};

use error::Result;
use project::Project;

fn main() {
    let matches = App::new("mux")
        .arg(Arg::with_name("name")
                .help("Name of session")
                .required(true)
        )
        .get_matches();

    let session_name = Some(matches.value_of("name")).unwrap().unwrap();

    let config = match load_config(&session_name) {
        Ok(config) => config,
        Err(err) => {
            println!("{:?}", err);
            return
        }
    };

    let project = Project::new(config);

    println!("{:?}", session_name);
    println!("{:?}", project.name);

    project.launch();
}

fn load_config(session_name: &str) -> Result<Yaml> {
    let file_string = try!(load_yaml(session_name));
    let yamls = try!(YamlLoader::load_from_str(&file_string));
    Ok(yamls[0].to_owned())
}

fn load_yaml(session_name: &str) -> Result<String> {
    let mut file = try!(File::open(session_filepath(session_name)));
    let mut file_string = String::new();
    try!(file.read_to_string(&mut file_string));
    Ok(file_string)
}

fn session_filepath(name: &str) -> String {
    "/Users/mikeastock/.tmuxinator/".to_owned() + name + ".yml"
}
