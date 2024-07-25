use std::fs::File;
use std::io::{BufRead, BufReader, stdin, Write};
use std::path::Path;

use anyhow::Result;
use log::info;

pub mod message;
pub mod packet;

pub fn load_templates(templates: Option<&Path>) -> Result<String> {
    return match templates {
        None => {
            info!("Using default templates...");
            Ok(include_str!("../templates.xml").to_string())
        }
        Some(path) => {
            info!("Reading templates from {} ...", path.display());
            Ok(String::from_utf8(std::fs::read(path)?)?)
        }
    };
}

pub fn get_data_reader(input: Option<&Path>) -> Result<Box<dyn BufRead>> {
    match input {
        None => {
            info!("Reading data from stdin...");
            Ok(Box::new(BufReader::new(stdin())))
        }
        Some(path) => {
            info!("Reading data from {} ...", path.display());
            Ok(Box::new(BufReader::new(File::open(path)?)))
        }
    }
}

pub fn get_data_writer(output: Option<&Path>) -> Result<Box<dyn Write>> {
    match output {
        None => {
            info!("Writing data to stdout...");
            Ok(Box::new(std::io::stdout()))
        }
        Some(path) => {
            info!("Writing data to {} ...", path.display());
            Ok(Box::new(std::fs::File::create(path)?))
        }
    }
}
