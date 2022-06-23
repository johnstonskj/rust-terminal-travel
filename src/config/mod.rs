/*!
One-line description.

More detailed description, with

# Example

*/

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tracing::debug;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const APP_CONFIG_NAME: &str = "travelcli";

pub const CONFIG_FILE_NAME: &str = "config.yml";

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Stage {
    Development,
    Test,
    Production,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceConfig {
    endpoint: BTreeMap<Stage, String>,
    api_key: String,
    #[serde(skip)]
    access_token: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    services: BTreeMap<String, ServiceConfig>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn get_stage() -> Stage {
    match std::env::var("STAGE") {
        Ok(s) => match Stage::from_str(&s) {
            Ok(v) => {
                debug!("Environment stage: {}", v);
                v
            }
            Err(_) => {
                warn!("environment variable value invalid: {}", s);
                Stage::default()
            }
        },
        Err(e) => {
            warn!("environment variable error: {}", e);
            Stage::default()
        }
    }
}

pub fn get_app_config_from(file_name: &Path) -> Result<AppConfig, crate::error::Error> {
    debug!("expecting to read config from file {:?}", file_name);
    if file_name.is_file() {
        warn!(
            "config file {:?} could not be read, using defaults",
            file_name
        );
        Ok(default_config_file())
    } else {
        warn!("config file {:?} does not exist, using defaults", file_name);
        Ok(default_config_file())
    }
}

pub fn get_app_config_path() -> PathBuf {
    match xdirs::config_dir_for(APP_CONFIG_NAME) {
        Some(p) => p.join(CONFIG_FILE_NAME),
        None => PathBuf::from(
            shellexpand::tilde(&format!(
                "~/.config/{}/{}",
                APP_CONFIG_NAME, CONFIG_FILE_NAME
            ))
            .as_ref(),
        ),
    }
}

pub fn write_default_config() -> Result<(), crate::error::Error> {
    debug!("writing default config");
    let default_config = default_config_file();

    let file_path = get_app_config_path();
    if let Some(parent_dir) = file_path.parent() {
        debug!("creating parent directory path {:?}", parent_dir);
        std::fs::create_dir_all(parent_dir)?;
    }

    let file = File::open(file_path)?;
    serde_yaml::to_writer(file, &default_config)?;
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Stage {
    fn default() -> Self {
        Self::Development
    }
}

impl Display for Stage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Stage::Development => "development",
                Stage::Test => "test",
                Stage::Production => "production",
            }
        )
    }
}

impl FromStr for Stage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dev" | "development" => Ok(Self::Development),
            "tst" | "test" => Ok(Self::Test),
            "prod" | "production" => Ok(Self::Production),
            _ => Err(()),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl ServiceConfig {
    pub fn endpoint(&self, stage: Stage) -> Option<&String> {
        self.endpoint.get(&stage)
    }

    pub fn api_key(&self) -> &String {
        &self.api_key
    }

    pub fn has_access_token(&self) -> bool {
        self.access_token.is_some()
    }

    pub fn access_token(&self) -> Option<&String> {
        self.access_token.as_ref()
    }

    pub fn set_access_token(&mut self, token: String) {
        self.access_token = Some(token)
    }
}

// ------------------------------------------------------------------------------------------------

impl AppConfig {
    pub fn service(&self, name: &str) -> Option<&ServiceConfig> {
        self.services.get(name)
    }

    pub fn service_mut(&mut self, name: &str) -> Option<&mut ServiceConfig> {
        self.services.get_mut(name)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn default_config_file() -> AppConfig {
    let mut app_config = AppConfig {
        services: Default::default(),
    };
    let mut amadeus_config = ServiceConfig {
        endpoint: Default::default(),
        api_key: "0Wt1GAHEegGd3uQZWGBQdmvjffdiqfw5".to_string(),
        access_token: None,
    };
    amadeus_config.endpoint.insert(
        Stage::Development,
        "https://test.api.amadeus.com".to_string(),
    );
    let _ = app_config
        .services
        .insert("amadeus".to_string(), amadeus_config);

    app_config
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
