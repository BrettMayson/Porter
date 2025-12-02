use crate::google::types::{Barcode as GoogleBarcode, GenericObject, LocalizedString, TranslatedString};
use crate::models::{Barcode, BarcodeFormat, Pass, PassState};

/// Convert a unified Pass model to a Google Wallet GenericObject
impl From<Pass> for GenericObject {
    fn from(pass: Pass) -> Self {
        Self::from(&pass)
    }
}

impl From<&Pass> for GenericObject {
    fn from(pass: &Pass) -> Self {
    let barcode = pass.barcode.as_ref().map(|b| GoogleBarcode {
        barcode_type: match b.format {
            BarcodeFormat::QrCode => "QR_CODE",
            BarcodeFormat::Pdf417 => "PDF_417",
            BarcodeFormat::Aztec => "AZTEC",
            BarcodeFormat::Code128 => "CODE_128",
        }
        .to_string(),
        value: b.value.clone(),
        alternate_text: b.alternate_text.clone(),
    });

    let state = Some(match pass.state {
        PassState::Active => "ACTIVE",
        PassState::Inactive => "INACTIVE",
        PassState::Expired => "EXPIRED",
        PassState::Completed => "COMPLETED",
    }
    .to_string());

    let card_title = Some(LocalizedString {
        default_value: Some(TranslatedString {
            language: "en-US".to_string(),
            value: pass.header.title.clone(),
        }),
        translated_values: None,
    });

    let header = pass.header.subtitle.as_ref().map(|subtitle| LocalizedString {
        default_value: Some(TranslatedString {
            language: "en-US".to_string(),
            value: subtitle.clone(),
        }),
        translated_values: None,
    });

        GenericObject {
            id: pass.id.clone(),
            class_id: pass.class_id.clone(),
            state,
            barcode,
            card_title,
            header,
            subheader: None,
            logo: None,
            hex_background_color: pass.header.background_color.clone(),
            hero_image: None,
            valid_time_interval: None,
            linked_offer_ids: if pass.linked_objects.is_empty() {
                None
            } else {
                Some(pass.linked_objects.clone())
            },
        }
    }
}

/// Convert a Google Wallet GenericObject to a unified Pass model
impl From<GenericObject> for Pass {
    fn from(object: GenericObject) -> Self {
        Self::from(&object)
    }
}

impl From<&GenericObject> for Pass {
    fn from(object: &GenericObject) -> Self {
    let barcode = object.barcode.as_ref().map(|b| {
        let format = match b.barcode_type.as_str() {
            "QR_CODE" => BarcodeFormat::QrCode,
            "PDF_417" => BarcodeFormat::Pdf417,
            "AZTEC" => BarcodeFormat::Aztec,
            "CODE_128" => BarcodeFormat::Code128,
            _ => BarcodeFormat::QrCode, // default
        };

        Barcode {
            format,
            value: b.value.clone(),
            alternate_text: b.alternate_text.clone(),
        }
    });

    let state = match object.state.as_deref() {
        Some("ACTIVE") => PassState::Active,
        Some("INACTIVE") => PassState::Inactive,
        Some("EXPIRED") => PassState::Expired,
        Some("COMPLETED") => PassState::Completed,
        _ => PassState::Active, // default
    };

    let title = object
        .card_title
        .as_ref()
        .and_then(|t| t.default_value.as_ref())
        .map(|v| v.value.clone())
        .unwrap_or_default();

    let subtitle = object
        .header
        .as_ref()
        .and_then(|h| h.default_value.as_ref())
        .map(|v| v.value.clone());

        Pass {
            id: object.id.clone(),
            class_id: object.class_id.clone(),
            pass_type: crate::models::PassType::Generic,
            header: crate::models::PassHeader {
                title,
                subtitle,
                logo: None,
                background_color: object.hex_background_color.clone(),
                foreground_color: None,
            },
            barcode,
            fields: vec![],
            linked_objects: object.linked_offer_ids.clone().unwrap_or_default(),
            state,
            valid_time_interval: None,
            updated_at: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_to_google_object() {
        let pass = Pass {
            id: "test.pass".to_string(),
            class_id: "test.class".to_string(),
            pass_type: crate::models::PassType::Generic,
            header: crate::models::PassHeader {
                title: "Test Pass".to_string(),
                subtitle: Some("Subtitle".to_string()),
                logo: None,
                background_color: Some("#FF0000".to_string()),
                foreground_color: None,
            },
            barcode: Some(Barcode {
                format: BarcodeFormat::QrCode,
                value: "12345".to_string(),
                alternate_text: Some("12345".to_string()),
            }),
            fields: vec![],
            linked_objects: vec![],
            state: PassState::Active,
            valid_time_interval: None,
            updated_at: None,
        };

        let google_object: GenericObject = pass.into();
        assert_eq!(google_object.id, "test.pass");
        assert_eq!(google_object.class_id, "test.class");
        assert_eq!(google_object.state, Some("ACTIVE".to_string()));
        assert!(google_object.barcode.is_some());
    }

    #[test]
    fn test_google_object_to_pass() {
        let google_object = GenericObject {
            id: "test.object".to_string(),
            class_id: "test.class".to_string(),
            state: Some("ACTIVE".to_string()),
            barcode: Some(GoogleBarcode {
                barcode_type: "QR_CODE".to_string(),
                value: "54321".to_string(),
                alternate_text: Some("54321".to_string()),
            }),
            card_title: Some(LocalizedString {
                default_value: Some(TranslatedString {
                    language: "en-US".to_string(),
                    value: "Test Card".to_string(),
                }),
                translated_values: None,
            }),
            ..Default::default()
        };

        let pass: Pass = google_object.into();
        assert_eq!(pass.id, "test.object");
        assert_eq!(pass.class_id, "test.class");
        assert_eq!(pass.state, PassState::Active);
        assert!(pass.barcode.is_some());
        assert_eq!(pass.header.title, "Test Card");
    }
}
