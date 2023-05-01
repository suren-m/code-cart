use std::{fs, path::PathBuf};

use crate::models::counties::County;
use axum::{http::StatusCode, Json};

pub async fn all() -> (StatusCode, Json<Vec<County>>) {
    let data = load_json();
    if data.is_err() {
        tracing::error!("{} failed to parse json", data.err().unwrap());
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]));
    }
    (StatusCode::OK, Json(data.unwrap()))
}

fn load_json() -> Result<Vec<County>, serde_json::Error> {
    let contents = fs::read_to_string(get_path()).expect("file not found");

    serde_json::from_str(&contents)

    //println!("{:#?}", counties);
    //Ok(counties)
}

fn get_path() -> PathBuf {
    let cwd = std::env::current_dir().unwrap();
    let filename = PathBuf::from("data/UKCounties.json");
    let path = cwd.join(filename);
    tracing::info!("{:?}", path);
    path
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
