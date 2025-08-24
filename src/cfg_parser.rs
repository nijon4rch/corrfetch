use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::process::exit;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub keys: String,
    pub format: HashMap<String, String>,
    pub separator: Option<char>,
    pub logo: Option<Logo>,
}

#[derive(Debug, Deserialize)]
pub struct Logo {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub charset: Option<Vec<String>>,
}

pub fn read_config(config_file: std::path::PathBuf) -> Config {
    let contents = match fs::read_to_string(&config_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not read file `{config_file:?}`: {e}");
            exit(1);
        }
    };

    match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Could not parse file `{config_file:?}`: {e}");
            exit(1);
        }
    }
}
