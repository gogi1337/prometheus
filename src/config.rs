use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct ProxyConfig {
    pub link: String,
    pub port: u16,
}

pub fn read_config(path: &str) -> Result<Vec<ProxyConfig>, String> {
    let contents = std::fs::read_to_string(path).map_err(|e| format!("Error reading config: {}", e))?;
    serde_yaml::from_str(&contents).map_err(|e| format!("Error parsing config: {}", e))
}