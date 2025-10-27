//! Data models and types for signature request operations.
//!
//! This module contains all the data structures needed for creating, sending,
//! and receiving signature requests through the Dropbox Sign API.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request structure for sending signature requests with templates.
///
/// This struct represents a complete signature request that will be sent to signers.
/// It requires signers and template IDs, with many optional configuration parameters
/// for customizing the signing experience.
///
/// # Examples
///
/// ```no_run
/// use dropboxsign_rs::signature_request::*;
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
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct SendSignatureRequest {
    /// List of signers who will receive the signature request
    pub signers: Vec<SubSignatureRequestTemplateSigner>,
    /// List of template IDs to use for this signature request
    pub template_ids: Vec<String>,
    /// Whether signers can decline to sign (default: true)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_decline: Option<bool>,
    /// List of CC recipients who will receive copies of the signature request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ccs: Option<Vec<SubCC>>,
    /// Client ID for API apps
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Custom form fields to pre-populate in the document
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<SubCustomField>>,
    /// File data as byte arrays (alternative to file_urls)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<Vec<u8>>>,
    /// URLs to files to be signed (alternative to files)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_urls: Option<Vec<String>>,
    /// Whether to enable eIDAS compliance (European electronic signatures)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_eid: Option<bool>,
    /// Custom message to include in the signature request email
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Key-value pairs for storing custom data with the signature request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// Configuration for signature methods and options
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signing_options: Option<SubSigningOptions>,
    /// URL to redirect signers to after completing their signature
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signing_redirect_url: Option<String>,
    /// Whether to create the signature request in test mode
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test_mode: Option<bool>,
    /// Title for the signature request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Represents a signer in a template-based signature request.
///
/// Each signer must have a role (matching the template), name, and email address.
/// Additional authentication options like PIN or SMS can be configured.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubSignatureRequestTemplateSigner {
    /// Role name that matches a role defined in the template
    pub role: String,
    /// Full name of the signer
    pub name: String,
    /// Email address where the signature request will be sent
    pub email_address: String,
    /// Optional PIN for additional security (4-12 digits)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pin: Option<String>,
    /// Phone number for SMS authentication or delivery
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sms_phone_number: Option<String>,
    /// Type of SMS usage (authentication or delivery)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sms_phone_number_type: Option<SMSPhoneNumberType>,
}

/// Specifies how SMS phone numbers are used in signature requests.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SMSPhoneNumberType {
    /// SMS is used for two-factor authentication
    Authentication,
    /// SMS is used for document delivery notifications
    Delivery,
}

/// Carbon copy recipient for signature requests.
///
/// CC recipients receive copies of signature request emails and completion notifications
/// but are not required to sign the document.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubCC {
    /// Role name for the CC recipient (must match template if using templates)
    pub role: String,
    /// Email address of the CC recipient
    pub email: String,
}

/// Custom form field that can be pre-populated in signature requests.
///
/// Custom fields allow you to set default values for form fields in the document
/// before sending it to signers.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubCustomField {
    /// Name of the custom field (must match field name in template)
    pub name: String,
    /// Email address of the person who can edit this field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editor: Option<String>,
    /// Whether this field is required to be filled out
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Default value for the field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Configuration for available signature methods.
///
/// Defines which signature methods are available to signers and which one
/// is the default option.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubSigningOptions {
    /// Default signature method that will be pre-selected
    pub default_type: SubSigningOptionsDefaultType,
    /// Whether signers can draw their signature
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draw: Option<bool>,
    /// Whether signers can use phone-based signatures
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<bool>,
    /// Whether signers can type their signature
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub o_type: Option<bool>,
    /// Whether signers can upload an image of their signature
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upload: Option<bool>,
}

/// Available signature methods for the default signing option.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SubSigningOptionsDefaultType {
    /// Draw signature with mouse/finger
    Draw,
    /// Phone-based signature verification
    Phone,
    /// Type signature using a font
    Type,
    /// Upload an image of the signature
    Upload,
}

