use crate::error::{PorterError, Result};
use crate::google::types::*;
use async_trait::async_trait;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const GOOGLE_WALLET_API_BASE: &str = "https://walletobjects.googleapis.com/walletobjects/v1";
const GOOGLE_TOKEN_URI: &str = "https://oauth2.googleapis.com/token";
const SCOPE: &str = "https://www.googleapis.com/auth/wallet_object.issuer";

/// Configuration for Google Wallet authentication
#[derive(Clone)]
pub struct GoogleWalletConfig {
    pub issuer_id: String,
    pub service_account_email: String,
    pub private_key: String,
}

/// JWT Claims for Google OAuth2
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    scope: String,
    aud: String,
    exp: u64,
    iat: u64,
}

/// Token response from Google
#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
    #[allow(dead_code)]
    token_type: String,
}

/// Google Wallet API client
pub struct GoogleWalletClient {
    config: GoogleWalletConfig,
    client: Client,
    access_token: Option<String>,
    token_expiry: Option<SystemTime>,
}

impl GoogleWalletClient {
    /// Create a new Google Wallet client
    pub fn new(config: GoogleWalletConfig) -> Self {
        Self {
            config,
            client: Client::new(),
            access_token: None,
            token_expiry: None,
        }
    }

    /// Generate a JWT for authentication
    fn generate_jwt(&self) -> Result<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| PorterError::AuthError(format!("Time error: {}", e)))?
            .as_secs();

        let claims = Claims {
            iss: self.config.service_account_email.clone(),
            scope: SCOPE.to_string(),
            aud: GOOGLE_TOKEN_URI.to_string(),
            exp: now + 3600,
            iat: now,
        };

        let key = EncodingKey::from_rsa_pem(self.config.private_key.as_bytes())?;
        let token = encode(&Header::new(Algorithm::RS256), &claims, &key)?;

        Ok(token)
    }

    /// Get an access token, refreshing if necessary
    async fn get_access_token(&mut self) -> Result<String> {
        // Check if we have a valid token
        if let (Some(token), Some(expiry)) = (&self.access_token, self.token_expiry) {
            if SystemTime::now() < expiry - Duration::from_secs(300) {
                return Ok(token.clone());
            }
        }

        // Generate new JWT
        let jwt = self.generate_jwt()?;

        // Exchange JWT for access token
        let params = [
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &jwt),
        ];

        let response = self
            .client
            .post(GOOGLE_TOKEN_URI)
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(PorterError::AuthError(format!(
                "Token exchange failed: {}",
                response.text().await?
            )));
        }

        let token_response: TokenResponse = response.json().await?;

        self.access_token = Some(token_response.access_token.clone());
        self.token_expiry =
            Some(SystemTime::now() + Duration::from_secs(token_response.expires_in));

        Ok(token_response.access_token)
    }

    /// Make an authenticated request
    async fn request<T: for<'de> Deserialize<'de>>(
        &mut self,
        method: reqwest::Method,
        path: &str,
        body: Option<&impl Serialize>,
    ) -> Result<T> {
        let token = self.get_access_token().await?;
        let url = format!("{}{}", GOOGLE_WALLET_API_BASE, path);

        let mut request = self
            .client
            .request(method, &url)
            .bearer_auth(token)
            .header("Content-Type", "application/json");

        if let Some(body) = body {
            request = request.json(body);
        }

        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            let result = response.json().await?;
            Ok(result)
        } else {
            let error_text = response.text().await?;
            Err(PorterError::ApiError {
                status: status.as_u16(),
                message: error_text,
            })
        }
    }

    /// Create a generic class
    pub async fn create_generic_class(&mut self, class: &GenericClass) -> Result<GenericClass> {
        self.request(reqwest::Method::POST, "/genericClass", Some(class))
            .await
    }

    /// Get a generic class
    pub async fn get_generic_class(&mut self, class_id: &str) -> Result<GenericClass> {
        self.request(
            reqwest::Method::GET,
            &format!("/genericClass/{}", class_id),
            None::<&()>,
        )
        .await
    }

    /// Update a generic class
    pub async fn update_generic_class(
        &mut self,
        class_id: &str,
        class: &GenericClass,
    ) -> Result<GenericClass> {
        self.request(
            reqwest::Method::PUT,
            &format!("/genericClass/{}", class_id),
            Some(class),
        )
        .await
    }

    /// Create a generic object (pass)
    pub async fn create_generic_object(&mut self, object: &GenericObject) -> Result<GenericObject> {
        self.request(reqwest::Method::POST, "/genericObject", Some(object))
            .await
    }

    /// Get a generic object
    pub async fn get_generic_object(&mut self, object_id: &str) -> Result<GenericObject> {
        self.request(
            reqwest::Method::GET,
            &format!("/genericObject/{}", object_id),
            None::<&()>,
        )
        .await
    }

    /// Update a generic object
    pub async fn update_generic_object(
        &mut self,
        object_id: &str,
        object: &GenericObject,
    ) -> Result<GenericObject> {
        self.request(
            reqwest::Method::PUT,
            &format!("/genericObject/{}", object_id),
            Some(object),
        )
        .await
    }

    /// Patch a generic object (partial update)
    pub async fn patch_generic_object(
        &mut self,
        object_id: &str,
        object: &GenericObject,
    ) -> Result<GenericObject> {
        self.request(
            reqwest::Method::PATCH,
            &format!("/genericObject/{}", object_id),
            Some(object),
        )
        .await
    }

    /// List generic objects
    pub async fn list_generic_objects(
        &mut self,
        class_id: Option<&str>,
    ) -> Result<GenericObjectListResponse> {
        let path = if let Some(class_id) = class_id {
            format!("/genericObject?classId={}", class_id)
        } else {
            "/genericObject".to_string()
        };

        self.request(reqwest::Method::GET, &path, None::<&()>).await
    }

    /// Add a message to a generic object
    pub async fn add_message_to_object(
        &mut self,
        object_id: &str,
        message: &AddMessageRequest,
    ) -> Result<GenericObject> {
        self.request(
            reqwest::Method::POST,
            &format!("/genericObject/{}/addMessage", object_id),
            Some(message),
        )
        .await
    }

    /// Create an event ticket object
    pub async fn create_event_ticket(
        &mut self,
        ticket: &EventTicketObject,
    ) -> Result<EventTicketObject> {
        self.request(reqwest::Method::POST, "/eventTicketObject", Some(ticket))
            .await
    }

    /// Get an event ticket object
    pub async fn get_event_ticket(&mut self, object_id: &str) -> Result<EventTicketObject> {
        self.request(
            reqwest::Method::GET,
            &format!("/eventTicketObject/{}", object_id),
            None::<&()>,
        )
        .await
    }

    /// Update an event ticket object
    pub async fn update_event_ticket(
        &mut self,
        object_id: &str,
        ticket: &EventTicketObject,
    ) -> Result<EventTicketObject> {
        self.request(
            reqwest::Method::PUT,
            &format!("/eventTicketObject/{}", object_id),
            Some(ticket),
        )
        .await
    }

    /// Create a loyalty object
    pub async fn create_loyalty_object(
        &mut self,
        loyalty: &LoyaltyObject,
    ) -> Result<LoyaltyObject> {
        self.request(reqwest::Method::POST, "/loyaltyObject", Some(loyalty))
            .await
    }

    /// Get a loyalty object
    pub async fn get_loyalty_object(&mut self, object_id: &str) -> Result<LoyaltyObject> {
        self.request(
            reqwest::Method::GET,
            &format!("/loyaltyObject/{}", object_id),
            None::<&()>,
        )
        .await
    }

    /// Update a loyalty object
    pub async fn update_loyalty_object(
        &mut self,
        object_id: &str,
        loyalty: &LoyaltyObject,
    ) -> Result<LoyaltyObject> {
        self.request(
            reqwest::Method::PUT,
            &format!("/loyaltyObject/{}", object_id),
            Some(loyalty),
        )
        .await
    }

    /// Generate a JWT for a pass object
    fn generate_pass_jwt(&self, objects: &[GenericObject]) -> Result<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| PorterError::AuthError(format!("Time error: {}", e)))?
            .as_secs() as i64;

        let payload = JwtPayload {
            iss: self.config.service_account_email.clone(),
            aud: "google".to_string(),
            typ: "savetowallet".to_string(),
            iat: now,
            origins: None,
            payload: JwtObjectPayload {
                generic_objects: Some(objects.to_vec()),
                event_ticket_objects: None,
                loyalty_objects: None,
            },
        };

        let key = EncodingKey::from_rsa_pem(self.config.private_key.as_bytes())?;
        let token = encode(&Header::new(Algorithm::RS256), &payload, &key)?;

        Ok(token)
    }

    /// Generate a save URL for a generic pass object
    ///
    /// This creates a JWT and calls the Google Wallet API to get a save URL
    /// that can be used to add the pass to a user's wallet.
    pub async fn generate_save_url(&mut self, object: &GenericObject) -> Result<String> {
        let jwt = self.generate_pass_jwt(std::slice::from_ref(object))?;

        let jwt_resource = JwtResource { jwt };

        let response: JwtInsertResponse = self
            .request(reqwest::Method::POST, "/jwt", Some(&jwt_resource))
            .await?;

        response.save_uri.ok_or_else(|| PorterError::ApiError {
            status: 500,
            message: "No save URI returned from API".to_string(),
        })
    }
}

