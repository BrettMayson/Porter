# Quick Start Guide

This guide will help you get started with Porter for managing Google Wallet passes.

## Prerequisites

- Rust 1.70 or later
- A Google Cloud project with the Google Wallet API enabled
- A service account with appropriate permissions

## Step 1: Set up Google Cloud

1. Go to the [Google Cloud Console](https://console.cloud.google.com)
2. Create a new project or select an existing one
3. Enable the Google Wallet API for your project
4. Create a service account:
   - Go to IAM & Admin > Service Accounts
   - Click "Create Service Account"
   - Give it a name and description
   - Grant it the "Google Wallet API Issuer" role
5. Create a key for the service account:
   - Click on the service account
   - Go to "Keys" tab
   - Click "Add Key" > "Create new key"
   - Choose JSON format
   - Download the key file

## Step 2: Extract Private Key

From the downloaded JSON file, extract the private key:

```bash
# Extract the private_key field from the JSON file
jq -r '.private_key' path/to/service-account-key.json > service-account-key.pem
```

Or manually copy the `private_key` value (including the `-----BEGIN PRIVATE KEY-----` and `-----END PRIVATE KEY-----` lines) to a new file.

## Step 3: Get Your Issuer ID

1. Go to the [Google Wallet Business Console](https://pay.google.com/business/console)
2. Your issuer ID is shown in the console (a long number like `1234567890123456789`)

## Step 4: Add Porter to Your Project

```toml
[dependencies]
porter = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Step 5: Create Your First Pass

```rust
use porter::google::{GoogleWalletClient, GoogleWalletConfig, GenericClass, GenericObject};

#[tokio::main]
async fn main() -> porter::error::Result<()> {
    // Configure the client
    let config = GoogleWalletConfig {
        issuer_id: "1234567890123456789".to_string(),
        service_account_email: "your-service-account@project.iam.gserviceaccount.com".to_string(),
        private_key: std::fs::read_to_string("service-account-key.pem")?,
    };

    let mut client = GoogleWalletClient::new(config.clone());

    // Create a class (do this once per type of pass)
    let class = GenericClass {
        id: format!("{}.my_first_class", config.issuer_id),
        issuer_name: Some("My Company".to_string()),
        review_status: Some("UNDER_REVIEW".to_string()),
    };

    client.create_generic_class(&class).await?;
    println!("✓ Class created");

    // Create a pass
    let pass = GenericObject {
        id: format!("{}.my_first_pass", config.issuer_id),
        class_id: class.id.clone(),
        state: Some("ACTIVE".to_string()),
        ..Default::default()
    };

    let created = client.create_generic_object(&pass).await?;
    println!("✓ Pass created: {}", created.id);

    // Generate a save URL
    let url = client.generate_save_url(&created.id);
    println!("Add to Google Wallet: {}", url);

    Ok(())
}
```

## Step 6: Test Your Pass

Run your program:

```bash
cargo run
```

Open the generated URL in a browser to add the pass to your Google Wallet!

## Next Steps

- Check out the [examples](examples/) directory for more advanced usage
- Read the [API documentation](https://docs.rs/porter)
- See the [README](README.md) for detailed API reference

## Troubleshooting

### Authentication Errors

If you see authentication errors:
- Verify your service account email is correct
- Ensure the private key file is readable and contains the full key
- Check that the Google Wallet API is enabled for your project
- Verify the service account has the "Google Wallet API Issuer" role

### API Errors

If you see API errors:
- Make sure your issuer ID is correct
- Verify the class exists before creating objects
- Check that IDs follow the format `issuerId.identifier`
- Ensure IDs only contain alphanumeric characters, '.', '_', or '-'

### Pass Not Appearing

If the pass doesn't appear in Google Wallet:
- Make sure the save URL is opened in a browser where you're signed in to Google
- Check that the pass state is "ACTIVE"
- Verify all required fields are set

## Environment Variables

For convenience, you can use environment variables:

```bash
export GOOGLE_WALLET_ISSUER_ID="1234567890123456789"
export GOOGLE_WALLET_SERVICE_ACCOUNT="your-service-account@project.iam.gserviceaccount.com"
```

Then in your code:

```rust
let config = GoogleWalletConfig {
    issuer_id: std::env::var("GOOGLE_WALLET_ISSUER_ID")?,
    service_account_email: std::env::var("GOOGLE_WALLET_SERVICE_ACCOUNT")?,
    private_key: std::fs::read_to_string("service-account-key.pem")?,
};
```

## Security Notes

- **Never commit your private key or service account JSON to version control**
- Store credentials securely (use environment variables or secret management)
- The `.gitignore` file is configured to exclude common credential files
- In production, use proper secret management (AWS Secrets Manager, Google Secret Manager, etc.)
