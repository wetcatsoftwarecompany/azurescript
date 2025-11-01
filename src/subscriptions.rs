use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::error::AzureError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub display_name: String,
}

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