/// Trait for pass operations (can be implemented for other platforms)
#[async_trait]
pub trait PassClient {
    async fn create_pass(&mut self, pass: &GenericObject) -> Result<GenericObject>;
    async fn get_pass(&mut self, pass_id: &str) -> Result<GenericObject>;
    async fn update_pass(&mut self, pass_id: &str, pass: &GenericObject) -> Result<GenericObject>;
    async fn delete_pass(&mut self, pass_id: &str) -> Result<()>;
}

#[async_trait]
impl PassClient for GoogleWalletClient {
    async fn create_pass(&mut self, pass: &GenericObject) -> Result<GenericObject> {
        self.create_generic_object(pass).await
    }

    async fn get_pass(&mut self, pass_id: &str) -> Result<GenericObject> {
        self.get_generic_object(pass_id).await
    }

    async fn update_pass(&mut self, pass_id: &str, pass: &GenericObject) -> Result<GenericObject> {
        self.update_generic_object(pass_id, pass).await
    }

    async fn delete_pass(&mut self, pass_id: &str) -> Result<()> {
        // Google Wallet doesn't support deletion, so we'll mark as expired instead
        let mut pass = self.get_generic_object(pass_id).await?;
        pass.state = Some("EXPIRED".to_string());
        self.update_generic_object(pass_id, &pass).await?;
        Ok(())
    }
}
