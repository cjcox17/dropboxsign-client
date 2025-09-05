//! HTTP client implementation for the Dropbox Sign API.
//!
//! This module provides the main client struct and associated functionality
//! for making authenticated requests to the Dropbox Sign API.

use crate::signature_request::{SendSignatureRequest, SignatureRequestResponse};
use crate::{ErrorResponse, ErrorResponseError, WarningResponse};
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value;
use thiserror::Error;

/// Base URL for the Dropbox Sign API (v3)
const API_URL: &str = "https://api.hellosign.com/v3";

/// Parses a JSON response from the Dropbox Sign API, extracting the main payload and any warnings.
///
/// This utility function handles the common pattern of Dropbox Sign API responses which
/// contain the main data under a specific key (e.g., "signature_request") and optional
/// warnings at the top level.
///
/// # Arguments
///
/// * `response` - The HTTP response from the API
/// * `key` - The JSON key under which the main payload is stored
///
/// # Returns
///
/// A tuple containing the deserialized payload and optional warnings, or an error
/// if the response cannot be parsed or the expected key is missing.
///
/// # Errors
///
/// Returns an error if:
/// - The response body cannot be read
/// - The JSON cannot be parsed
/// - The specified key is missing from the response
/// - The payload cannot be deserialized into type `T`
pub async fn parse_response<T: DeserializeOwned>(
    response: reqwest::Response,
    key: &str,
) -> Result<(T, Option<Vec<WarningResponse>>), Box<dyn std::error::Error + Send + Sync>> {
    let body = response.text().await?;
    let json: Value = serde_json::from_str(&body)?;

    // Extract main payload by key
    let payload = json
        .get(key)
        .ok_or_else(|| format!("Missing key `{}` in response", key))?;

    // Deserialize the payload into T
    let inner: T = serde_json::from_value(payload.clone())?;

    // Extract warnings if present
    let warnings = json
        .get("warnings")
        .map(|w| serde_json::from_value(w.clone()))
        .transpose()?;

    Ok((inner, warnings))
}

/// HTTP client for interacting with the Dropbox Sign API.
///
/// This client handles authentication, request/response processing, and error handling
/// for Dropbox Sign API operations. It supports configuration of connection pooling
/// and request timeouts.
///
/// # Examples
///
/// ```no_run
/// use dropboxsign_rs::DropboxSignClient;
///
/// let client = DropboxSignClient::new("your-api-key")
///     .with_pool(10)
///     .with_timeout(60);
/// ```
#[derive(Clone)]
pub struct DropboxSignClient {
    /// API key for authentication
    api_key: String,
    /// HTTP client for making requests
    client: Client,
    /// Connection pool size (currently unused, reserved for future use)
    pool: usize,
    /// Request timeout in seconds (currently unused, reserved for future use)
    timeout: usize,
}

/// Errors that can occur when using the Dropbox Sign client.
///
/// This enum covers all possible error conditions including HTTP errors,
/// JSON parsing errors, and API-specific errors returned by Dropbox Sign.
#[derive(Error, Debug)]
pub enum DropboxSignClientError {
    #[error("non 200 status: {0} {1}")]
    DropboxSignClient(String, String),

    #[error("HTTP request error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Json error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("DropboxSign error: {0}")]
    ResponseError(ErrorResponseError),

    #[error("Other error: {0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl DropboxSignClient {
    /// Creates a new Dropbox Sign client with the specified API key.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your Dropbox Sign API key
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dropboxsign_rs::DropboxSignClient;
    ///
    /// let client = DropboxSignClient::new("your-api-key");
    /// ```
    pub fn new(api_key: impl Into<String>) -> Self {
        let client = Client::new();
        Self {
            api_key: api_key.into(),
            client,
            pool: 5,
            timeout: 30,
        }
    }

    /// Sets the connection pool size for the client.
    ///
    /// # Arguments
    ///
    /// * `pool` - Maximum number of connections in the pool
    ///
    /// # Returns
    ///
    /// The client instance for method chaining
    ///
    /// # Note
    ///
    /// This setting is currently reserved for future use and does not affect behavior.
    pub fn with_pool(mut self, pool: usize) -> Self {
        self.pool = pool;
        self
    }

