use porter::google::{
    AddMessageRequest, Barcode, GenericClass, GenericObject, GoogleWalletClient,
    GoogleWalletConfig, LocalizedString, Message, TranslatedString,
};
use porter::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration from environment or file
    let config = GoogleWalletConfig {
        issuer_id: std::env::var("GOOGLE_WALLET_ISSUER_ID")
            .expect("GOOGLE_WALLET_ISSUER_ID must be set"),
        service_account_email: std::env::var("GOOGLE_WALLET_SERVICE_ACCOUNT")
            .expect("GOOGLE_WALLET_SERVICE_ACCOUNT must be set"),
        private_key: std::fs::read_to_string("service-account-key.pem")
            .expect("Failed to read private key"),
    };

    let mut client = GoogleWalletClient::new(config.clone());

    // Step 1: Create a class (template)
    println!("Creating a class...");
    let class_id = format!("{}.example_class", config.issuer_id);
    let class = GenericClass {
        id: class_id.clone(),
        issuer_name: Some("Porter Example".to_string()),
        review_status: Some("UNDER_REVIEW".to_string()),
    };

    match client.create_generic_class(&class).await {
        Ok(created_class) => println!("✓ Created class: {}", created_class.id),
        Err(e) => println!("Class may already exist: {}", e),
    }

    // Step 2: Create a pass (object)
    println!("\nCreating a pass...");
    let pass_id = format!("{}.example_pass_001", config.issuer_id);
    let pass = GenericObject {
        id: pass_id.clone(),
        class_id: class_id.clone(),
        state: Some("ACTIVE".to_string()),
        card_title: Some(LocalizedString {
            default_value: Some(TranslatedString {
                language: "en-US".to_string(),
                value: "Example Pass".to_string(),
            }),
            translated_values: None,
        }),
        header: Some(LocalizedString {
            default_value: Some(TranslatedString {
                language: "en-US".to_string(),
                value: "Welcome to Porter!".to_string(),
            }),
            translated_values: None,
        }),
        subheader: Some(LocalizedString {
            default_value: Some(TranslatedString {
                language: "en-US".to_string(),
                value: "Your example pass".to_string(),
            }),
            translated_values: None,
        }),
        barcode: Some(Barcode {
            barcode_type: "QR_CODE".to_string(),
            value: "EXAMPLE123456".to_string(),
            alternate_text: Some("EXAMPLE123456".to_string()),
        }),
        hex_background_color: Some("#4285F4".to_string()),
        ..Default::default()
    };

    let created_pass = client.create_generic_object(&pass).await?;
    println!("✓ Created pass: {}", created_pass.id);

    // Step 3: Get the pass
    println!("\nRetrieving pass...");
    let retrieved_pass = client.get_generic_object(&pass_id).await?;
    println!("✓ Retrieved pass: {}", retrieved_pass.id);
    println!("  State: {}", retrieved_pass.state.as_deref().unwrap_or("UNKNOWN"));

    // Step 4: Update the pass
    println!("\nUpdating pass...");
    let mut updated_pass = retrieved_pass.clone();
    updated_pass.subheader = Some(LocalizedString {
        default_value: Some(TranslatedString {
            language: "en-US".to_string(),
            value: "Updated subheader".to_string(),
        }),
        translated_values: None,
    });
    
    let result = client.update_generic_object(&pass_id, &updated_pass).await?;
    println!("✓ Updated pass: {}", result.id);

    // Step 5: Add a message
    println!("\nAdding message to pass...");
    let message = AddMessageRequest {
        message: Message {
            header: Some("Important Update".to_string()),
            body: Some("This is an example message sent to your pass!".to_string()),
            display_interval: None,
        },
    };

    client.add_message_to_object(&pass_id, &message).await?;
    println!("✓ Message added");

    // Step 6: List passes
    println!("\nListing passes for class...");
    let list = client.list_generic_objects(Some(&class_id)).await?;
    let count = list
        .resources
        .as_ref()
        .map(|r| r.len())
        .unwrap_or(0);
    println!("✓ Found {} passes", count);

    // Step 7: Generate save URL
    let save_url = client.generate_save_url(&pass_id);
    println!("\n📱 Add to Google Wallet:");
    println!("{}", save_url);

    println!("\n✅ Example completed successfully!");
    
    Ok(())
}
