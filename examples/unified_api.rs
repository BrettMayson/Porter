/// This example shows how to create passes using the unified PassBuilder API
/// The same pass definition can be used for Google Wallet or (in the future) Apple Wallet
use porter::builder::PassBuilder;
use porter::google::{GenericClass, GenericObject, GoogleWalletClient, GoogleWalletConfig};
use porter::models::{BarcodeFormat, PassType};
use porter::Result;

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

    // Step 1: Create a class using Google-specific types
    println!("Creating class...");
    let class_id = format!("{}.unified_example_class", config.issuer_id);
    let class = GenericClass {
        id: class_id.clone(),
        issuer_name: Some("Porter Unified API Demo".to_string()),
        review_status: Some("UNDER_REVIEW".to_string()),
        class_template_info: None,
    };

    match client.create_generic_class(&class).await {
        Ok(_) => println!("✓ Class created"),
        Err(_) => println!("✓ Class already exists"),
    }

    // Step 2: Create a pass using the UNIFIED builder
    // This is platform-agnostic and could work with Apple Wallet too
    println!("\nCreating pass with unified builder...");
    let unified_pass = PassBuilder::new(
        format!("{}.unified_pass_001", config.issuer_id),
        class_id.clone(),
    )
    .pass_type(PassType::EventTicket)
    .title("Concert Ticket")
    .subtitle("The Rust Band - World Tour 2025")
    .background_color("#8B5CF6")
    .foreground_color("#FFFFFF")
    .barcode_with_text(BarcodeFormat::QrCode, "UNIFIED-TICKET-12345", "12345")
    .field("venue", "Venue", "RustConf Arena")
    .field("date", "Date", "December 15, 2025")
    .field("seat", "Seat", "A23")
    .field("section", "Section", "Main Floor")
    .build();

    println!("✓ Unified pass model created");
    println!("  ID: {}", unified_pass.id);
    println!("  Title: {}", unified_pass.header.title);
    println!("  Type: {:?}", unified_pass.pass_type);
    println!("  Fields: {}", unified_pass.fields.len());

    // Step 3: Convert to Google Wallet format and create
    println!("\nConverting to Google Wallet format...");
    let google_pass: GenericObject = unified_pass.clone().into();

    let created_pass = client.create_generic_object(&google_pass).await?;
    println!("✓ Pass created in Google Wallet");

    // Step 4: Retrieve and convert back to unified model
    println!("\nRetrieving pass and converting to unified model...");
    let retrieved_google = client.get_generic_object(&created_pass.id).await?;
    let retrieved_unified: porter::models::Pass = retrieved_google.into();

    println!("✓ Retrieved as unified model:");
    println!("  Title: {}", retrieved_unified.header.title);
    println!("  State: {:?}", retrieved_unified.state);

    // Step 5: Update using unified model
    println!("\nUpdating pass using unified builder...");
    let updated_unified = PassBuilder::new(
        format!("{}.unified_pass_001", config.issuer_id),
        class_id.clone(),
    )
    .pass_type(PassType::EventTicket)
    .title("Concert Ticket")
    .subtitle("The Rust Band - DOORS OPEN 7PM") // Updated
    .background_color("#8B5CF6")
    .foreground_color("#FFFFFF")
    .barcode_with_text(BarcodeFormat::QrCode, "UNIFIED-TICKET-12345", "12345")
    .field("venue", "Venue", "RustConf Arena")
    .field("date", "Date", "December 15, 2025")
    .field("seat", "Seat", "A23")
    .field("section", "Section", "Main Floor")
    .field("doors", "Doors Open", "7:00 PM") // New field
    .build();

    let updated_google: GenericObject = updated_unified.into();
    let updated_pass = client
        .update_generic_object(&created_pass.id, &updated_google)
        .await?;
    println!("✓ Pass updated");

    // Generate save URL
    let save_url = client.generate_save_url(&updated_pass).await?;
    println!("\n🎫 Add to Google Wallet:");
    println!("{}", save_url);

    println!("\n✅ Unified API demonstration complete!");
    println!("\nKey Takeaway:");
    println!("  - Use PassBuilder to create passes in a platform-agnostic way");
    println!("  - Use .into() to convert to platform-specific formats");
    println!("  - The same Pass model works for both Google and Apple Wallet");

    Ok(())
}
