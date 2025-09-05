//! # Dropbox Sign Rust Client
//!
//! A Rust client library for the Dropbox Sign API (formerly HelloSign).
//!
//! This crate provides a type-safe, async interface to interact with the Dropbox Sign API
//! for sending and managing signature requests.
//!
//! ## Features
//!
//! - Type-safe API interactions with comprehensive error handling
//! - Async/await support using `reqwest`
//! - Support for signature requests with templates
//! - Proper handling of API warnings and errors
//! - Builder patterns for complex request construction
//!
//! ## Example
//!
//! ```no_run
//! use dropboxsign_rs::{DropboxSignClient, signature_request::*};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = DropboxSignClient::new("your-api-key");
//!
//!     let signer = SubSignatureRequestTemplateSigner::new(
//!         "Signer".to_string(),
//!         "John Doe".to_string(),
//!         "john@example.com".to_string()
//!     );
//!
//!     let request = SendSignatureRequest::new(
//!         vec![signer],
//!         vec!["template-id".to_string()]
//!     );
//!
//!     let (response, warnings) = client.send_with_template(request).await?;
//!     println!("Signature request sent: {}", response.signature_request_id);
//!
//!     Ok(())
//! }
//! ```

#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt;

/// HTTP client implementation for Dropbox Sign API
pub mod client;

/// Data models and types for signature request operations
pub mod signature_request;

// Re-export the main types for convenience
pub use client::DropboxSignClient;

/// Generic wrapper for API responses that may contain warnings alongside the main data.
///
/// The `inner` field contains the actual response data, while `warnings` contains
/// any non-fatal warnings returned by the API.
#[derive(Debug, Deserialize)]
pub struct ResponseWithWarnings<T> {
    /// The main response data
    #[serde(flatten)]
    pub inner: T,
    /// Optional warnings returned by the API
    #[serde(default)]
    pub warnings: Option<Vec<WarningResponse>>,
}

/// Represents a non-fatal warning returned by the Dropbox Sign API.
///
/// Warnings indicate potential issues or important information that doesn't
/// prevent the operation from completing successfully.
#[derive(Debug, Serialize, Deserialize)]
pub struct WarningResponse {
    /// Human-readable warning message
    warning_msg: String,
    /// Machine-readable warning identifier
    warning_name: String,
}

/// Top-level error response structure from the Dropbox Sign API.
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// The detailed error information
    pub error: ErrorResponseError,
}

/// Detailed error information from the Dropbox Sign API.
///
/// Contains structured error details including HTTP status codes,
/// error messages, and optional path information for field-specific errors.
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponseError {
    /// HTTP status code (not serialized, set by client)
    #[serde(skip)]
    pub status: StatusCode,
    /// Human-readable error message
    pub error_msg: String,
    /// Optional path to the field that caused the error
    pub error_path: Option<String>,
    /// Machine-readable error identifier
    pub error_name: String,
}

impl fmt::Display for WarningResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.warning_msg, self.warning_name)
    }
}

impl fmt::Display for ErrorResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(path) = &self.error_path {
            write!(f, "{} ({}): {}", self.error_name, path, self.error_msg)
        } else {
            write!(f, "{}: {}", self.error_name, self.error_msg)
        }
    }
}
