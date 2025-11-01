use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::error::AzureError;

/// Represents an Azure subscription.
///
/// This struct is used to deserialize and serialize Azure subscription information.
///
/// # Fields
///
/// * `id` - The unique identifier of the subscription.
/// * `display_name` - The display name of the subscription.
#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub display_name: String,
}

/// Asynchronously lists all Azure subscriptions accessible with the provided access token.
///
/// This function sends a GET request to the Azure Resource Manager API to retrieve a list of subscriptions.
///
/// # Arguments
///
/// * `access_token` - A valid Azure access token, typically obtained via `get_access_token`.
///
/// # Returns
///
/// * `Result<Vec<Subscription>, AzureError>` - On success, returns a vector of `Subscription` objects.
///   On failure, returns an `AzureError`.
///
/// # Examples
///
/// ```rust
/// use your_crate::{list_subscriptions, get_access_token};
/// use your_crate::error::AzureError;
///
/// #[tokio::main]
/// async fn example() -> Result<(), AzureError> {
///     let access_token = get_access_token("client_id", "client_secret", "tenant_id").await?;
///     let subscriptions = list_subscriptions(&access_token).await?;
///     for sub in subscriptions {
///         println!("ID: {}, Name: {}", sub.id, sub.display_name);
///     }
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// * Returns `AzureError::ParseError` if the response cannot be parsed as a JSON array.
/// * Returns `AzureError::InvalidResponse` if the required fields (`subscriptionId` or `displayName`) are missing in the response.
/// * Returns a `reqwest::Error` if there is an issue sending the HTTP request or parsing the response.
///
/// # Notes
///
/// * This function uses the `reqwest` crate to send an authenticated GET request to the Azure API.
/// * The `api-version` is hardcoded to `2020-01-01`, which is a stable version for listing subscriptions.
/// * Ensure the provided `access_token` has the necessary permissions to list subscriptions.
pub async fn list_subscriptions(access_token: &str) -> Result<Vec<Subscription>, AzureError> {
    let subscriptions_url = "https://management.azure.com/subscriptions?api-version=2020-01-01";
    let client = Client::new();
    let response = client
        .get(subscriptions_url)
        .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
        .send()
        .await?
        .json::<Value>()
        .await?;

    let subscriptions = response["value"]
        .as_array()
        .ok_or(AzureError::ParseError)?;

    subscriptions
        .iter()
        .map(|sub| {
            Ok(Subscription {
                id: sub["subscriptionId"]
                    .as_str()
                    .ok_or(AzureError::InvalidResponse)?
                    .to_string(),
                display_name: sub["displayName"]
                    .as_str()
                    .ok_or(AzureError::InvalidResponse)?
                    .to_string(),
            })
        })
        .collect()
}
