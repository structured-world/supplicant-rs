//! # supplicant-rs — Pure Rust WPA Supplicant
//!
//! Memory-safe replacement for wpa_supplicant. WPA2/WPA3, EAP, 802.1X.
//!
//! ## Protocols
//!
//! - **WPA2** — 4-way handshake, PMKSA caching
//! - **WPA3-SAE** — Simultaneous Authentication of Equals (dragonfly)
//! - **EAP** — EAP-TLS, EAP-TTLS, EAP-PEAP
//! - **802.1X** — EAPOL state machine
//! - **PMF** — Protected Management Frames
//!
//! ## Architecture
//!
//! Uses Linux nl80211 netlink interface for kernel communication.
//! Runs as unprivileged daemon with CAP_NET_ADMIN capability.

#![deny(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![warn(missing_docs)]

pub mod error;

#[cfg(feature = "wpa2")]
pub mod wpa2;

#[cfg(feature = "wpa3")]
pub mod wpa3;

pub mod eapol;
pub mod nl80211;

pub use error::SupplicantError;