/// Complete response data for a signature request.
///
/// Contains all information about a signature request including its status,
/// signer information, URLs, and metadata.
#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureRequestResponse {
    /// Whether this signature request was created in test mode
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test_mode: Option<bool>,
    /// Unique identifier for this signature request
    pub signature_request_id: String,
    /// Email address of the person who created this request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requester_email_address: Option<String>,
    /// Current title of the signature request
    pub title: String,
    /// Original title of the signature request (before any modifications)
    pub original_title: String,
    /// Subject line used in signature request emails
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Custom message included in signature request emails
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Custom metadata key-value pairs
    pub metadata: HashMap<String, String>,
    /// Unix timestamp when the signature request was created
    pub created_at: u64,
    /// Unix timestamp when the signature request expires (if set)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u64>,
    /// Whether all required signatures have been completed
    pub is_complete: bool,
    /// Whether any signer has declined to sign
    pub is_declined: bool,
    /// Whether there are any errors with this signature request
    pub has_error: bool,
    /// URL to download the signed documents
    pub files_url: String,
    /// URL for signers to access the signing interface
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signing_url: Option<String>,
    /// URL to view signature request details
    pub details_url: String,
    /// Email addresses that received CC copies of the request
    pub cc_email_addresses: Vec<String>,
    /// URL to redirect signers after they complete signing
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signing_redirect_url: Option<String>,
    /// URI to the final signed document copy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub final_copy_uri: Option<String>,
    /// Template IDs used to create this signature request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_ids: Option<Vec<String>>,
    /// Custom IDs associated with this signature request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_ids: Option<Vec<String>>,
    /// File attachments associated with this signature request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<SignatureRequestResponseAttachment>,
    /// Form field response data from signers
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_data: Option<Vec<SignatureRequestResponseData>>,
    /// Individual signature status for each signer
    pub signatures: Vec<SignatureRequestResponseSignatures>,
    /// Bulk send job ID if this was part of a bulk operation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bulk_send_job_id: Option<String>,
}

/// Base structure for custom form fields in signature request responses.
///
/// Represents form fields that were filled out by signers or pre-populated
/// when the signature request was created.
#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureRequestResponseCustomFieldBase {
    /// Type of the form field (text, checkbox, etc.)
    #[serde(rename = "type")]
    pub o_type: SignatureRequestResponseCustomFieldBaseType,
    /// Name/identifier of the form field
    pub name: String,
    /// Whether this field was required to be filled out
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// API identifier for this field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    /// Email of the person who can edit this field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editor: Option<String>,
    /// Current value of the form field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Types of custom form fields available in signature requests.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SignatureRequestResponseCustomFieldBaseType {
    /// Single-line or multi-line text input field
    Text,
    /// Checkbox that can be checked or unchecked
    Checkbox,
}

/// File attachment associated with a signature request.
///
/// Represents additional documents that signers can upload as part of
/// the signing process.
#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureRequestResponseAttachment {
    /// Unique identifier for this attachment
    pub id: String,
    /// Email address of the signer this attachment is assigned to
    pub signer: String,
    /// Display name of the attachment
    pub name: String,
    /// Whether uploading this attachment is required
    pub required: bool,
    /// Instructions for the signer about this attachment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Unix timestamp when the attachment was uploaded
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_at: Option<u64>,
}

