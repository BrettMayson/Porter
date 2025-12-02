use serde::{Deserialize, Serialize};

/// Google Wallet Generic Object
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenericObject {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub class_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<Barcode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_title: Option<LocalizedString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<LocalizedString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subheader: Option<LocalizedString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex_background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hero_image: Option<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_time_interval: Option<TimeInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_offer_ids: Option<Vec<String>>,
}

/// Google Wallet Generic Class
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericClass {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_status: Option<String>,
}

/// Localized string for multi-language support
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalizedString {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<TranslatedString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_values: Option<Vec<TranslatedString>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslatedString {
    pub language: String,
    pub value: String,
}

/// Barcode definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Barcode {
    #[serde(rename = "type")]
    pub barcode_type: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_text: Option<String>,
}

/// Image definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub source_uri: ImageUri,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_description: Option<LocalizedString>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageUri {
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Time interval
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeInterval {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateTime {
    pub date: String, // ISO 8601 format
}

/// Message to add to a pass
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMessageRequest {
    pub message: Message,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_interval: Option<TimeInterval>,
}

/// List response for objects
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericObjectListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<GenericObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results_per_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

/// Event Ticket Object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventTicketObject {
    pub id: String,
    pub class_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<Barcode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_info: Option<EventSeat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_holder_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventSeat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat: Option<LocalizedString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row: Option<LocalizedString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<LocalizedString>,
}

/// Loyalty Object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoyaltyObject {
    pub id: String,
    pub class_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<Barcode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_points: Option<LoyaltyPoints>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoyaltyPoints {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<LoyaltyPointsBalance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoyaltyPointsBalance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double: Option<f64>,
}
