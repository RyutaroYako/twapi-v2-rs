use serde::{Serialize, Deserialize};
use crate::responses::{trends::Trends};
use reqwest::RequestBuilder;
use crate::{error::Error, rate_limit::RateLimit, api::{execute_twitter, Authentication}};

const URL: &str = "https://api.twitter.com/2/trends/by/woeid/:woeid";





#[derive(Debug, Clone, Default)]
pub struct Api {
    woeid: String,
}

impl Api {
    pub fn new(woeid: &str) -> Self {
        Self {
            woeid: woeid.to_owned(),
        }
    }
    
    pub fn build(self, authentication: &impl Authentication) -> RequestBuilder {
        
        let client = reqwest::Client::new();
        let url = URL.replace(":woeid", &self.woeid);
        let builder = client
            .get(&url)
        ;
        authentication.execute(builder, "GET", &url, &[])
    }

    pub async fn execute(self, authentication: &impl Authentication) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build(authentication)).await
    }
}



#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Trends>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Response {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.data.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true);
        if !res {
          println!("Response {:?}", self.extra);
        }
        res
    }
}