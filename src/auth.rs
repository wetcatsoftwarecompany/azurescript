use reqwest::Client;
use serde_json::Value;
use crate::error::AzureError;

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
