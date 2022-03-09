use crate::twitter::{TwitterClient, UserResponse, UserResponseErrorTitle};

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

pub async fn get_metric(client: &TwitterClient, usernames: &Vec<String>, _prefix: &str) -> String {
    let mut response = String::with_capacity(usernames.len() * 12);
    response += "# HELP twitter twitter_account_status\n";
    response += "# TYPE twitter summary\n";
    for username in usernames {
        let status: AccountStatus = client
            .get_user(username)
            .await
            .map(From::from)
            .unwrap_or_default();
        response += &format!("twitter{{account=\"{}\"}} {:?}\n", username, status as u8);
    }
    response
}
