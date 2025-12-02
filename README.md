# Porter

A Rust crate for managing Google Wallet and Apple Wallet passes with a unified API.

## Features

- 🎫 **Unified Pass Model**: Platform-agnostic `Pass` and `PassBuilder` API that works across both platforms
- 🔄 **Automatic Conversion**: Use `.into()` to convert between unified models and platform-specific types
- 🔐 **Authentication**: Built-in OAuth2 authentication for Google Wallet
- 📱 **Multiple Pass Types**: Support for event tickets, loyalty cards, gift cards, generic passes, and more
- ⚡ **Async/Await**: Full async support using Tokio
- 🔧 **Builder Pattern**: Fluent API for constructing passes with compile-time safety
- 📊 **Type Safety**: Strongly typed Rust structs for all pass data

## Current Status

- ✅ **Google Wallet**: Full implementation with authentication and CRUD operations
- 🚧 **Apple Wallet**: Stub implementation (coming soon)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
porter = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Usage

### Unified API (Recommended)

Porter provides a platform-agnostic way to create passes that can be used with any wallet platform:

```rust
use porter::PassBuilder;
use porter::models::{PassType, BarcodeFormat};

// Create a pass using the unified builder - works for any platform!
let pass = PassBuilder::new("issuer.pass001", "issuer.class001")
    .pass_type(PassType::EventTicket)
    .title("Concert Ticket")
    .subtitle("The Rust Band Live")
    .background_color("#4285F4")
    .barcode_with_text(BarcodeFormat::QrCode, "TICKET123", "TICKET123")
    .field("seat", "Seat", "A23")
    .field("section", "Section", "Main Floor")
    .build();

// Convert to Google Wallet format
use porter::google::GenericObject;
let google_pass: GenericObject = pass.into();

// Or in the future, convert to Apple Wallet format
// let apple_pass: ApplePass = pass.into();
```

### Google Wallet

#### Setup

First, you'll need to set up a Google Cloud project and create a service account with the Google Wallet API enabled. Then, download the service account JSON key.

```rust
use porter::google::{GoogleWalletClient, GoogleWalletConfig, GenericObject, GenericClass};
use porter::PassBuilder;

#[tokio::main]
async fn main() -> porter::error::Result<()> {
    // Configure the client
    let config = GoogleWalletConfig {
        issuer_id: "1234567890123456789".to_string(),
        service_account_email: "your-service-account@project.iam.gserviceaccount.com".to_string(),
        private_key: std::fs::read_to_string("private-key.pem")?,
    };

    let mut client = GoogleWalletClient::new(config);

    Ok(())
}
```

#### Creating a Class

Classes are templates for your passes. You need to create a class before creating passes.

```rust
use porter::google::GenericClass;

// Create a class (template)
let class = GenericClass {
    id: format!("{}.my_class", config.issuer_id),
    issuer_name: Some("My Company".to_string()),
    review_status: Some("UNDER_REVIEW".to_string()),
};

let created_class = client.create_generic_class(&class).await?;
println!("Created class: {:?}", created_class);
```

#### Creating a Pass (Unified Way)

```rust
use porter::PassBuilder;
use porter::models::{PassType, BarcodeFormat};
use porter::google::GenericObject;

// Create using the unified builder
let pass = PassBuilder::new(
    format!("{}.my_pass_001", config.issuer_id),
    format!("{}.my_class", config.issuer_id)
)
.pass_type(PassType::Generic)
.title("My Card")
.subtitle("Welcome!")
.background_color("#4285F4")
.barcode_with_text(BarcodeFormat::QrCode, "123456789", "123456789")
.build();

// Convert to Google Wallet format
let google_pass: GenericObject = pass.into();

// Create in Google Wallet
let created_pass = client.create_generic_object(&google_pass).await?;
println!("Created pass: {:?}", created_pass);
```

#### Creating a Pass (Google-Specific Way)

You can also use Google-specific types directly if you need platform-specific features:

```rust
use porter::google::{GenericObject, Barcode, LocalizedString, TranslatedString};

let pass = GenericObject {
    id: format!("{}.my_pass_001", config.issuer_id),
    class_id: format!("{}.my_class", config.issuer_id),
    state: Some("ACTIVE".to_string()),
    card_title: Some(LocalizedString {
        default_value: Some(TranslatedString {
            language: "en-US".to_string(),
            value: "My Card".to_string(),
        }),
        translated_values: None,
    }),
    hex_background_color: Some("#4285F4".to_string()),
    ..Default::default()
};

let created_pass = client.create_generic_object(&pass).await?;
```

#### Updating a Pass

```rust
// Update the pass
let mut updated_pass = created_pass.clone();
updated_pass.state = Some("COMPLETED".to_string());

let result = client.update_generic_object(&pass.id, &updated_pass).await?;
println!("Updated pass: {:?}", result);
```

#### Getting a Pass

```rust
let pass = client.get_generic_object("issuer_id.pass_001").await?;
println!("Retrieved pass: {:?}", pass);
```

#### Listing Passes

```rust
let list = client.list_generic_objects(Some("issuer_id.my_class")).await?;
println!("Found {} passes", list.resources.as_ref().map(|r| r.len()).unwrap_or(0));
```

#### Adding a Message