/// Form field response data from a signature request.
///
/// Contains the values that signers entered in form fields, along with
/// metadata about each field.
#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureRequestResponseData {
    /// API identifier for this form field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    /// ID of the signature this data belongs to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_id: Option<String>,
    /// Name/label of the form field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether this field was required
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Type of form field (text, checkbox, dropdown, etc.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub o_type: Option<SignatureRequestResponseDataType>,
    /// Value entered by the signer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Individual signature status and metadata for each signer.
///
/// Contains detailed information about each signer's interaction with
/// the signature request, including status, timestamps, and authentication details.
#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureRequestResponseSignatures {
    /// Unique identifier for this signature
    pub signature_id: String,

    /// Group GUID if this signer belongs to a signer group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signer_group_guid: Option<String>,

    /// Email address of the signer
    pub signer_email_address: String,

    /// Full name of the signer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signer_name: Option<String>,

    /// Role of the signer in the signature request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signer_role: Option<String>,

    /// Signing order (for sequential signing workflows)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,

    /// Current status of this signature (awaiting_signature, signed, declined, etc.)
    pub status_code: String,

    /// Reason provided if the signer declined to sign
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decline_reason: Option<String>,

    /// Unix timestamp when the signature was completed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signed_at: Option<i64>,

    /// Unix timestamp when the signer last viewed the document
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_viewed_at: Option<i64>,

    /// Unix timestamp when the signer was last sent a reminder
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_reminded_at: Option<i64>,

    /// Whether this signer is required to enter a PIN
    pub has_pin: bool,

    /// Whether SMS authentication is enabled for this signer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_sms_auth: Option<bool>,

    /// Whether SMS delivery notifications are enabled for this signer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_sms_delivery: Option<bool>,

    /// Phone number used for SMS authentication or delivery
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sms_phone_number: Option<String>,

    /// Email of the person who reassigned this signature
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reassigned_by: Option<String>,

    /// Reason provided for reassigning this signature
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reassignment_reason: Option<String>,

    /// Email of the original signer before reassignment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reassigned_from: Option<String>,

    /// Error message if there was a problem with this signature
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Types of form fields that can appear in signature request responses.
///
/// Covers all possible field types that signers can interact with in documents.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SignatureRequestResponseDataType {
    /// Single-line or multi-line text input
    Text,
    /// Checkbox that can be checked or unchecked
    Checkbox,
    /// Dropdown menu with predefined options
    Dropdown,
    /// Radio button group (single selection)
    Radio,
    /// Electronic signature field
    Signature,
    /// Automatically filled date when document was signed
    #[serde(rename = "date_signed")]
    DateSigned,
    /// Initial signature field
    Initials,
    /// Text field merged from template data
    #[serde(rename = "text-merge")]
    TextMerge,
    /// Checkbox field merged from template data
    #[serde(rename = "checkbox-merge")]
    CheckboxMerge,
}

impl SendSignatureRequest {
    /// Creates a new signature request with the minimum required fields.
    ///
    /// # Arguments
    ///
    /// * `signers` - List of signers who will receive the signature request
    /// * `template_ids` - List of template IDs to use for this signature request
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dropboxsign_rs::signature_request::*;
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
    /// );
    /// ```
    pub fn new(signers: Vec<SubSignatureRequestTemplateSigner>, template_ids: Vec<String>) -> Self {
        Self {
            signers,
            template_ids,
            allow_decline: None,
            ccs: None,
            client_id: None,
            custom_fields: None,
            files: None,
            file_urls: None,
            is_eid: None,
            message: None,
            metadata: None,
            signing_options: None,
            signing_redirect_url: None,
            test_mode: None,
            title: None,
        }
    }

    /// Sets whether signers can decline to sign the document.
    ///
    /// # Arguments
    ///
    /// * `allow_decline` - If true, signers can decline; if false, they must either sign or ignore
    pub fn allow_decline(mut self, allow_decline: bool) -> Self {
        self.allow_decline = Some(allow_decline);
        self
    }

    /// Sets the list of CC recipients for the signature request.
    ///
    /// # Arguments
    ///
    /// * `ccs` - List of people who will receive copies of signature request emails
    pub fn ccs(mut self, ccs: Vec<SubCC>) -> Self {
        self.ccs = Some(ccs);
        self
    }

    /// Sets the client ID for API apps.
    ///
    /// # Arguments
    ///
    /// * `client_id` - Client ID for your API app
    pub fn client_id(mut self, client_id: String) -> Self {
        self.client_id = Some(client_id);
        self
    }

    /// Sets custom form fields to pre-populate in the document.
    ///
    /// # Arguments
    ///
    /// * `custom_fields` - List of custom fields with default values
    pub fn custom_fields(mut self, custom_fields: Vec<SubCustomField>) -> Self {
        self.custom_fields = Some(custom_fields);
        self
    }

    /// Sets file data as byte arrays for documents to be signed.
    ///
    /// # Arguments
    ///
    /// * `files` - List of file contents as byte arrays
    pub fn files(mut self, files: Vec<Vec<u8>>) -> Self {
        self.files = Some(files);
        self
    }

