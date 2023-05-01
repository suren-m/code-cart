use std::{fs, path::PathBuf};

use crate::models::counties::County;
use axum::{extract::Path, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

pub async fn all() -> (StatusCode, Json<Vec<Location>>) {
    (StatusCode::OK, Json(load_json().unwrap()))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub name: String,
    pub abbreviation: String,
    pub country: String,
}

fn load_json() -> Result<Vec<Location>, serde_json::Error> {
    // let data = get_data();
    // let mut deserializer = Deserializer::from_str(&data);
    // let counties: Vec<County> = Deserialize::deserialize(&mut deserializer)?;
    // Ok(counties)
    let contents = fs::read_to_string("./data/UKCounties.json").expect("file not found");

    let locations: Vec<Location> = serde_json::from_str(&contents).expect("Failed to parse JSON");

    println!("{:#?}", locations);
    Ok(locations)
}

fn get_data() -> String {
    let cwd = std::env::current_dir().unwrap();
    let filename = PathBuf::from("data/UKCounties.json");
    let path = cwd.join(filename);
    println!("{:?}", path);

    let data = std::fs::read_to_string(path).unwrap();
    println!("retrieved data");
    data
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let d = load_json();
        if d.is_err() {
            println!("{}", d.err().unwrap());
        } else {
            println!("deserialization successful");
        }
    }
}
