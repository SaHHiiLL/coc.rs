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

use crate::dev::{APIAccount, Credential, Index};

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use crate::dev;

#[derive(Debug)]
pub struct Client {
    pub client: reqwest::Client,
    pub ready: bool,

    pub accounts: Vec<APIAccount>,
    pub index: Mutex<Index>,
    pub use_cache: bool,

    pub ip_address: String,
}

#[derive(Debug)]
pub enum ApiError {
    Request(reqwest::Error),
    Api(reqwest::StatusCode),
}

pub const BASE_URL: &str = "https://api.clashofclans.com/v1";
pub const BASE_DEV_URL: &str = "https://developer.clashofclans.com/api";
const IP_URL: &str = "https://api.ipify.org";

impl Client {

    pub fn new(cred: Vec<dev::Credential>) -> Self {
        let client = Self {
            client: reqwest::Client::builder().cookie_store(true).build().unwrap(),
            ready: false,
            accounts: Vec::new(),
            index: Mutex::new(
                Index::new()
            ),
            use_cache: false,
            ip_address: String::new(),
        };
    }

    /// Should return an error variant if the account is already in the list (will not add).
    /// May be a better error here in future.
    pub fn add_login_credentials(&mut self, email: String, password: String) -> Result<(), ()> {
        let credential = dev::Credential::new(email, password);
        let account = dev::APIAccount::new(credential);

        //let vec = &self.accounts.
        //let x: Vec<APIAccount> = vec.into_iter().filter(|x|  x.credential().email == email).collect();

        //if x.len() == 0 {
        //    Err(())
        //}else {
        self.accounts.push(account);
        Ok(())
    }

    pub async fn login_all(&self) {
        for x in &self.accounts {
            let x1 = x.login(&self.client).await.unwrap();
            println!("{:?}", x1);

            self.get_keys(&self.client).await
        }
    }

    async fn get_keys(&self, client: &reqwest::Client){
        let response = client.post(format!("{}/apikey/list", BASE_DEV_URL))
            .send()
            .await
            .unwrap();

        println!("{}", response.text().await.expect("cannot paste text"))
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

    fn get_current_token(&self) -> String{

        // let acc_ind = self.index.lock().unwrap().key_account_index();
        // let key_ind = self.index.lock().unwrap().key_index();
        //
        // let x = self.accounts.get(acc_ind as usize).unwrap();
        // let x1 = x.keys().keys().get(key_ind as usize).unwrap().key();
        //
        // self.inc_index();
        //
        // x1.to_string()
        todo!()
    }

    fn get(&self, url: String) -> Result<reqwest::RequestBuilder, reqwest::Error> {
        let res = reqwest::Client::new().get(url).bearer_auth(self.get_current_token());
        Ok(res)
    }

    fn post(&self, url: String, body: String) -> Result<reqwest::RequestBuilder, reqwest::Error> {
        let res = reqwest::Client::new().post(url).bearer_auth(self.get_current_token()).body(body);
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