    /// Sets URLs to files that should be downloaded and used as documents.
    ///
    /// # Arguments
    ///
    /// * `file_urls` - List of publicly accessible URLs to PDF files
    pub fn file_urls(mut self, file_urls: Vec<String>) -> Self {
        self.file_urls = Some(file_urls);
        self
    }

    /// Sets whether to enable eIDAS compliance for European electronic signatures.
    ///
    /// # Arguments
    ///
    /// * `is_eid` - True to enable eIDAS compliance
    pub fn is_eid(mut self, is_eid: bool) -> Self {
        self.is_eid = Some(is_eid);
        self
    }

    /// Sets a custom message to include in signature request emails.
    ///
    /// # Arguments
    ///
    /// * `message` - Custom message text (supports basic HTML)
    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    /// Sets custom metadata key-value pairs for the signature request.
    ///
    /// # Arguments
    ///
    /// * `metadata` - Key-value pairs for storing custom data
    pub fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Sets configuration for available signature methods.
    ///
    /// # Arguments
    ///
    /// * `signing_options` - Configuration for signature method preferences
    pub fn signing_options(mut self, signing_options: SubSigningOptions) -> Self {
        self.signing_options = Some(signing_options);
        self
    }

    /// Sets the URL to redirect signers to after they complete signing.
    ///
    /// # Arguments
    ///
    /// * `signing_redirect_url` - URL for post-signing redirect
    pub fn signing_redirect_url(mut self, signing_redirect_url: String) -> Self {
        self.signing_redirect_url = Some(signing_redirect_url);
        self
    }

    /// Sets whether to create the signature request in test mode.
    ///
    /// # Arguments
    ///
    /// * `test_mode` - True for test mode (no emails sent, no charges apply)
    pub fn test_mode(mut self, test_mode: bool) -> Self {
        self.test_mode = Some(test_mode);
        self
    }

    /// Sets the title for the signature request.
    ///
    /// # Arguments
    ///
    /// * `title` - Title that will appear in emails and the signing interface
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }
}

impl SubSignatureRequestTemplateSigner {
    /// Creates a new signer with the minimum required information.
    ///
    /// # Arguments
    ///
    /// * `role` - Role name that matches a role defined in the template
    /// * `name` - Full name of the signer
    /// * `email_address` - Email address where the signature request will be sent
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dropboxsign_rs::signature_request::SubSignatureRequestTemplateSigner;
    ///
    /// let signer = SubSignatureRequestTemplateSigner::new(
    ///     "Signer".to_string(),
    ///     "Jane Doe".to_string(),
    ///     "jane@example.com".to_string()
    /// );
    /// ```
    pub fn new(role: String, name: String, email_address: String) -> Self {
        Self {
            role,
            name,
            email_address,
            pin: None,
            sms_phone_number: None,
            sms_phone_number_type: None,
        }
    }

    /// Sets a PIN that the signer must enter before signing.
    ///
    /// # Arguments
    ///
    /// * `pin` - 4-12 digit PIN for additional security
    pub fn pin(mut self, pin: String) -> Self {
        self.pin = Some(pin);
        self
    }

    /// Sets the phone number for SMS authentication or delivery.
    ///
    /// # Arguments
    ///
    /// * `sms_phone_number` - Phone number in international format
    pub fn sms_phone_number(mut self, sms_phone_number: String) -> Self {
        self.sms_phone_number = Some(sms_phone_number);
        self
    }

    /// Sets how the SMS phone number should be used.
    ///
    /// # Arguments
    ///
    /// * `sms_phone_number_type` - Whether to use SMS for authentication or delivery
    pub fn sms_phone_number_type(mut self, sms_phone_number_type: SMSPhoneNumberType) -> Self {
        self.sms_phone_number_type = Some(sms_phone_number_type);
        self
    }
}

impl SubCC {
    /// Creates a new CC recipient.
    ///
    /// # Arguments
    ///
    /// * `role` - Role name for the CC recipient
    /// * `email` - Email address of the CC recipient
    pub fn new(role: String, email: String) -> Self {
        Self { role, email }
    }
}

