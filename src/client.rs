use crate::error::{Error, Result};
use md5::Md5;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use sha2::Sha512;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use urlencoding::encode;

const API_ENDPOINT: &str = "/cgi/json-req";

#[derive(Clone, Copy, PartialEq, Default)]
pub enum AuthMethod {
    #[default]
    SHA512,
    MD5,
}

impl From<&str> for AuthMethod {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "MD5" => AuthMethod::MD5,
            _ => AuthMethod::SHA512,
        }
    }
}

#[derive(Clone)]
pub struct Client {
    host: String,
    username: String,
    password_hash: String,
    http_client: reqwest::Client,
    session: Arc<Mutex<Session>>,
    refresh_interval: Duration,
    auth_method: AuthMethod,
}

struct Session {
    session_id: Option<i64>,
    server_nonce: Option<String>,
    request_id: i64,
    last_refresh: Instant,
}

impl Client {
    pub fn new(
        host: String,
        username: String,
        password: String,
        refresh_interval: Duration,
        auth_method: AuthMethod,
    ) -> Self {
        let password_hash = match auth_method {
            AuthMethod::SHA512 => {
                let mut hasher = Sha512::new();
                hasher.update(password.as_bytes());
                format!("{:x}", hasher.finalize())
            }
            AuthMethod::MD5 => {
                let mut hasher = Md5::new();
                hasher.update(password.as_bytes());
                format!("{:x}", hasher.finalize())
            }
        };

        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self {
            host,
            username,
            password_hash,
            http_client,
            session: Arc::new(Mutex::new(Session {
                session_id: None,
                server_nonce: None,
                request_id: -1,
                last_refresh: Instant::now(),
            })),
            refresh_interval,
            auth_method,
        }
    }

    fn generate_hash(&self, data: &str) -> String {
        match self.auth_method {
            AuthMethod::SHA512 => {
                let mut hasher = Sha512::new();
                hasher.update(data.as_bytes());
                format!("{:x}", hasher.finalize())
            }
            AuthMethod::MD5 => {
                let mut hasher = Md5::new();
                hasher.update(data.as_bytes());
                format!("{:x}", hasher.finalize())
            }
        }
    }

    fn generate_auth_key(&self, server_nonce: &str, request_id: i64, nonce: i64) -> String {
        let credential_hash = {
            let cred = format!("{}:{}:{}", self.username, server_nonce, self.password_hash);
            self.generate_hash(&cred)
        };

        let auth_string = format!(
            "{}:{}:{}:JSON:{}",
            credential_hash, request_id, nonce, API_ENDPOINT
        );

        self.generate_hash(&auth_string)
    }

    async fn do_request(&self, url: &str, body: String) -> Result<serde_json::Value> {
        let resp = self
            .http_client
            .post(url)
            .header(
                "Content-Type",
                "application/x-www-form-urlencoded; charset=UTF-8",
            )
            .header("Accept", "text/javascript, application/json, */*;q=0.01")
            .body(body)
            .send()
            .await
            .map_err(Error::Http)?;

        let status = resp.status();
        let text = resp.text().await.map_err(Error::Http)?;
        if !status.is_success() {
            tracing::warn!("non-success status: {} body: {}", status, text);
        }

        tracing::debug!("response: {}", text);

        let result: serde_json::Value = serde_json::from_str(&text).map_err(Error::Json)?;

        Ok(result)
    }

    async fn login(&self) -> Result<()> {
        let mut session = self.session.lock().await;
        session.session_id = None;
        session.server_nonce = None;
        session.request_id = -1;

        let request_id = session.request_id + 1;
        session.request_id = request_id;
        let nonce = generate_nonce();

        let auth_key = self.generate_auth_key("", request_id, nonce);

        #[derive(serde::Serialize)]
        struct LoginAction<'a> {
            #[serde(rename = "method")]
            method: &'a str,
            #[serde(rename = "parameters")]
            parameters: LoginParameters<'a>,
        }

