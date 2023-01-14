use crate::api::{fetch_pilot, fetch_report};
use crate::types::{Drone, ParsedReport, Pilot, Report, Violation, ViolationSummary};
use futures::future::join_all;
use std::collections::HashMap;

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

fn distance_from_origin(drone: &Drone) -> f64 {
    let origin_x = 250000.0;
    let origin_y = 250000.0;
    let x = drone.position_x;
    let y = drone.position_y;
    ((x - origin_x).powi(2) + (y - origin_y).powi(2)).sqrt()
}
