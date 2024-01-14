use reqwest::{Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geolocation {
    pub status: String,
    pub country: String,
    pub country_code: String,
    pub region: String,
    pub region_name: String,
    pub city: String,
    pub lat: f64,
    pub lon: f64,
    pub timezone: String,
    pub isp: String,
    pub org: String,
}

#[tokio::main]
async fn main() {

    // replace the URL with any ip-geolocation vendor url, the one used currently provides free service
    let ip = "<IP_ADDRESS>";
    let uri = format!("http://ip-api.com/json/{}", &ip);
    let result: Geolocation  = Client::new()
        .get(uri)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    
    println!("{:#?}", result);
}

