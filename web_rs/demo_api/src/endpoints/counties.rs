use crate::models::counties::County;
use axum::{http::StatusCode, Json};
use std::fs;
pub async fn all() -> (StatusCode, Json<Vec<County>>) {
    (StatusCode::OK, Json(load_data().unwrap()))
}

fn load_data() -> Result<Vec<County>, serde_json::Error> {
    let data = fs::read_to_string("./data/UkCounties.json").unwrap();
    println!("{}", data);
    let res = serde_json::from_str::<Vec<County>>(&data);
    if let Err(e) = res {
        println!("{}", e);
        Err(e)
    } else {
        Ok(res.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let d = load_data();
        //println!("{:?}", d);
    }
}
