use std::path::PathBuf;
use crate::models::{AzureConfig, Authentication};
use std::fs;

pub async fn retrieve_az_resources(file_path: &PathBuf, access_token: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    print!("Retrieving current Azure state...\n");
    //store the access token for later use
    let auth = Authentication {
        access_token: access_token.to_string(),
    };
    //read yaml file and parse into AzureConfig struct.
    let yaml_content = fs::read_to_string(file_path)?;
    println!("YAML content read from file: \n{}", yaml_content);
    let config: AzureConfig = match serde_yaml::from_str(&yaml_content) {
    Ok(c) => c,
    Err(e) => {
        println!("Failed to parse YAML: {:?}", e);
        return Err(Box::new(e));
    }
    };
    
    Ok(())
}