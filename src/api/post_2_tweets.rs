use crate::responses::errors::Errors;
use crate::{api::execute_twitter, error::Error, rate_limit::RateLimit};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.twitter.com/2/tweets";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Geo {
    place_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Media {
    media_ids: Vec<String>,
    tagged_user_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Poll {
    duration_minutes: String,
    options: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Reply {
    exclude_reply_user_ids: Option<Vec<String>>,
    in_reply_to_tweet_id: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum ReplySettings {
    Mentionedusers,
    Following,
}

impl std::fmt::Display for ReplySettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Mentionedusers => write!(f, "mentionedUsers"),
            Self::Following => write!(f, "following"),
        }
    }
}

impl Default for ReplySettings {
    fn default() -> Self {
        Self::Mentionedusers
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    direct_message_deep_link: Option<String>,
    for_super_followers_only: Option<String>,
    geo: Option<Geo>,
    media: Option<Media>,
    poll: Option<Poll>,
    quote_tweet_id: Option<String>,
    reply: Option<Reply>,
    reply_settings: Option<ReplySettings>,
    text: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    bearer_code: String,
    body: Body,
}

impl Api {
    pub fn new(bearer_code: &str, body: Body) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            body,
        }
    }

    pub fn build(self) -> RequestBuilder {
        let client = reqwest::Client::new();
        client
            .post(URL)
            .bearer_auth(self.bearer_code)
            .json(&serde_json::to_value(&self.body).unwrap())
    }

    pub async fn execute(self) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build()).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub data: Option<Data>,
    pub errors: Option<Vec<Errors>>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Response {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .data
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true)
            && self
                .errors
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true);
        if !res {
            println!("Response {:?}", self.extra);
        }
        res
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Data {
    pub id: Option<String>,
    pub text: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Data {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("Data {:?}", self.extra);
        }
        res
    }
}
