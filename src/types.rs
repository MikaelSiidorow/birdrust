//! Types for the API responses and types for the server implementation

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Report {
    #[serde(rename = "deviceInformation")]
    pub device_information: DeviceInformation,
    pub capture: Capture,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Capture {
    #[serde(rename = "snapshotTimestamp")]
    pub snapshot_timestamp: String,
    #[serde(rename(deserialize = "drone"))]
    pub drones: Vec<Drone>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Drone {
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    pub model: String,
    pub manufacturer: String,
    pub mac: String,
    pub ipv4: String,
    pub ipv6: String,
    pub firmware: String,
    #[serde(rename = "positionY")]
    pub position_y: f64,
    #[serde(rename = "positionX")]
    pub position_x: f64,
    pub altitude: f64,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DeviceInformation {
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "listenRange")]
    pub listen_range: i32,
    #[serde(rename = "deviceStarted")]
    pub device_started: String,
    #[serde(rename = "uptimeSeconds")]
    pub uptime_seconds: i32,
    #[serde(rename = "updateIntervalMs")]
    pub update_interval_ms: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Pilot {
    #[serde(rename = "pilotId")]
    pub pilot_id: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    pub email: String,
    #[serde(rename = "createdDt")]
    pub created_date: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Violation {
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    pub pilot: Option<Pilot>,
    #[serde(rename = "distanceFromOrigin")]
    pub distance_from_origin: f64,
    #[serde(rename = "violatedDt")]
    pub violated_date: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ViolationSummary {
    pub pilot: Option<Pilot>,
    #[serde(rename = "timesSeen")]
    pub times_seen: i32,
    #[serde(rename = "closestDistance")]
    pub closest_distance: f64,
    #[serde(rename = "latestDt")]
    pub latest_date: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ParsedReport {
    #[serde(rename = "deviceInformation")]
    pub device_information: DeviceInformation,
    #[serde(rename = "latestCapture")]
    pub latest_capture: Capture,
    #[serde(rename = "recentViolations")]
    pub recent_violations: HashMap<String, ViolationSummary>,
}
