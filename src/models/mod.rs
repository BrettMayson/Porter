use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Platform-agnostic pass data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pass {
    /// Unique identifier for the pass
    pub id: String,

    /// Class ID that this pass belongs to
    pub class_id: String,

    /// Pass type
    pub pass_type: PassType,

    /// Header information
    pub header: PassHeader,

    /// Barcode data (if applicable)
    pub barcode: Option<Barcode>,

    /// Custom fields specific to the pass type
    pub fields: Vec<PassField>,

    /// Links to related passes or offers
    pub linked_objects: Vec<String>,

    /// State of the pass
    pub state: PassState,

    /// Validity period
    pub valid_time_interval: Option<TimeInterval>,

    /// Last updated timestamp
    pub updated_at: Option<DateTime<Utc>>,
}

/// Types of passes supported
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PassType {
    EventTicket,
    Flight,
    Generic,
    GiftCard,
    Loyalty,
    Offer,
    Transit,
}

/// Pass header information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassHeader {
    pub title: String,
    pub subtitle: Option<String>,
    pub logo: Option<Image>,
    pub background_color: Option<String>,
    pub foreground_color: Option<String>,
}

/// Image resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub source_uri: String,
    pub alt_text: Option<String>,
}

/// Barcode representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Barcode {
    pub format: BarcodeFormat,
    pub value: String,
    pub alternate_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BarcodeFormat {
    QrCode,
    Pdf417,
    Aztec,
    Code128,
}

/// Dynamic field on a pass
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassField {
    pub key: String,
    pub label: String,
    pub value: String,
    pub text_alignment: Option<TextAlignment>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Natural,
}

/// Pass state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PassState {
    Active,
    Inactive,
    Expired,
    Completed,
}

/// Time interval for pass validity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeInterval {
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
}

/// Message that can be sent to pass holders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassMessage {
    pub header: Option<String>,
    pub body: String,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

/// Class definition (template for passes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassClass {
    pub id: String,
    pub pass_type: PassType,
    pub issuer_name: String,
    pub review_status: ReviewStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReviewStatus {
    Draft,
    UnderReview,
    Approved,
    Rejected,
}
