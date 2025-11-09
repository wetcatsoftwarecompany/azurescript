//! This module provides an Azure authentication helper.

use reqwest::Client;
use serde_json::Value;
use crate::error::AzureError;

/// Asynchronously retrieves an access token from Azure Active Directory using the OAuth 2.0 client credentials flow.
///
/// This function is used to authenticate with Azure services by obtaining an access token for the specified client ID, client secret, and tenant ID.
///
/// # Arguments
///
/// * `client_id` - The application (client) ID registered in Azure Active Directory.
/// * `client_secret` - The client secret associated with the application.
/// * `tenant_id` - The Azure Active Directory tenant ID.
///
/// # Returns
///
/// * `Result<String, AzureError>` - On success, returns the access token as a `String`. On failure, returns an `AzureError`.
///
/// # Examples
///
/// ```rust
/// use your_crate::get_access_token;
/// use your_crate::error::AzureError;
///
/// #[tokio::main]
/// async fn example() -> Result<(), AzureError> {
///     let client_id = "your_client_id";
///     let client_secret = "your_client_secret";
///     let tenant_id = "your_tenant_id";
///
///     let token = get_access_token(client_id, client_secret, tenant_id).await?;
///     println!("Access token: {}", token);
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// * Returns `AzureError::TokenError` if the access token is not found in the response or if the request fails.
/// * Returns a `reqwest::Error` if there is an issue sending the HTTP request or parsing the response.
///
/// # Notes
///
/// * This function uses the `request` crate to send an HTTP POST request to Azure's OAuth 2.0 token endpoint.
/// * The `scope` is hardcoded to `https://management.azure.com/.default`, which is suitable for Azure Resource Manager APIs.
/// * Ensure that the client ID and client secret are kept secure and not hardcoded in your application.
pub async fn get_access_token(
    client_id: &str,
    client_secret: &str,
    tenant_id: &str,
) -> Result<String, AzureError> {
    let token_url = format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
        tenant_id
    );
    let params = [
        ("client_id", client_id),
        ("scope", "https://management.azure.com/.default"),
        ("client_secret", client_secret),
        ("grant_type", "client_credentials"),
    ];

    let client = Client::new();
    let response = client
        .post(&token_url)
        .form(&params)
        .send()
        .await?
        .json::<Value>()
        .await?;

    response["access_token"]
        .as_str()
        .ok_or(AzureError::TokenError)
        .map(|s| s.to_string())
}
