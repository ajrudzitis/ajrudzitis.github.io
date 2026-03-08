use anyhow::{Context, Result};
use reqwest::{Client, RequestBuilder};

use crate::config::{Config, BUTTONDOWN_API_BASE};
use crate::models::{
    ButtondownEmail, CreateEmailRequest, EmailListResponse, UpdateEmailRequest,
};

/// Buttondown API client
pub struct ButtondownClient {
    client: Client,
    api_key: String,
}

impl ButtondownClient {
    pub fn new(config: &Config) -> Self {
        Self {
            client: Client::new(),
            api_key: config.api_key.clone(),
        }
    }

    /// Add authorization header to a request
    fn auth(&self, request: RequestBuilder) -> RequestBuilder {
        request.header("Authorization", format!("Token {}", self.api_key))
    }

    /// Build the URL for a single email
    fn email_url(&self, id: &str) -> String {
        format!("{}/emails/{}", BUTTONDOWN_API_BASE, id)
    }

    /// Build the URL for listing emails
    fn emails_url(&self) -> String {
        format!("{}/emails", BUTTONDOWN_API_BASE)
    }

    /// List all emails from Buttondown, optionally filtered by status
    pub async fn list_emails(&self, status: Option<&str>) -> Result<Vec<ButtondownEmail>> {
        let mut all_emails = Vec::new();
        let mut url = self.emails_url();

        if let Some(s) = status {
            url.push_str(&format!("?status={}", s));
        }

        loop {
            let response: EmailListResponse = self
                .auth(self.client.get(&url))
                .send()
                .await
                .with_context(|| "Failed to send request to Buttondown API")?
                .error_for_status()
                .with_context(|| "Buttondown API returned an error")?
                .json()
                .await
                .with_context(|| "Failed to parse Buttondown API response")?;

            all_emails.extend(response.results);

            if let Some(next) = response.next {
                url = next;
            } else {
                break;
            }
        }

        Ok(all_emails)
    }

    /// Get a specific email by ID
    pub async fn get_email(&self, id: &str) -> Result<ButtondownEmail> {
        let email: ButtondownEmail = self
            .auth(self.client.get(self.email_url(id)))
            .send()
            .await
            .with_context(|| "Failed to send request to Buttondown API")?
            .error_for_status()
            .with_context(|| format!("Failed to get email {}", id))?
            .json()
            .await
            .with_context(|| "Failed to parse Buttondown API response")?;

        Ok(email)
    }

    /// Create a new email as draft
    /// SAFETY: This always sets status to "draft" to prevent accidental sends
    pub async fn create_email(&self, subject: &str, body: &str) -> Result<ButtondownEmail> {
        let request = CreateEmailRequest {
            subject: subject.to_string(),
            body: body.to_string(),
            status: "draft".to_string(), // ALWAYS draft for safety
        };

        let email: ButtondownEmail = self
            .auth(self.client.post(self.emails_url()))
            .json(&request)
            .send()
            .await
            .with_context(|| "Failed to send request to Buttondown API")?
            .error_for_status()
            .with_context(|| "Failed to create email")?
            .json()
            .await
            .with_context(|| "Failed to parse Buttondown API response")?;

        Ok(email)
    }

    /// Update an existing email
    /// SAFETY: This only updates subject and body, never status
    pub async fn update_email(&self, id: &str, subject: &str, body: &str) -> Result<ButtondownEmail> {
        let request = UpdateEmailRequest {
            subject: subject.to_string(),
            body: body.to_string(),
            // NOTE: status is intentionally excluded to prevent accidental sends
        };

        let email: ButtondownEmail = self
            .auth(self.client.patch(self.email_url(id)))
            .json(&request)
            .send()
            .await
            .with_context(|| "Failed to send request to Buttondown API")?
            .error_for_status()
            .with_context(|| format!("Failed to update email {}", id))?
            .json()
            .await
            .with_context(|| "Failed to parse Buttondown API response")?;

        Ok(email)
    }
}
