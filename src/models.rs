use serde::{Deserialize, Serialize};

/// Represents an Azure authentication token.

pub struct Authentication {
    pub access_token: String
}

/// Represents an Azure Key Vault.
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyVault {
    pub name: Option<String>,
    pub region: Option<String>,
}

/// Represents an Azure Resource Group.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceGroup {
    pub name: Option<String>,
    pub region: Option<String>,
    pub keyvaults: Option<Vec<KeyVault>>,
}

/// Represents an Azure Subscription.
#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub name: String,
    pub id: String,
    pub resource_groups: Option<Vec<ResourceGroup>>,
}

/// Represents the overall Azure configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct AzureConfig {
    pub subscriptions: Vec<Subscription>,
}