impl SubCustomField {
    /// Creates a new custom field with the specified name.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the custom field (must match field name in template)
    pub fn new(name: String) -> Self {
        Self {
            name,
            editor: None,
            required: None,
            value: None,
        }
    }

    /// Sets the email address of the person who can edit this field.
    ///
    /// # Arguments
    ///
    /// * `editor` - Email address of the field editor
    pub fn editor(mut self, editor: String) -> Self {
        self.editor = Some(editor);
        self
    }

    /// Sets whether this field is required to be filled out.
    ///
    /// # Arguments
    ///
    /// * `required` - True if the field must be completed
    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    /// Sets the default value for this field.
    ///
    /// # Arguments
    ///
    /// * `value` - Default value to pre-populate the field
    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }
}

impl SubSigningOptions {
    /// Creates new signing options with the specified default signature method.
    ///
    /// # Arguments
    ///
    /// * `default_type` - Default signature method that will be pre-selected
    pub fn new(default_type: SubSigningOptionsDefaultType) -> Self {
        Self {
            default_type,
            draw: None,
            phone: None,
            o_type: None,
            upload: None,
        }
    }

    /// Sets whether signers can draw their signature with mouse or finger.
    ///
    /// # Arguments
    ///
    /// * `draw` - True to enable drawing signatures
    pub fn draw(mut self, draw: bool) -> Self {
        self.draw = Some(draw);
        self
    }

    /// Sets whether signers can use phone-based signature verification.
    ///
    /// # Arguments
    ///
    /// * `phone` - True to enable phone signatures
    pub fn phone(mut self, phone: bool) -> Self {
        self.phone = Some(phone);
        self
    }

    /// Sets whether signers can type their signature using a font.
    ///
    /// # Arguments
    ///
    /// * `o_type` - True to enable typed signatures
    pub fn o_type(mut self, o_type: bool) -> Self {
        self.o_type = Some(o_type);
        self
    }

    /// Sets whether signers can upload an image of their signature.
    ///
    /// # Arguments
    ///
    /// * `upload` - True to enable signature uploads
    pub fn upload(mut self, upload: bool) -> Self {
        self.upload = Some(upload);
        self
    }
}

impl SignatureRequestResponseCustomFieldBase {
    /// Creates a new custom field response with the specified type and name.
    ///
    /// # Arguments
    ///
    /// * `o_type` - Type of the form field (text, checkbox, etc.)
    /// * `name` - Name/identifier of the form field
    pub fn new(o_type: SignatureRequestResponseCustomFieldBaseType, name: String) -> Self {
        Self {
            o_type,
            name,
            required: None,
            api_id: None,
            editor: None,
            value: None,
        }
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    /// Sets the API identifier for this field.
    ///
    /// # Arguments
    ///
    /// * `api_id` - Unique API identifier for the field
    pub fn api_id(mut self, api_id: String) -> Self {
        self.api_id = Some(api_id);
        self
    }

    pub fn editor(mut self, editor: String) -> Self {
        self.editor = Some(editor);
        self
    }

    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }
}

impl SignatureRequestResponseAttachment {
    /// Creates a new attachment response with the required fields.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for this attachment
    /// * `signer` - Email address of the signer this attachment is assigned to
    /// * `name` - Display name of the attachment
    /// * `required` - Whether uploading this attachment is required
    pub fn new(id: String, signer: String, name: String, required: bool) -> Self {
        Self {
            id,
            signer,
            name,
            required,
            instructions: None,
            uploaded_at: None,
        }
    }

    /// Sets instructions for the signer about this attachment.
    ///
    /// # Arguments
    ///
    /// * `instructions` - Instructions text for the signer
    pub fn instructions(mut self, instructions: String) -> Self {
        self.instructions = Some(instructions);
        self
    }

    /// Sets the timestamp when the attachment was uploaded.
    ///
    /// # Arguments
    ///
    /// * `uploaded_at` - Unix timestamp of upload time
    pub fn uploaded_at(mut self, uploaded_at: u64) -> Self {
        self.uploaded_at = Some(uploaded_at);
        self
    }
}
