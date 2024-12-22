use reqwest::{Client, Request, StatusCode};
use url::Url;
use crate::errors::SeqeraError;
use crate::utils::mask_auth_header;

pub mod organization;
pub mod team;
pub mod workspace;
pub mod member;
pub mod platform;
pub mod compute_env;

const DEFAULT_BASE_URL: &str = "https://api.cloud.seqera.io/";

pub struct SeqeraClient {
    client: Client,
    base_url: Url,
    token: String,
    verbose: bool,
}

impl SeqeraClient {
    pub fn new(token: String) -> Result<Self, SeqeraError> {
        Self::with_base_url(token, DEFAULT_BASE_URL)
    }

    pub fn with_base_url(token: String, base_url: &str) -> Result<Self, SeqeraError> {
        let base_url = Url::parse(base_url)?;
        Ok(Self {
            client: Client::new(),
            base_url,
            token,
            verbose: false,
        })
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn auth_header(&self) -> String {
        format!("Bearer {}", self.token)
    }

    fn request_to_curl(request: &Request) -> String {
        let mut curl = format!("curl -X {}", request.method());

        for (name, value) in request.headers() {
            curl.push_str(&format!(
                " -H '{}: {}'",
                name,
                value.to_str().unwrap_or("<binary>")
            ));
        }

        curl.push_str(&format!(" '{}'", request.url()));
        curl
    }

    async fn handle_response(&self, request: Request) -> Result<reqwest::Response, SeqeraError> {
        if self.verbose {
            self.log_request(&request);
        }

        let method = request.method().clone();
        let response = self.client.execute(request).await?;

        if response.status().is_success() {
            return Ok(response);
        }

        let status = response.status();
        let url = response.url().clone();
        let message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());

        if self.verbose {
            eprintln!("\nResponse:");
            eprintln!("  Status: {}", status);
            eprintln!("  Body: {}", message);
        }

        match status {
            StatusCode::FORBIDDEN => Err(SeqeraError::Forbidden),
            _ => Err(SeqeraError::Api {
                status,
                message,
                url: url.to_string(),
                method: method.to_string(),
            }),
        }
    }

    fn log_request(&self, request: &Request) {
        eprintln!("Request:");
        eprintln!("  Method: {}", request.method());
        eprintln!("  URL: {}", request.url());
        eprintln!("  Headers:");

        for (name, value) in request.headers() {
            let value_str = value.to_str().unwrap_or("<binary>");
            let masked_value = if name == "authorization" {
                mask_auth_header(value_str)
            } else {
                value_str.to_string()
            };
            eprintln!("    {}: {}", name, masked_value);
        }

        eprintln!("\nEquivalent curl command:");
        eprintln!("{}", Self::request_to_curl(request));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Server, ServerGuard};

    fn setup() -> (ServerGuard, SeqeraClient) {
        let server = Server::new();
        let client = SeqeraClient::with_base_url(
            "test-token".to_string(),
            &server.url(),
        ).unwrap();
        
        (server, client)
    }

    #[test]
    fn test_auth_header() {
        let (_server, client) = setup();
        assert_eq!(client.auth_header(), "Bearer test-token");
    }

    #[test]
    fn test_request_to_curl() {
        let client = Client::new();
        let request = client
            .get("https://example.com")
            .header("Authorization", "Bearer token")
            .build()
            .unwrap();

        let curl = SeqeraClient::request_to_curl(&request);
        assert!(curl.starts_with("curl -X GET"));
        assert!(curl.contains("-H 'authorization: Bearer token'"));
        assert!(curl.contains("example.com"));
    }
}