        #[derive(serde::Serialize)]
        struct LoginParameters<'a> {
            #[serde(rename = "user")]
            user: &'a str,
            #[serde(rename = "persistent")]
            persistent: bool,
            #[serde(rename = "session-options")]
            session_options: SessionOptions,
        }

        #[derive(serde::Serialize)]
        struct SessionOptions {
            #[serde(rename = "nss")]
            nss: Vec<Nss>,
            #[serde(rename = "language")]
            language: &'static str,
            #[serde(rename = "context-flags")]
            context_flags: ContextFlags,
            #[serde(rename = "capability-depth")]
            capability_depth: i32,
            #[serde(rename = "capability-flags")]
            capability_flags: CapabilityFlags,
            #[serde(rename = "time-format")]
            time_format: &'static str,
            #[serde(rename = "write-only-string")]
            write_only_string: &'static str,
            #[serde(rename = "undefined-write-only-string")]
            undefined_write_only_string: &'static str,
        }

        #[derive(serde::Serialize)]
        struct Nss {
            #[serde(rename = "name")]
            name: &'static str,
            #[serde(rename = "uri")]
            uri: &'static str,
        }

        #[derive(serde::Serialize)]
        struct ContextFlags {
            #[serde(rename = "get-content-name")]
            get_content_name: bool,
            #[serde(rename = "local-time")]
            local_time: bool,
        }

        #[derive(serde::Serialize)]
        struct CapabilityFlags {
            #[serde(rename = "name")]
            name: bool,
            #[serde(rename = "default-value")]
            default_value: bool,
            #[serde(rename = "restriction")]
            restriction: bool,
            #[serde(rename = "description")]
            description: bool,
        }

        #[derive(serde::Serialize)]
        struct LoginRequest<'a> {
            #[serde(rename = "id")]
            id: i64,
            #[serde(rename = "session-id")]
            session_id: i64,
            #[serde(rename = "priority")]
            priority: bool,
            #[serde(rename = "actions")]
            actions: Vec<LoginAction<'a>>,
            #[serde(rename = "cnonce")]
            cnonce: i64,
            #[serde(rename = "auth-key")]
            auth_key: &'a str,
        }

        let request = LoginRequest {
            id: request_id,
            session_id: -1,
            priority: false,
            actions: vec![LoginAction {
                method: "logIn",
                parameters: LoginParameters {
                    user: &self.username,
                    persistent: true,
                    session_options: SessionOptions {
                        nss: vec![Nss {
                            name: "gtw",
                            uri: "http://sagemcom.com/gateway-data",
                        }],
                        language: "ident",
                        context_flags: ContextFlags {
                            get_content_name: true,
                            local_time: true,
                        },
                        capability_depth: 2,
                        capability_flags: CapabilityFlags {
                            name: true,
                            default_value: false,
                            restriction: true,
                            description: false,
                        },
                        time_format: "ISO_8601",
                        write_only_string: "_XMO_WRITE_ONLY_",
                        undefined_write_only_string: "_XMO_UNDEFINED_WRITE_ONLY_",
                    },
                },
            }],
            cnonce: nonce,
            auth_key: &auth_key,
        };

        #[derive(serde::Serialize)]
        struct Payload<'a> {
            #[serde(rename = "request")]
            request: LoginRequest<'a>,
        }

        let payload = Payload { request };
        let payload_json = serde_json::to_string(&payload).map_err(Error::Json)?;

        let url = format!("http://{}{}", self.host, API_ENDPOINT);
        let body = format!("req={}", encode(&payload_json.to_string()));
        tracing::debug!("login request body: {}", body);

        let result = self.do_request(&url, body).await?;

        tracing::debug!("login response: {}", result);

        let (id, nonce) = parse_login_reply(&result)?;

        session.session_id = Some(id);
        session.server_nonce = Some(nonce);
        session.last_refresh = Instant::now();

        tracing::info!(session_id = id, "logged in");

        Ok(())
    }

    pub async fn api_request(&self, actions: Vec<Action>) -> Result<serde_json::Value> {
        let mut session = self.session.lock().await;
        let needs_login = session.session_id.is_none();
        let needs_refresh = !needs_login && session.last_refresh.elapsed() >= self.refresh_interval;

        if needs_login || needs_refresh {
            drop(session);
            self.login().await?;
            session = self.session.lock().await;
        }

        let request_id = session.request_id + 1;
        session.request_id = request_id;
        let nonce = generate_nonce();

        let auth_key = self.generate_auth_key(
            session.server_nonce.as_ref().unwrap_or(&"".to_string()),
            request_id,
            nonce,
        );

        let payload = serde_json::json!({
            "request": {
                "id": request_id,
                "session-id": session.session_id.unwrap_or(-1),
                "priority": false,
                "actions": actions,
                "cnonce": nonce,
                "auth-key": auth_key
            }
        });

        drop(session);

        let url = format!("http://{}{}", self.host, API_ENDPOINT);
        let body = format!("req={}", encode(&payload.to_string()));

        let result = self.do_request(&url, body).await?;

        Ok(result)
    }
}

fn generate_nonce() -> i64 {
    (rand::random::<u32>() % 500000) as i64
}

