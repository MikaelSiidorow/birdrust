//! Types for the API responses and types for the server implementation

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub device_information: DeviceInformation,
    pub capture: Capture,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Capture {
    #[serde(rename = "@snapshotTimestamp")]
    pub snapshot_timestamp: String,
    #[serde(rename(deserialize = "drone"))]
    pub drones: Vec<Drone>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Drone {
    pub serial_number: String,
    pub model: String,
    pub manufacturer: String,
    pub mac: String,
    pub ipv4: String,
    pub ipv6: String,
    pub firmware: String,
    pub position_y: f64,
    pub position_x: f64,
    pub altitude: f64,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInformation {
    #[serde(rename = "@deviceId")]
    pub device_id: String,
    pub listen_range: i32,
    pub device_started: String,
    pub uptime_seconds: i32,
    pub update_interval_ms: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pilot {
    pub pilot_id: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub email: String,
    #[serde(rename(deserialize = "createdDt"))]
    pub created_date: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Violation {
    pub serial_number: String,
    pub pilot: Option<Pilot>,
    pub distance_from_origin: f64,
    pub violated_date: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ViolationSummary {
    pub pilot: Option<Pilot>,
    pub times_seen: i32,
    pub closest_distance: f64,
    pub latest_date: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParsedReport {
    pub device_information: DeviceInformation,
    pub latest_capture: Capture,
    pub recent_violations: HashMap<String, ViolationSummary>,
}
