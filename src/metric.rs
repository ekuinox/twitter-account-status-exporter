use crate::twitter::{TwitterClient, UserResponse, UserResponseErrorTitle};
use cached::proc_macro::cached;
use chrono::Utc;
use futures::future::join_all;

#[derive(Debug)]
#[repr(u8)]
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

#[cached(time = 60)]
pub async fn get_metric(client: TwitterClient, usernames: Vec<String>) -> String {
    async fn get_status(username: String, client: &TwitterClient) -> (String, AccountStatus, i64) {
        let status = client
            .get_user(&username)
            .await
            .map(From::from)
            .unwrap_or_default();
        let time = Utc::now().timestamp_millis();
        (username, status, time)
    }

    let metrics = join_all(
        usernames
            .into_iter()
            .map(|username| get_status(username, &client)),
    )
    .await;

    let metrics = metrics.into_iter().fold(
        "# HELP twitter twitter_account_status\n# TYPE twitter summary\n".to_string(),
        |mut acc, (username, status, time)| {
            let metric = format!(
                "twitter{{account=\"{}\"}} {:?} {}\n",
                username, status as u8, time
            );
            acc.push_str(&metric);
            acc
        },
    );

    metrics
}
