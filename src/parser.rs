//! This module provides functionality to parse and process Azure configuration files.
//! It supports YAML-based configuration for Azure subscriptions, resource groups, and key vaults.
//! The module also includes helpers for making authenticated API calls to Azure Resource Manager.

use serde::{Deserialize, Serialize};
use serde_yaml;
use serde_json::json;
use std::fs;
use std::path::PathBuf;
use crate::rest_helper;
use crate::models::{Authentication, AzureConfig, ResourceGroup, Subscription};
use crate::subscriptions::list_subscriptions;




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
    //store the access token for later use
    let auth = Authentication {
        access_token: access_token.to_string(),
    };

    //parse yaml file into AzureConfig struct.
    let yaml_content = fs::read_to_string(file_path)?;
    println!("YAML content read from file: \n{}", yaml_content);
    let config: AzureConfig = match serde_yaml::from_str(&yaml_content) {
    Ok(c) => c,
    Err(e) => {
        println!("Failed to parse YAML: {:?}", e);
        return Err(Box::new(e));
    }
    };

    //get the list of subscriptions from Azure API
    let azure_subscriptions = list_subscriptions(&auth.access_token).await?;

    //iterate over the subscriptions in the azure config
    for subscription in &config.subscriptions {
        //check if subscription actually exists in Azure
        if !azure_subscriptions.iter().any(|sub| sub.id == subscription.id) {
            println!("Subscription ID {} not found in Azure. Skipping...", subscription.id);
            continue;
        }
        //if subscription exists, process it
        println!("Processing subscription: {:?}", subscription);
        parse_rg_call(subscription, &auth).await;
    }

    
    Ok(config)
}


async fn parse_rg_call(subscription: &Subscription, access_token: &Authentication) {
    let resource_groups = match &subscription.resource_groups {
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

    //iterate over the resourcegroups
    for ResourceGroup in resource_groups {
        println!("Processing Resource Group: {:?}", ResourceGroup);

        let endpoint = format!(
        "https://management.azure.com/subscriptions/{}/resourcegroups/{}?api-version=2021-04-01",
        subscription.id,
        ResourceGroup.name.as_deref().unwrap_or("unknown")
        );

        let body = json!({
            "location": ResourceGroup.region.as_deref().unwrap_or("eastus"),
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
    
}


