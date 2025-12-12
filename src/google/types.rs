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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_modules_data: Option<Vec<TextModuleData>>,
}

/// Google Wallet Generic Class
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenericClass {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub class_template_info: Option<ClassTemplateInfo>,
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

/// JWT payload for creating save URLs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JwtPayload {
    pub iss: String, // Issuer (service account email)
    pub aud: String, // Audience (should be "google")
    pub typ: String, // Type (should be "savetowallet")
    pub iat: i64,    // Issued at timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origins: Option<Vec<String>>,
    pub payload: JwtObjectPayload,
}

/// Container for objects to be saved
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JwtObjectPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic_objects: Option<Vec<GenericObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ticket_objects: Option<Vec<EventTicketObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_objects: Option<Vec<LoyaltyObject>>,
}

/// Request body for JWT insert endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtResource {
    pub jwt: String,
}

/// Response from JWT insert endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtInsertResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_uri: Option<String>,
}

/// Text module data for displaying custom fields
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextModuleData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localized_header: Option<LocalizedString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localized_body: Option<LocalizedString>,
}

/// Template information about how the class should be displayed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassTemplateInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_template_override: Option<CardTemplateOverride>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_template_override: Option<DetailsTemplateOverride>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_template_override: Option<ListTemplateOverride>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_barcode_section_details: Option<CardBarcodeSectionDetails>,
}

/// Override for the card view
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardTemplateOverride {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_row_template_infos: Option<Vec<CardRowTemplateInfo>>,
}

/// Template for a row in the card
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRowTemplateInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_item: Option<CardRowOneItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_items: Option<CardRowTwoItems>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_items: Option<CardRowThreeItems>,
}

/// Template for a row containing one item
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRowOneItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<TemplateItem>,
}

/// Template for a row containing two items
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRowTwoItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_item: Option<TemplateItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_item: Option<TemplateItem>,
}

/// Template for a row containing three items
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRowThreeItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_item: Option<TemplateItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_item: Option<TemplateItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_item: Option<TemplateItem>,
}

/// Template item that can display field data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_value: Option<FieldSelector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_item: Option<String>,
}

/// Field selector for referencing fields
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldSelector {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<FieldReference>>,
}

/// Reference to a specific field
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_format: Option<String>,
}

/// Override for the details view
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailsTemplateOverride {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_item_infos: Option<Vec<DetailsItemInfo>>,
}

/// Item info for the details view
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailsItemInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<TemplateItem>,
}

/// Override for the list view
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTemplateOverride {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_row_option: Option<FirstRowOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_row_option: Option<FieldSelector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_row_option: Option<FieldSelector>,
}

/// Options for the first row in list view
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstRowOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_option: Option<FieldSelector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_option: Option<String>,
}

/// Card barcode section details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardBarcodeSectionDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_top_detail: Option<BarcodeSectionDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_top_detail: Option<BarcodeSectionDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_bottom_detail: Option<BarcodeSectionDetail>,
}

/// Barcode section detail
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BarcodeSectionDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_selector: Option<FieldSelector>,
}
