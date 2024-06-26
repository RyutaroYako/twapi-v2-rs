use std::io::Cursor;

use crate::{
    api::{Authentication, TwapiOptions},
    error::Error,
    headers::Headers,
    upload::{execute_no_response, make_url},
};
use reqwest::{
    multipart::{Form, Part},
    RequestBuilder,
};

#[derive(Debug, Clone, Default)]
pub struct Data {
    pub media_id: String,
    pub segment_index: u64,
    pub cursor: Cursor<Vec<u8>>,
}

impl Data {
    fn make_form(self) -> Form {
        Form::new()
            .text("command", "APPEND")
            .text("media_id", self.media_id)
            .text("segment_index", self.segment_index.to_string())
            .part("media", Part::bytes(self.cursor.into_inner()))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    data: Data,
    twapi_options: Option<TwapiOptions>,
}

impl Api {
    pub fn new(data: Data) -> Self {
        Self {
            data,
            ..Default::default()
        }
    }

    pub fn twapi_options(mut self, value: TwapiOptions) -> Self {
        self.twapi_options = Some(value);
        self
    }

    pub fn build(self, authentication: &impl Authentication) -> RequestBuilder {
        let client = reqwest::Client::new();
        let url = make_url(&self.twapi_options, None);
        let builder = client.post(&url).multipart(self.data.make_form());
        authentication.execute(builder, "POST", &url, &[])
    }

    pub async fn execute(self, authentication: &impl Authentication) -> Result<Headers, Error> {
        execute_no_response(self.build(authentication)).await
    }
}
