use crate::twitter::{TwitterClient, UserResponse, UserResponseErrorTitle};
use cached::proc_macro::cached;
use chrono::Utc;

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
    let mut response = String::with_capacity(usernames.len() * 12);
    let time = Utc::now().timestamp_millis();
    response += "# HELP twitter twitter_account_status\n";
    response += "# TYPE twitter summary\n";
    for username in usernames {
        let status: AccountStatus = client
            .get_user(&username)
            .await
            .map(From::from)
            .unwrap_or_default();
        response += &format!(
            "twitter{{account=\"{}\"}} {:?} {}\n",
            username, status as u8, time
        );
    }
    response
}
