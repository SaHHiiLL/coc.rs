use regex::Regex;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::error::Error;
use std::sync::Mutex;

extern crate reqwest;

use crate::models::clan::Clan;
use crate::models::current_war::War;
use crate::models::gold_pass::GoldPass;
use crate::models::player::{Player, PlayerToken};

use crate::dev::{APIAccount, Index};

use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    ready: bool,

    accounts: Vec<APIAccount>,
    index: Mutex<Index>,
    use_cache: bool,

    ip_address: String,
}

#[derive(Debug)]
pub enum ApiError {
    Request(reqwest::Error),
    Api(reqwest::StatusCode),
}

const BASE_URL: &str = "https://api.clashofclans.com/v1";
const IP_URL: &str = "https://api.ipify.org";

impl Client {
    pub fn new(token: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            ready: false,
            accounts: Vec::new(),
            index: Mutex::new(Index {
                key_account_index: 0,
                key_index: 0,
            }),
            use_cache: false,
            ip_address: String::new(),
        }
    }

    async fn get_ip(&self) -> Result<String, reqwest::Error> {
        let res = self.client.get(IP_URL).send().await?;
        let ip = res.text().await?;
        Ok(ip)
    }

    fn inc_index(&self) {
        let mut index = self.index.lock().unwrap();
        if index.key_index()
            == (self
                .accounts
                .get(index.key_account_index() as usize)
                .unwrap()
                .keys()
                .keys()
                .len()
                - 1) as i8
        {
            index.inc_account()
        } else {
            index.inc()
        }
    }

    fn get(&self, url: String) -> Result<reqwest::RequestBuilder, reqwest::Error> {
        let string = format!("Bearer {}", &self.token);
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_str(&string).unwrap());
        let res = reqwest::Client::new().get(url).headers(headers);
        Ok(res)
    }

    fn post(&self, url: String, body: String) -> Result<reqwest::RequestBuilder, reqwest::Error> {
        let string = format!("Bearer {}", &self.token);
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_str(&string).unwrap());
        let res = reqwest::Client::new().post(url).headers(headers).body(body);
        Ok(res)
    }

    pub async fn get_clan(&self, tag: String) -> Result<Clan, ApiError> {
        let url = format!("{}/clans/{}", BASE_URL, self.format_tag(tag));
        self.parse_json::<Clan>(self.get(url)).await
    }

    pub async fn get_player(&self, tag: String) -> Result<Player, ApiError> {
        let url = format!("{}/players/{}", BASE_URL, self.format_tag(tag));
        self.parse_json::<Player>(self.get(url)).await
    }

    pub async fn get_current_war(&self, tag: String) -> Result<War, ApiError> {
        let url = format!("{}/clans/{}/currentwar", BASE_URL, self.format_tag(tag));
        self.parse_json::<War>(self.get(url)).await
    }

    pub async fn get_goldpass(&self, tag: String) -> Result<GoldPass, ApiError> {
        let url = format!("{}/goldpass/seasons/current", BASE_URL);
        self.parse_json::<GoldPass>(self.get(url)).await
    }

    pub async fn get_verified_player(
        &self,
        tag: String,
        token: String,
    ) -> Result<PlayerToken, ApiError> {
        let url = format!("{}/players/{}/verifytoken", BASE_URL, self.format_tag(tag));
        let token = format!("{{\"token\":\"{}\"}}", token);
        self.parse_json::<PlayerToken>(self.post(url, token)).await
    }

    fn format_tag(&self, tag: String) -> String {
        return if tag[0..1].eq_ignore_ascii_case("#") {
            tag.replace("#", "%23")
        } else {
            format!("%23{}", tag)
        };
    }

    fn fix_tag(&self, tag: String) -> String {
        let re = Regex::new("[^A-Z0-9]+").unwrap();
        "#".to_owned()
            + &re
                .replace_all(tag.to_uppercase().as_str(), "")
                .replace("O", "0")
    }

    async fn parse_json<T: DeserializeOwned>(
        &self,
        rb: Result<reqwest::RequestBuilder, reqwest::Error>,
    ) -> Result<T, ApiError> {
        match rb {
            Ok(rb) => match rb.send().await {
                Ok(res) => match res.status() {
                    reqwest::StatusCode::OK => Ok(res
                        .json()
                        .await
                        .expect("Unexpected json response from the API, cannot parse json")),
                    _ => Err(ApiError::Api(res.status())),
                },
                Err(e) => Err(ApiError::Request(e)),
            },
            Err(e) => return Err(ApiError::Request(e)),
        }
    }
}
