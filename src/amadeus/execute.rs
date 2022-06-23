/*!
One-line description.

More detailed description, with

# Example

*/

use crate::amadeus::error::Error as ApiError;
use crate::config::{get_stage, ServiceConfig};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::GetRequest;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseMetadata {
    count: u32,
    links: BTreeMap<String, String>,
}

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

pub(crate) const AMADEUS_SERVICE_NAME: &str = "amadeus";

pub(crate) const AMADEUS_RESPONSE_TYPE: &str = "application/vnd.amadeus+json";

macro_rules! make_api_call {
    ($fn_name:ident, $request:ident, $response:ident) => {
        pub async fn $fn_name(
            config: &mut AppConfig,
            request: &$request,
        ) -> Result<$response, Box<dyn std::error::Error>> {
            let service = config
                .service_mut($crate::amadeus::execute::AMADEUS_SERVICE_NAME)
                .unwrap();

            if !service.has_access_token() {
                let secret_name = format!(
                    "{}_API_SECRET",
                    $crate::amadeus::execute::AMADEUS_SERVICE_NAME.to_uppercase());
                if let Ok(secret) = std::env::var(secret_name) {
                    let token = $crate::amadeus::execute::get_access_token(
                        service,
                        secret,
                    )
                        .await?;
                    service.set_access_token(token);
                } else {
                    error!(
                        "no API secret set for service {}",
                        $crate::amadeus::execute::AMADEUS_SERVICE_NAME);
                    panic!();
                }
            }

            debug!("{:?}", service);

            let request_url = $crate::amadeus::execute::make_service_url(service, request);
            debug!("GET {}", request_url);

            let client = reqwest::Client::new();

            let request = client
                .get(&request_url)
                .header("Accept", $crate::amadeus::execute::AMADEUS_RESPONSE_TYPE)
                .bearer_auth(&service.access_token().unwrap())
                .build()?;

            debug!("{:?}", request);

            let response = client.execute(request).await?;

            debug!("response: {:?}", response);
            let success = response.status().is_success();

            let text = response.text().await?;

            if success {
                let actual: $response = serde_json::from_str(&text)?;
                debug!("actual: {:?}", actual);
                Ok(actual)
            } else {
                let error: $crate::amadeus::error::Error = serde_json::from_str(&text)?;
                error!("{}", text);
                Err(Box::new(error))
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

struct AuthRequest {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RequestState {
    Approved,
    Expired,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct AuthResponse {
    #[serde(rename = "type")]
    request_type: String,
    username: String,
    application_name: String,
    client_id: String,
    token_type: String,
    access_token: String,
    expires_in: u32,
    state: RequestState,
    scope: String,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl GetRequest for AuthRequest {
    const VERSION: u16 = 1;
    const PATH: &'static str = "/security/oauth2/token";
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline]
pub(crate) fn make_service_url(config: &ServiceConfig, request: &impl GetRequest) -> String {
    format!(
        "{}/v{}{}{}",
        config.endpoint(get_stage()).unwrap(),
        request.version(),
        request.path(),
        match request.query() {
            Some(v) => format!("?{}", v),
            None => String::new(),
        }
    )
}

const AUTH_REQUEST: AuthRequest = AuthRequest {};

pub(crate) async fn get_access_token(
    config: &mut ServiceConfig,
    secret: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let request_url = make_service_url(config, &AUTH_REQUEST);
    debug!("POST {}", request_url);

    let api_key = config.api_key().clone();

    let mut form = std::collections::HashMap::new();
    form.insert("grant_type", "client_credentials");
    form.insert("client_id", &api_key);
    form.insert("client_secret", &secret);

    debug!("{:?}", form);

    let response = reqwest::Client::new()
        .post(&request_url)
        .header("Accept", AMADEUS_RESPONSE_TYPE)
        .form(&form)
        .send()
        .await?;

    debug!("response: {:?}", response);
    let success = response.status().is_success();

    let text: String = response.text().await?;

    if success {
        let actual: AuthResponse = serde_json::from_str(&text)?;
        Ok(actual.access_token)
    } else {
        let error: ApiError = serde_json::from_str(&text)?;
        error!("{}", text);
        Err(Box::new(error))
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