    /// Sets the request timeout for the client.
    ///
    /// # Arguments
    ///
    /// * `timeout` - Request timeout in seconds
    ///
    /// # Returns
    ///
    /// The client instance for method chaining
    ///
    /// # Note
    ///
    /// This setting is currently reserved for future use and does not affect behavior.
    pub fn with_timeout(mut self, timeout: usize) -> Self {
        self.timeout = timeout;
        self
    }

    /// Retrieves a signature request by its ID.
    ///
    /// # Arguments
    ///
    /// * `signature_request_id` - The unique identifier of the signature request
    ///
    /// # Returns
    ///
    /// A tuple containing the signature request data and any warnings, or an error
    /// if the request fails or the signature request is not found.
    ///
    /// # Errors
    ///
    /// Returns `DropboxSignClientError` if:
    /// - The HTTP request fails
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dropboxsign_rs::DropboxSignClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DropboxSignClient::new("your-api-key");
    /// let (signature_request, warnings) = client
    ///     .get_signature_request("signature_request_id")
    ///     .await?;
    ///
    /// println!("Title: {}", signature_request.title);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_signature_request(
        &self,
        signature_request_id: &str,
    ) -> Result<(SignatureRequestResponse, Option<Vec<WarningResponse>>), DropboxSignClientError>
    {
        let url = format!("{}/signature_request/{signature_request_id}", API_URL);

        let response = self
            .client
            .get(&url)
            .basic_auth(&self.api_key, Some(""))
            .send()
            .await?;

        let status = response.status();

        if status == StatusCode::OK {
            let (sig_req, warnings) =
                parse_response::<SignatureRequestResponse>(response, "signature_request") // Add the key parameter
                    .await
                    .map_err(DropboxSignClientError::Other)?;
            Ok((sig_req, warnings))
        } else {
            let body = response.text().await?;
            let parsed: ErrorResponse = serde_json::from_str(&body)?;
            Err(DropboxSignClientError::ResponseError(parsed.error))
        }
    }

    /// Sends a signature request using a template.
    ///
    /// This method creates and sends a signature request based on a pre-existing
    /// template. The template defines the document layout and form fields, while
    /// the request specifies the signers and other dynamic parameters.
    ///
    /// # Arguments
    ///
    /// * `send_signature_request` - The signature request configuration including
    ///   signers, template IDs, and optional parameters
    ///
    /// # Returns
    ///
    /// A tuple containing the created signature request data and any warnings,
    /// or an error if the request fails.
    ///
    /// # Errors
    ///
    /// Returns `DropboxSignClientError` if:
    /// - The HTTP request fails
    /// - The API returns an error response (e.g., invalid template ID, missing signers)
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dropboxsign_rs::{DropboxSignClient, signature_request::*};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DropboxSignClient::new("your-api-key");
    ///
    /// let signer = SubSignatureRequestTemplateSigner::new(
    ///     "Signer".to_string(),
    ///     "John Doe".to_string(),
    ///     "john@example.com".to_string()
    /// );
    ///
    /// let request = SendSignatureRequest::new(
    ///     vec![signer],
    ///     vec!["template-id".to_string()]
    /// )
    /// .title("Contract Signature".to_string())
    /// .test_mode(true);
    ///
    /// let (signature_request, warnings) = client
    ///     .send_with_template(request)
    ///     .await?;
    ///
    /// println!("Sent signature request: {}", signature_request.signature_request_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_with_template(
        &self,
        send_signature_request: SendSignatureRequest,
    ) -> Result<(SignatureRequestResponse, Option<Vec<WarningResponse>>), DropboxSignClientError>
    {
        let url = format!("{}/signature_request/send_with_template", API_URL);

        let response = self
            .client
            .post(&url)
            .basic_auth(&self.api_key, Some(""))
            .json(&send_signature_request)
            .send()
            .await?;

        let status = response.status();

        if status == StatusCode::OK {
            let (sig_req, warnings) =
                parse_response::<SignatureRequestResponse>(response, "signature_request").await?;
            println!("Dropbox send_with_template response: {sig_req:?}");
            Ok((sig_req, warnings))
        } else {
            let body = response.text().await?;
            let parsed: ErrorResponse = serde_json::from_str(&body)?;
            Err(DropboxSignClientError::ResponseError(parsed.error))
        }
    }
}
