use porter::error::Result;
use porter::google::{
    Barcode, EventSeat, EventTicketObject, GenericObject, GoogleWalletClient, GoogleWalletConfig,
    LocalizedString, TranslatedString,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = GoogleWalletConfig {
        issuer_id: std::env::var("GOOGLE_WALLET_ISSUER_ID")
            .expect("GOOGLE_WALLET_ISSUER_ID must be set"),
        service_account_email: std::env::var("GOOGLE_WALLET_SERVICE_ACCOUNT")
            .expect("GOOGLE_WALLET_SERVICE_ACCOUNT must be set"),
        private_key: std::fs::read_to_string("service-account-key.pem")
            .expect("Failed to read private key"),
    };

    let mut client = GoogleWalletClient::new(config.clone());

    // Create an event ticket
    println!("Creating event ticket...");
    let ticket_id = format!("{}.concert_ticket_001", config.issuer_id);
    let ticket = EventTicketObject {
        id: ticket_id.clone(),
        class_id: format!("{}.concert_class", config.issuer_id),
        state: Some("ACTIVE".to_string()),
        ticket_holder_name: Some("Jane Smith".to_string()),
        seat_info: Some(EventSeat {
            seat: Some(LocalizedString {
                default_value: Some(TranslatedString {
                    language: "en-US".to_string(),
                    value: "B15".to_string(),
                }),
                translated_values: None,
            }),
            row: Some(LocalizedString {
                default_value: Some(TranslatedString {
                    language: "en-US".to_string(),
                    value: "B".to_string(),
                }),
                translated_values: None,
            }),
            section: Some(LocalizedString {
                default_value: Some(TranslatedString {
                    language: "en-US".to_string(),
                    value: "Main Floor".to_string(),
                }),
                translated_values: None,
            }),
        }),
        barcode: Some(Barcode {
            barcode_type: "QR_CODE".to_string(),
            value: "CONCERT-B15-001".to_string(),
            alternate_text: Some("CONCERT-B15-001".to_string()),
        }),
    };

    let created_ticket = client.create_event_ticket(&ticket).await?;
    println!("✓ Created ticket: {}", created_ticket.id);
    println!(
        "  Holder: {}",
        created_ticket.ticket_holder_name.unwrap_or_default()
    );

    if let Some(seat_info) = &created_ticket.seat_info {
        if let Some(seat) = &seat_info.seat {
            if let Some(default) = &seat.default_value {
                println!("  Seat: {}", default.value);
            }
        }
    }

    // Generate save URL
    // Note: We need to get the full object for JWT generation
    // For event tickets, we'd typically use eventTicketObjects in the JWT
    // For simplicity here, we'll use the generic object pattern
    let generic_obj = GenericObject {
        id: created_ticket.id.clone(),
        class_id: created_ticket.class_id.clone(),
        state: created_ticket.state.clone(),
        barcode: created_ticket.barcode.clone(),
        ..Default::default()
    };
    let save_url = client.generate_save_url(&generic_obj).await?;
    println!("\n🎫 Add ticket to Google Wallet:");
    println!("{}", save_url);

    Ok(())
}