```rust
use porter::google::{AddMessageRequest, Message, TimeInterval, DateTime};

let message = AddMessageRequest {
    message: Message {
        header: Some("Important Update".to_string()),
        body: Some("Your pass has been updated!".to_string()),
        display_interval: None,
    },
};

client.add_message_to_object(&pass.id, &message).await?;
```

#### Generating Save URLs

```rust
let save_url = client.generate_save_url(&pass.id);
println!("Add to Google Wallet: {}", save_url);
```

### Event Tickets

```rust
use porter::google::{EventTicketObject, EventSeat, LocalizedString, TranslatedString};

let ticket = EventTicketObject {
    id: format!("{}.ticket_001", config.issuer_id),
    class_id: format!("{}.event_class", config.issuer_id),
    state: Some("ACTIVE".to_string()),
    ticket_holder_name: Some("John Doe".to_string()),
    seat_info: Some(EventSeat {
        seat: Some(LocalizedString {
            default_value: Some(TranslatedString {
                language: "en-US".to_string(),
                value: "A23".to_string(),
            }),
            translated_values: None,
        }),
        row: Some(LocalizedString {
            default_value: Some(TranslatedString {
                language: "en-US".to_string(),
                value: "A".to_string(),
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
        value: "TICKET123456".to_string(),
        alternate_text: Some("TICKET123456".to_string()),
    }),
};

let created_ticket = client.create_event_ticket(&ticket).await?;
```

### Loyalty Cards

```rust
use porter::google::{LoyaltyObject, LoyaltyPoints, LoyaltyPointsBalance};

let loyalty = LoyaltyObject {
    id: format!("{}.loyalty_001", config.issuer_id),
    class_id: format!("{}.loyalty_class", config.issuer_id),
    state: Some("ACTIVE".to_string()),
    account_id: Some("USER123".to_string()),
    account_name: Some("John Doe".to_string()),
    loyalty_points: Some(LoyaltyPoints {
        label: "Points".to_string(),
        balance: Some(LoyaltyPointsBalance {
            string: None,
            int: Some(1500),
            double: None,
        }),
    }),
    barcode: Some(Barcode {
        barcode_type: "CODE_128".to_string(),
        value: "USER123".to_string(),
        alternate_text: Some("USER123".to_string()),
    }),
};

let created_loyalty = client.create_loyalty_object(&loyalty).await?;
```

## Unified Models

Porter provides platform-agnostic models that can be converted to platform-specific formats:

```rust
use porter::models::{Pass, PassType, PassHeader, Barcode, BarcodeFormat};

let pass = Pass {
    id: "my_pass_001".to_string(),
    class_id: "my_class".to_string(),
    pass_type: PassType::Generic,
    header: PassHeader {
        title: "My Pass".to_string(),
        subtitle: Some("Subtitle".to_string()),
        logo: None,
        background_color: Some("#4285F4".to_string()),
        foreground_color: Some("#FFFFFF".to_string()),
    },
    barcode: Some(Barcode {
        format: BarcodeFormat::QrCode,
        value: "123456789".to_string(),
        alternate_text: Some("123456789".to_string()),
    }),
    fields: vec![],
    linked_objects: vec![],
    state: porter::models::PassState::Active,
    valid_time_interval: None,
    updated_at: None,
};
```

## Error Handling

Porter uses the `thiserror` crate for error handling:

```rust
use porter::error::{PorterError, Result};

async fn create_pass() -> Result<()> {
    // Your code here
    Ok(())
}

// Handle specific errors
match create_pass().await {
    Ok(_) => println!("Success!"),
    Err(PorterError::AuthError(msg)) => eprintln!("Authentication failed: {}", msg),
    Err(PorterError::ApiError { status, message }) => {
        eprintln!("API error {}: {}", status, message)
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Authentication

### Google Wallet

Google Wallet uses OAuth2 with service account authentication. The client automatically:
- Generates JWTs for authentication
- Exchanges JWTs for access tokens
- Refreshes tokens when they expire
- Includes authentication headers in all requests

## API Reference

### Google Wallet Client Methods

- `create_generic_class(class)` - Create a new pass class
- `get_generic_class(class_id)` - Get a class by ID
- `update_generic_class(class_id, class)` - Update a class
- `create_generic_object(object)` - Create a new pass
- `get_generic_object(object_id)` - Get a pass by ID
- `update_generic_object(object_id, object)` - Update a pass
- `patch_generic_object(object_id, object)` - Partially update a pass
- `list_generic_objects(class_id)` - List passes, optionally filtered by class
- `add_message_to_object(object_id, message)` - Send a message to pass holders
- `create_event_ticket(ticket)` - Create an event ticket
- `create_loyalty_object(loyalty)` - Create a loyalty card
- `generate_save_url(object_id)` - Generate an "Add to Google Wallet" URL

## Requirements

- Rust 1.70 or later
- Tokio runtime
- Google Cloud project with Google Wallet API enabled (for Google Wallet)
- Service account with appropriate permissions (for Google Wallet)

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Roadmap

- [x] Google Wallet API implementation
- [x] Authentication handling
- [x] CRUD operations for passes
- [x] Event tickets, loyalty cards, and other pass types
- [ ] Apple Wallet PKPass generation
- [ ] Pass conversion between platforms
- [ ] CLI tool for pass management
- [ ] Web service for pass distribution

## Examples

See the `examples/` directory for more complete examples.
