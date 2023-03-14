use std::{
    fs,
    path::Path,
};

use serde::de::DeserializeOwned;
use serde_yaml::from_str;

pub mod methods;
pub mod types;

pub mod items;

pub fn write(to: impl AsRef<Path>, content: impl AsRef<[u8]>) {
    fs::write(to, content).unwrap();
}

pub fn read(from: impl AsRef<Path>) -> String {
    fs::read_to_string(from).unwrap()
}

#[track_caller]
pub fn parse_yaml<T: DeserializeOwned>(
    content: impl AsRef<str>,
) -> T {
    match from_str(content.as_ref()) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e}");
            panic!()
        }
    }
}
