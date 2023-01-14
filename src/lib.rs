//! # Birdrust - birdnest server
//! Fetches and stores 10 minutes of drone data, pilots and recent violations from the Birdnest API
//! Serves it to all websocket clients

#![warn(missing_docs)]

pub mod api;
pub mod logic;
pub mod types;
