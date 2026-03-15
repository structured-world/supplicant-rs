//! Error types for supplicant-rs.

/// Errors that can occur during supplicant operations.
#[derive(Debug, thiserror::Error)]
pub enum SupplicantError {
    /// Netlink communication error.
    #[error("nl80211 error: {0}")]
    Netlink(String),

    /// Authentication failed.
    #[error("Authentication failed: {0}")]
    AuthFailed(String),

    /// Handshake timeout.
    #[error("Handshake timeout after {0:?}")]
    Timeout(std::time::Duration),

    /// Cryptographic error.
    #[error("Crypto error: {0}")]
    Crypto(String),

    /// EAP method error.
    #[error("EAP error: {0}")]
    Eap(String),

    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Interface not found.
    #[error("Interface not found: {0}")]
    InterfaceNotFound(String),
}
