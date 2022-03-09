mod twitter;

use crate::twitter::*;
use anyhow::Result;

#[derive(Debug)]
enum AccountStatus {
    Ok,
    Suspended,
    NotFound,
    Unknown,
}

impl Default for AccountStatus {
    fn default() -> AccountStatus {
        AccountStatus::Unknown
    }
}

impl From<UserResponse> for AccountStatus {
    fn from(response: UserResponse) -> AccountStatus {
        match response {
            UserResponse::Success { .. } => AccountStatus::Ok,
            UserResponse::Errors(errors) => {
                for error in errors {
                    let status = match error.title {
                        UserResponseErrorTitle::Forbidden => AccountStatus::Suspended,
                        UserResponseErrorTitle::NotFound => AccountStatus::NotFound,
                    };
                    return status;
                }
                AccountStatus::Unknown
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv().ok();

    let api_key = std::env::var("TWITTER_API_KEY")?;
    let api_secret = std::env::var("TWITTER_API_SECRET")?;
    let usernames = std::env::var("TWITTER_USERNAMES")?;
    let usernames = usernames.split(',').into_iter().collect::<Vec<&str>>();

    let client = TwitterClient::with_keys(api_key, api_secret).await?;

    for username in usernames {
        let status: AccountStatus = client
            .get_user(username)
            .await
            .map(From::from)
            .unwrap_or_default();
        println!("{}: {:?}", username, status);
    }

    Ok(())
}
