use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct County {
    // #[serde(rename = "abbreviation")]
    pub abbreviation: String,
    pub name: String,
    pub country: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Country {
    England,
    Scotland,
    Wales,
    NorthernIreland,
}