fn parse_login_reply(result: &serde_json::Value) -> Result<(i64, String)> {
    let reply = result
        .get("reply")
        .ok_or_else(|| Error::InvalidResponse("no reply in response".to_string()))?;

    let reply_error_code = reply
        .get("error")
        .and_then(|e| e.get("code"))
        .and_then(|c| c.as_i64())
        .unwrap_or(0);
    if reply_error_code != 0
        && reply_error_code != 16777238
        && reply_error_code != 16777216
        && reply_error_code != 16777236
    {
        let desc = reply
            .get("error")
            .and_then(|e| e.get("description"))
            .and_then(|d| d.as_str())
            .unwrap_or("unknown error");
        return Err(Error::Api(format!("{} ({})", desc, reply_error_code)));
    }

    let callback = reply
        .get("actions")
        .and_then(|a| a.as_array())
        .and_then(|a| a.first())
        .and_then(|a| a.get("callbacks"))
        .and_then(|c| c.as_array())
        .and_then(|c| c.first())
        .ok_or_else(|| Error::InvalidResponse("missing callbacks".to_string()))?;

    if let Some(result_obj) = callback.get("result") {
        let code = result_obj.get("code").and_then(|c| c.as_i64()).unwrap_or(0);
        if code != 16777238 {
            let desc = result_obj
                .get("description")
                .and_then(|d| d.as_str())
                .unwrap_or("unknown error");
            return Err(Error::Api(format!("{} ({})", desc, code)));
        }
    } else if reply_error_code == 16777236 {
        let desc = reply
            .get("error")
            .and_then(|e| e.get("description"))
            .and_then(|d| d.as_str())
            .unwrap_or("request action error");
        return Err(Error::Api(format!("{} ({})", desc, reply_error_code)));
    }

    let parameters = callback
        .get("parameters")
        .ok_or_else(|| Error::InvalidResponse("missing callback parameters".to_string()))?;

    tracing::debug!("login parameters: {}", parameters);

    let nonce_value = parameters
        .get("nonce")
        .ok_or_else(|| Error::InvalidResponse("missing nonce in response".to_string()))?;
    let nonce = parse_nonce(nonce_value)?;

    let id = parameters
        .get("id")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| Error::InvalidResponse("id is not a number".to_string()))?;

    Ok((id, nonce))
}

fn parse_nonce(value: &serde_json::Value) -> Result<String> {
    if let Some(nonce) = value.as_str() {
        return Ok(nonce.to_string());
    }

    if let Some(nonce) = value.as_i64() {
        return Ok(nonce.to_string());
    }

    if let Some(nonce) = value.as_u64() {
        return Ok(nonce.to_string());
    }

    tracing::debug!("nonce value: {:?}", value);
    Err(Error::InvalidResponse(
        "nonce is not a string or number".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::{parse_login_reply, parse_nonce, Error};
    use serde_json::json;

    #[test]
    fn parse_nonce_accepts_string() {
        let nonce = parse_nonce(&json!("4152204859")).expect("parse nonce");
        assert_eq!(nonce, "4152204859");
    }

    #[test]
    fn parse_nonce_accepts_number() {
        let nonce = parse_nonce(&json!(4152204859u64)).expect("parse nonce");
        assert_eq!(nonce, "4152204859");
    }

    #[test]
    fn parse_login_reply_returns_auth_error() {
        let result = json!({
            "reply": {
                "uid": 0,
                "id": 0,
                "error": { "code": 16777236, "description": "XMO_REQUEST_ACTION_ERR" },
                "actions": [{
                    "uid": 1,
                    "id": 0,
                    "error": { "code": 16777223, "description": "XMO_AUTHENTICATION_ERR" },
                    "callbacks": [{
                        "uid": 1,
                        "result": { "code": 16777223, "description": "XMO_AUTHENTICATION_ERR" },
                        "xpath": "Device",
                        "parameters": {}
                    }]
                }],
                "events": []
            }
        });

        match parse_login_reply(&result) {
            Err(Error::Api(msg)) => assert!(msg.contains("XMO_AUTHENTICATION_ERR")),
            other => panic!("expected auth error, got {:?}", other),
        }
    }

    #[test]
    fn parse_login_reply_success() {
        let result = json!({
            "reply": {
                "actions": [{
                    "callbacks": [{
                        "parameters": {
                            "nonce": "4152204859",
                            "id": 123
                        },
                        "result": { "code": 16777238, "description": "XMO_NO_ERR" }
                    }]
                }],
                "error": { "code": 16777238, "description": "XMO_NO_ERR" }
            }
        });

        let (id, nonce) = parse_login_reply(&result).expect("parse login reply");
        assert_eq!(id, 123);
        assert_eq!(nonce, "4152204859");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: i64,
    pub method: String,
    pub xpath: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parameters: Option<serde_json::Value>,
}
