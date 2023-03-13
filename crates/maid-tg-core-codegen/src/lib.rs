use serde::de::DeserializeOwned;
use serde_yaml::from_str;

pub mod methods;
pub mod types;

pub mod items;

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
