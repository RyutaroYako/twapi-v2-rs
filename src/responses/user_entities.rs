use crate::responses::{user_url::UserUrl, description::Description};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserEntities {
    pub url: Option<UserUrl>, 
    pub description: Option<Description>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl UserEntities {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.url.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true) &&
        self.description.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true);
        if !res {
          println!("UserEntities {:?}", self.extra);
        }
        res
    }
}
