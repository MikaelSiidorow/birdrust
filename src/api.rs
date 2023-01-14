use crate::types::{Pilot, Report};

pub async fn fetch_report() -> Report {
    let res = reqwest::get("https://assignments.reaktor.com/birdnest/drones")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    quick_xml::de::from_str(&res).unwrap()
}

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
