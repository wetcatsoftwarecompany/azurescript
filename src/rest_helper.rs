//! This module provides azure rest api helpers.
use reqwest::{Client, header};


/// Makes an authenticated HTTP PUT request to an Azure REST API endpoint.
///
/// # Arguments
/// * `endpoint` - The full URL of the Azure REST API endpoint.
/// * `access_token` - The Azure OAuth2 access token for authentication.
/// * `body` - The request body as a `serde_json::Value`.
///
/// # Returns
/// * `Result<serde_json::Value, reqwest::Error>` - The JSON response from the API or an error.
///
/// # Errors
/// This function will return an error if:
/// - The HTTP request fails (e.g., network issues, invalid endpoint).
/// - The response cannot be parsed as JSON.
/// - The Azure API returns an error status code.
///
/// # Example
/// ```rust,ignore
/// use serde_json::json;
/// let endpoint = "https://management.azure.com/...";
/// let access_token = "your_access_token";
/// let body = json!({ "location": "eastus" });
/// let response = call_azure_api(endpoint, access_token, &body).await?;
/// ```
/// 
pub async fn call_azure_api(endpoint: &str, access_token: &str, body: &serde_json::Value) -> Result<serde_json::Value, reqwest::Error> {
    let client = Client::new();
    let response = client
        .put(endpoint)
        .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
        .header(header::CONTENT_TYPE, "application/json")
        .json(body)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("API Response: {:?}", response);
    Ok(response)
}