use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct County {
    #[serde(rename = "abbreviation")]
    pub code: String,
    pub name: String,
    pub country: Country,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Country {
    England,
    Scotland,
    Wales,
    #[serde(rename = "Northern Ireland")]
    NorthernIreland,
}
