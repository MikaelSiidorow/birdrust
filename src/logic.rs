//! Logic for the server, and updating data

use crate::api::{fetch_pilot, fetch_report};
use crate::types::{Drone, ParsedReport, Pilot, Report, Violation, ViolationSummary};
use futures::future::join_all;
use std::collections::HashMap;

/// Fetches the report from the API, and parses it into a ParsedReport
///
/// # Arguments
///
/// * `previous_parsed_report` - The previous ParsedReport, if there is one. Used as initial state for the new ParsedReport
///
/// # Examples
///
/// ```
/// use birdnest_server::logic::update_report;
/// async {
///     let new_parsed_report = update_report(&None).await;
///     // Do something with the new report
///     // ...
/// };
/// ```
pub async fn update_report(previous_parsed_report: &Option<ParsedReport>) -> ParsedReport {
    let new_report: Report = fetch_report().await;
    let new_violating_drones = &new_report
        .capture
        .drones
        .iter()
        .map(|drone| (drone.serial_number.to_string(), distance_from_origin(drone)))
        .filter(|(_, distance_from_origin)| distance_from_origin < &100000.0)
        .collect::<Vec<(String, f64)>>();

    let new_violations = join_all(new_violating_drones.iter().map(
        |(serial_number, distance_from_origin)| async {
            let pilot: Option<Pilot> = fetch_pilot(serial_number).await;

            Violation {
                serial_number: serial_number.to_string(),
                pilot,
                distance_from_origin: *distance_from_origin,
                violated_date: new_report.capture.snapshot_timestamp.clone(),
            }
        },
    ))
    .await;

    let previous_violations = if let Some(previous_state) = previous_parsed_report {
        previous_state.recent_violations.clone()
    } else {
        HashMap::new()
    };

    let recent_violations =
        new_violations
            .iter()
            .fold(previous_violations, |mut violations, violation| {
                let summary = violations
                    .entry(violation.serial_number.to_string())
                    .or_insert(ViolationSummary {
                        pilot: violation.pilot.clone(),
                        times_seen: 0,
                        closest_distance: -1.0,
                        latest_date: "".to_string(),
                    });
                summary.times_seen += 1;
                summary.closest_distance = if summary.closest_distance == -1.0 {
                    violation.distance_from_origin
                } else {
                    summary.closest_distance.min(violation.distance_from_origin)
                };
                summary.latest_date = violation.violated_date.clone();
                violations
            });

    ParsedReport {
        device_information: new_report.device_information,
        latest_capture: new_report.capture,
        recent_violations,
    }
}

/// Calculate drone distance from birdnest located at (250000, 250000)
///
/// # Arguments
///
/// * `drone` - The drone to calculate the distance from origin
///
/// # Examples
///
/// ```
/// use birdnest_server::{logic::distance_from_origin, types::Drone};
/// let drone = Drone {
///     serial_number: "SN-wVJnFAznMo".to_string(),
///     model: "HRP-DRP 1 Pro".to_string(),
///     manufacturer: "ProDröne Ltd".to_string(),
///     mac: "4f:a7:2a:72:fe:01".to_string(),
///     ipv4: "115.226.53.29".to_string(),
///     ipv6: "e3b6:a70d:0d56:96eb:7924:56ca:ddc1:8d41".to_string(),
///     firmware: "6.3.3".to_string(),
///     position_y: 250000.0,
///     position_x: 250000.0,
///     altitude: 4718.8464039730825,
/// };
/// assert_eq!(distance_from_origin(&drone), 0.0);
/// ```
///
/// ```
/// use birdnest_server::{logic::distance_from_origin, types::Drone};
/// let drone = Drone {
///     serial_number: "SN-wVJnFAznMo".to_string(),
///     model: "HRP-DRP 1 Pro".to_string(),
///     manufacturer: "ProDröne Ltd".to_string(),
///     mac: "4f:a7:2a:72:fe:01".to_string(),
///     ipv4: "115.226.53.29".to_string(),
///     ipv6: "e3b6:a70d:0d56:96eb:7924:56ca:ddc1:8d41".to_string(),
///     firmware: "6.3.3".to_string(),
///     position_y: 436597.82481145335,
///     position_x: 141326.63552825956,
///     altitude: 4718.8464039730825,
/// };
/// assert_eq!(distance_from_origin(&drone), 215936.67675958519);
/// ```
pub fn distance_from_origin(drone: &Drone) -> f64 {
    let origin_x = 250000.0;
    let origin_y = 250000.0;
    let x = drone.position_x;
    let y = drone.position_y;
    ((x - origin_x).powi(2) + (y - origin_y).powi(2)).sqrt()
}
