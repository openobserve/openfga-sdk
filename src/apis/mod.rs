use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Applies the authentication configured on `configuration` to an outgoing request.
///
/// Precedence: `bearer_access_token` > `oauth_access_token` > `api_key` > `basic_auth`.
/// All token-based methods produce an `Authorization` header; a pre-shared key / OIDC
/// access token is sent as `Authorization: Bearer <token>`, which is what the OpenFGA
/// server expects for both its `preshared` and `oidc` auth methods. When no auth field is
/// set the request is returned unchanged, preserving the unauthenticated behavior.
pub(crate) fn apply_auth(
    mut req_builder: reqwest::RequestBuilder,
    configuration: &configuration::Configuration,
) -> reqwest::RequestBuilder {
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token);
    } else if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token);
    } else if let Some(ref api_key) = configuration.api_key {
        let value = match api_key.prefix {
            Some(ref prefix) => format!("{} {}", prefix, api_key.key),
            None => api_key.key.clone(),
        };
        req_builder = req_builder.header(reqwest::header::AUTHORIZATION, value);
    } else if let Some((ref username, ref password)) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(username, password.clone());
    }
    req_builder
}

pub mod assertions_api;
pub mod authorization_models_api;
pub mod relationship_queries_api;
pub mod relationship_tuples_api;
pub mod stores_api;

pub mod configuration;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apis::configuration::{ApiKey, Configuration};

    fn auth_header(configuration: &Configuration) -> Option<String> {
        let req = apply_auth(
            configuration.client.get("http://localhost/stores"),
            configuration,
        )
        .build()
        .unwrap();
        req.headers()
            .get(reqwest::header::AUTHORIZATION)
            .map(|v| v.to_str().unwrap().to_owned())
    }

    #[test]
    fn no_auth_when_all_fields_none() {
        let cfg = Configuration::default();
        assert_eq!(auth_header(&cfg), None);
    }

    #[test]
    fn bearer_token_sets_authorization_header() {
        let cfg = Configuration {
            bearer_access_token: Some("secret-key".to_owned()),
            ..Configuration::default()
        };
        assert_eq!(auth_header(&cfg), Some("Bearer secret-key".to_owned()));
    }

    #[test]
    fn bearer_token_takes_precedence_over_other_methods() {
        let cfg = Configuration {
            bearer_access_token: Some("the-bearer".to_owned()),
            oauth_access_token: Some("the-oauth".to_owned()),
            api_key: Some(ApiKey {
                prefix: None,
                key: "the-key".to_owned(),
            }),
            basic_auth: Some(("user".to_owned(), Some("pass".to_owned()))),
            ..Configuration::default()
        };
        assert_eq!(auth_header(&cfg), Some("Bearer the-bearer".to_owned()));
    }

    #[test]
    fn api_key_with_prefix_sets_authorization_header() {
        let cfg = Configuration {
            api_key: Some(ApiKey {
                prefix: Some("Bearer".to_owned()),
                key: "the-key".to_owned(),
            }),
            ..Configuration::default()
        };
        assert_eq!(auth_header(&cfg), Some("Bearer the-key".to_owned()));
    }
}
