//! Helper functions for fetching data from the API

use crate::types::{Pilot, Report};

/// Fetches the most recent report from the and parses the XML into a Report struct
pub async fn fetch_report() -> Report {
    let res = reqwest::get("https://assignments.reaktor.com/birdnest/drones")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    quick_xml::de::from_str(&res).unwrap()
}

/// Fetches a pilot from the API and parses the JSON into a Pilot struct, if the pilot is found
///
/// # Arguments
///
/// * `serial_number` - The serial number of the drone
///
pub async fn fetch_pilot(serial_number: &str) -> Option<Pilot> {
    let response = reqwest::get(format!(
        "https://assignments.reaktor.com/birdnest/pilots/{}",
        serial_number
    ))
    .await;

    match response {
        Ok(response) => Some(response.json::<Pilot>().await.unwrap()),
        Err(_) => None,
    }
}
