//! `os-stub` — Handl stdlib port.

/// Checkout root of this repo (build-time).
pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

pub const ENV_HNL: &str = include_str!("env.hnl");

pub const PATH_HNL: &str = include_str!("path.hnl");

pub const TIME_HNL: &str = include_str!("time.hnl");
