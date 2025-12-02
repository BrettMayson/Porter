use crate::models::*;

/// Builder for creating passes with a fluent API
/// 
/// This provides a platform-agnostic way to create passes that can then
/// be converted to Google Wallet or Apple Wallet specific formats.
/// 
/// # Example
/// 
/// ```
/// use porter::builder::PassBuilder;
/// use porter::models::{PassType, BarcodeFormat};
/// 
/// let pass = PassBuilder::new("issuer_id.pass_001", "issuer_id.class_001")
///     .pass_type(PassType::EventTicket)
///     .title("Concert Ticket")
///     .subtitle("The Rust Band Live")
///     .barcode(BarcodeFormat::QrCode, "TICKET123456")
///     .background_color("#4285F4")
///     .field("seat", "Seat", "A23")
///     .field("section", "Section", "Main Floor")
///     .build();
/// ```
pub struct PassBuilder {
    pass: Pass,
}

impl PassBuilder {
    /// Create a new pass builder
    pub fn new(id: impl Into<String>, class_id: impl Into<String>) -> Self {
        Self {
            pass: Pass {
                id: id.into(),
                class_id: class_id.into(),
                pass_type: PassType::Generic,
                header: PassHeader {
                    title: String::new(),
                    subtitle: None,
                    logo: None,
                    background_color: None,
                    foreground_color: None,
                },
                barcode: None,
                fields: vec![],
                linked_objects: vec![],
                state: PassState::Active,
                valid_time_interval: None,
                updated_at: None,
            },
        }
    }

    /// Set the pass type
    pub fn pass_type(mut self, pass_type: PassType) -> Self {
        self.pass.pass_type = pass_type;
        self
    }

    /// Set the title (displayed prominently on the pass)
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.pass.header.title = title.into();
        self
    }

    /// Set the subtitle
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.pass.header.subtitle = Some(subtitle.into());
        self
    }

    /// Set the logo image
    pub fn logo(mut self, source_uri: impl Into<String>, alt_text: Option<String>) -> Self {
        self.pass.header.logo = Some(Image {
            source_uri: source_uri.into(),
            alt_text,
        });
        self
    }

    /// Set background color (hex format like "#FF0000")
    pub fn background_color(mut self, color: impl Into<String>) -> Self {
        self.pass.header.background_color = Some(color.into());
        self
    }

    /// Set foreground color (hex format like "#FFFFFF")
    pub fn foreground_color(mut self, color: impl Into<String>) -> Self {
        self.pass.header.foreground_color = Some(color.into());
        self
    }

    /// Add a barcode to the pass
    pub fn barcode(
        mut self,
        format: BarcodeFormat,
        value: impl Into<String>,
    ) -> Self {
        self.pass.barcode = Some(Barcode {
            format,
            value: value.into(),
            alternate_text: None,
        });
        self
    }

    /// Add a barcode with alternate text
    pub fn barcode_with_text(
        mut self,
        format: BarcodeFormat,
        value: impl Into<String>,
        alternate_text: impl Into<String>,
    ) -> Self {
        self.pass.barcode = Some(Barcode {
            format,
            value: value.into(),
            alternate_text: Some(alternate_text.into()),
        });
        self
    }

    /// Add a field to the pass
    pub fn field(
        mut self,
        key: impl Into<String>,
        label: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.pass.fields.push(PassField {
            key: key.into(),
            label: label.into(),
            value: value.into(),
            text_alignment: None,
        });
        self
    }

    /// Add a field with specific text alignment
    pub fn field_with_alignment(
        mut self,
        key: impl Into<String>,
        label: impl Into<String>,
        value: impl Into<String>,
        alignment: TextAlignment,
    ) -> Self {
        self.pass.fields.push(PassField {
            key: key.into(),
            label: label.into(),
            value: value.into(),
            text_alignment: Some(alignment),
        });
        self
    }

    /// Link another pass or offer
    pub fn link_object(mut self, object_id: impl Into<String>) -> Self {
        self.pass.linked_objects.push(object_id.into());
        self
    }

    /// Set the pass state
    pub fn state(mut self, state: PassState) -> Self {
        self.pass.state = state;
        self
    }

    /// Set validity time interval
    pub fn valid_from(mut self, start: chrono::DateTime<chrono::Utc>) -> Self {
        if let Some(ref mut interval) = self.pass.valid_time_interval {
            interval.start = start;
        } else {
            self.pass.valid_time_interval = Some(TimeInterval {
                start,
                end: None,
            });
        }
        self
    }

    /// Set validity end time
    pub fn valid_until(mut self, end: chrono::DateTime<chrono::Utc>) -> Self {
        if let Some(ref mut interval) = self.pass.valid_time_interval {
            interval.end = Some(end);
        } else {
            self.pass.valid_time_interval = Some(TimeInterval {
                start: chrono::Utc::now(),
                end: Some(end),
            });
        }
        self
    }

    /// Build the pass
    pub fn build(self) -> Pass {
        self.pass
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let pass = PassBuilder::new("test.pass", "test.class")
            .title("Test Pass")
            .build();

        assert_eq!(pass.id, "test.pass");
        assert_eq!(pass.class_id, "test.class");
        assert_eq!(pass.header.title, "Test Pass");
    }

    #[test]
    fn test_builder_full() {
        let pass = PassBuilder::new("event.ticket001", "event.concert")
            .pass_type(PassType::EventTicket)
            .title("Concert Ticket")
            .subtitle("The Rust Band")
            .background_color("#FF5733")
            .foreground_color("#FFFFFF")
            .barcode_with_text(BarcodeFormat::QrCode, "TICKET123", "TICKET123")
            .field("seat", "Seat", "A23")
            .field("row", "Row", "A")
            .field("section", "Section", "Main Floor")
            .state(PassState::Active)
            .build();

        assert_eq!(pass.pass_type, PassType::EventTicket);
        assert_eq!(pass.header.title, "Concert Ticket");
        assert_eq!(pass.header.subtitle, Some("The Rust Band".to_string()));
        assert_eq!(pass.fields.len(), 3);
        assert!(pass.barcode.is_some());
    }
}
