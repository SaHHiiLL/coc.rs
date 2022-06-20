use crate::api;
use serde::{ Serialize, Deserialize };

// manage a session
#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    key_account_index: i8,
    key_index: i8,
}

impl Index {
    pub fn new() -> Self {
        Self {
            key_account_index: 0,
            key_index: 0,
        }
    }

    pub fn key_account_index(&self) -> i8 {
        self.key_account_index
    }

    pub fn key_index(&self) -> i8 {
        self.key_index
    }

    pub fn set_key_account_index(&mut self, key_account_index: i8) {
        self.key_account_index = key_account_index;
    }

    pub fn set_key_index(&mut self, key_index: i8) {
        self.key_index = key_index;
    }

    pub fn reset(&mut self) {
        self.key_account_index = 0;
        self.key_index = 0;
    }

    // Increment the key index
    pub fn inc(&mut self) {
        self.key_index += 1;
    }

    // Increment the key account index, reset the key index
    pub fn inc_account(&mut self) {
        self.key_account_index += 1;
        self.key_index = 0;
    }
}

#[derive(Debug, Serialize, Deserialize)]

pub struct APIAccount {
    credential: Credential,
    response: LoginResponse,
    keys: Keys,
}

impl APIAccount {
    pub fn new(credential: Credential) -> Self {
        Self {
            credential,
            response: LoginResponse::new(),
            keys: Keys::new(),
        }
    }

    pub fn credential(&self) -> &Credential {
        &self.credential
    }

    pub fn response(&self) -> &LoginResponse {
        &self.response
    }

    pub fn keys(&self) -> &Keys {
        &self.keys
    }

    pub async fn login(&self, client: reqwest::Client) -> Result<LoginResponse, reqwest::Error> {
        let mut res = client
            .post(api::BASE_URL.to_string() + "/login")
            .json(&self.credential)
            .send()
            .await?;
        let response = res.json().await?;
        Ok(response)
    }
}

#[derive(Debug, Serialize, Deserialize)]

struct Credential {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]

struct LoginResponse {
    status: Status,
    session_expires_in_seconds: i32,
    auth: Auth,
    developer: Developer,
    temporary_api_token: String,
    swagger_url: String,
}

impl LoginResponse {
    pub fn new() -> Self {
        Self {
            status: Status::new(),
            session_expires_in_seconds: 0,
            auth: Auth::new(),
            developer: Developer::new(),
            temporary_api_token: String::new(),
            swagger_url: String::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]

struct Auth {
    uid: String,
    token: String,
    ua: String,
    ip: String,
}

impl Auth {
    pub fn new() -> Self {
        Self {
            uid: String::new(),
            token: String::new(),
            ua: String::new(),
            ip: String::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]

struct Developer {
    id: String,
    name: String,
    game: String,
    email: String,
    tier: String,
    allowed_scopes: String,
    max_cidrs: String,
    prev_login_ts: String,
    prev_login_ip: String,
    prev_login_ua: String,
}

impl Developer {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            game: String::new(),
            email: String::new(),
            tier: String::new(),
            allowed_scopes: String::new(),
            max_cidrs: String::new(),
            prev_login_ts: String::new(),
            prev_login_ip: String::new(),
            prev_login_ua: String::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]

struct Keys {
    keys: Vec<Key>,
    status: Status,
    session_expire: i32,
}

impl Keys {
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            status: Status::new(),
            session_expire: 0,
        }
    }

    pub fn keys(&self) -> &Vec<Key> {
        &self.keys
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn session_expire(&self) -> i32 {
        self.session_expire
    }
}

#[derive(Debug, Serialize, Deserialize)]

struct Key {
    id: String,
    developer_id: String,
    tier: String,
    name: String,
    description: String,
    origins: String,
    scopes: Vec<String>,
    cidr_ranges: Vec<String>,
    valid_until: String,
    key: String,
}

#[derive(Debug, Serialize, Deserialize)]

struct KeyResponse {
    status: Status,
    session_expire_in_seconds: i32,
    key: Key,
}

#[derive(Debug, Serialize, Deserialize)]

struct Status {
    code: i32,
    message: String,
    detail: String,
}
