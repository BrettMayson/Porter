//! Porter - A unified API for managing Google Wallet and Apple Wallet passes
//!
//! This crate provides:
//! - A unified, platform-agnostic data model for wallet passes
//! - A fluent builder API (`PassBuilder`) for creating passes
//! - Automatic conversion between unified and platform-specific types
//! - Platform-specific clients (Google Wallet implemented, Apple Wallet coming soon)
//! - Authentication handling for Google Wallet API
//! - CRUD operations for passes
//!
//! # Philosophy
//!
//! Porter uses a **unified model** approach: define your pass once using platform-agnostic
//! types, then convert to the specific platform format you need. This means:
//!
//! - Write pass creation code once, use it for multiple platforms
//! - Use `.into()` for automatic conversion to platform-specific types
//! - Platform-specific features still available when needed
//!
//! # Examples
//!
//! ## Creating a pass with the unified API
//!
//! ```
//! use porter::PassBuilder;
//! use porter::models::{PassType, BarcodeFormat};
//!
//! // Create a platform-agnostic pass
//! let pass = PassBuilder::new("issuer.pass001", "issuer.class001")
//!     .pass_type(PassType::EventTicket)
//!     .title("Concert Ticket")
//!     .subtitle("The Rust Band")
//!     .background_color("#4285F4")
//!     .barcode_with_text(BarcodeFormat::QrCode, "TICKET123", "TICKET123")
//!     .field("seat", "Seat", "A23")
//!     .build();
//!
//! // Convert to Google Wallet format
//! use porter::google::GenericObject;
//! let google_pass: GenericObject = pass.into();
//! ```
//!
//! ## Using with Google Wallet
//!
//! ```no_run
//! use porter::PassBuilder;
//! use porter::models::{PassType, BarcodeFormat};
//! use porter::google::{GoogleWalletClient, GoogleWalletConfig, GenericObject};
//!
//! # async fn example() -> porter::error::Result<()> {
//! let config = GoogleWalletConfig {
//!     issuer_id: "your_issuer_id".to_string(),
//!     service_account_email: "your-service-account@project.iam.gserviceaccount.com".to_string(),
//!     private_key: "-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----".to_string(),
//! };
//!
//! let mut client = GoogleWalletClient::new(config);
//!
//! // Build pass with unified API
//! let pass = PassBuilder::new("issuer.pass001", "issuer.class001")
//!     .title("My Pass")
//!     .build();
//!
//! // Convert and create
//! let google_pass: GenericObject = pass.into();
//! let created = client.create_generic_object(&google_pass).await?;
//! # Ok(())
//! # }
//! ```

pub mod builder;
pub mod error;
pub mod models;
pub mod google;
pub mod apple;

// Re-export commonly used types
pub use builder::PassBuilder;
pub use error::{PorterError, Result};
pub use models::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_types() {
        // Verify our pass types compile
        let pass_type = PassType::Generic;
        assert_eq!(pass_type, PassType::Generic);
    }
}
