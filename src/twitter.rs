use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Twitter Client (App Only)
#[derive(Deserialize, Debug)]
pub struct TwitterClient {
    bearer_token: String,
}

/// `POST oauth2/token` のレスポンス
#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Title {
    #[serde(rename = "Not Found Error")]
    NotFound,

    #[serde(rename = "Forbidden")]
    Forbidden,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum UserResponseErrorTitle {
    #[serde(rename = "Not Found Error")]
    NotFound,

    #[serde(rename = "Forbidden")]
    Forbidden,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserErrorResponse {
    pub value: String,
    pub title: UserResponseErrorTitle,
}

/// `2/users/by/username/:username` のレスポンス
#[derive(Deserialize, Serialize, Debug)]
pub enum UserResponse {
    #[serde(rename = "data")]
    Success {
        id: String,
        name: String,
        username: String,
    },
    #[serde(rename = "errors")]
    Errors(Vec<UserErrorResponse>),
}

impl TwitterClient {
    pub fn new(bearer_token: String) -> TwitterClient {
        TwitterClient { bearer_token }
    }

    /// api_key, api_secret からクライアントを作る
    pub async fn with_keys(api_key: String, api_secret: String) -> Result<TwitterClient> {
        let client = reqwest::Client::new();
        let json = client
            .post("https://api.twitter.com/oauth2/token")
            .basic_auth(api_key, Some(api_secret))
            .query(&[("grant_type", "client_credentials")])
            .send()
            .await?
            .text()
            .await?;
        let response = serde_json::from_str::<TokenResponse>(&json)?;
        Ok(TwitterClient::new(response.access_token))
    }

    /// ユーザを取得する
    pub async fn get_user(&self, username: &str) -> Result<UserResponse> {
        let client = reqwest::Client::new();
        let json = client
            .get(format!(
                "https://api.twitter.com/2/users/by/username/{}",
                username
            ))
            .bearer_auth(&self.bearer_token)
            .send()
            .await?
            .text()
            .await?;
        let response = serde_json::from_str::<UserResponse>(&json)?;
        Ok(response)
    }
}
