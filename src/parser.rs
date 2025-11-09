//! This module provides functionality to parse and process Azure configuration files.
//! It supports YAML-based configuration for Azure subscriptions, resource groups, and key vaults.
//! The module also includes helpers for making authenticated API calls to Azure Resource Manager.

use serde::{Deserialize, Serialize};
use serde_yaml;
use serde_json::json;
use std::fs;
use std::path::PathBuf;
use crate::rest_helper;


/// Represents an Azure authentication token.
struct Authentication {
    access_token: String
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

/// Parses an Azure configuration file from a YAML file.
///
/// # Arguments
/// * `file_path` - The path to the YAML configuration file.
/// * `access_token` - The Azure access token for API authentication.
///
/// # Returns
/// * `Result<AzureConfig, Box<dyn std::error::Error>>` - The parsed configuration or an error.
///
/// # Example
/// ```rust, ignore
/// use std::path::PathBuf;
/// let file_path = PathBuf::from("config.yaml");
/// let access_token = "your_access_token";
/// let config = parse(&file_path, access_token).await?;
/// ```
pub async fn parse(file_path: &PathBuf, access_token: &str) -> Result<AzureConfig, Box<dyn std::error::Error>> {
    print!("Starting to parse the Azure configuration file...\n");
    let auth = Authentication {
        access_token: access_token.to_string(),
    };
    let yaml_content = fs::read_to_string(file_path)?;
    println!("YAML content read from file: \n{}", yaml_content);
    let config: AzureConfig = match serde_yaml::from_str(&yaml_content) {
    Ok(c) => c,
    Err(e) => {
        println!("Failed to parse YAML: {:?}", e);
        return Err(Box::new(e));
    }
    };
    let input_subscription: &Subscription;
    if let Some(first_subscription) = config.subscriptions.first() {
        println!("First subscription: {:?}", first_subscription);
        input_subscription = first_subscription;
    } else {   
        println!("The vector is empty!");
        return Err("No subscriptions found".into());
    }
    println!("input_subscription");
    parse_rg_call(&input_subscription, &auth).await;
    Ok(config)
}


async fn parse_rg_call(config: &Subscription, access_token: &Authentication) {
    let resource_groups = match &config.resource_groups {
        Some(rg) if !rg.is_empty() => rg,
        Some(_) => {
            println!("The resource_groups vector is empty!");
            return;
        }
        None => {
            println!("No resource_groups found!");
            return;
        }
    };

    let first_rg = &resource_groups[0];
    println!("First RG: {:?}", first_rg);

    let endpoint = format!(
        "https://management.azure.com/subscriptions/{}/resourcegroups/{}?api-version=2021-04-01",
        config.id,
        first_rg.name.as_deref().unwrap_or("unknown")
    );

    let body = json!({
        "location": first_rg.region.as_deref().unwrap_or("eastus"),
    });

    println!("Endpoint: {}", endpoint);
    println!("Body: {}", body);

    match rest_helper::call_azure_api(&endpoint, &access_token.access_token, &body).await {
        Ok(response) => {
            println!("API call successful. Response: {:?}", response);
        }
        Err(e) => {
            println!("API call failed. Error: {:?}", e);
        }
    }
}


