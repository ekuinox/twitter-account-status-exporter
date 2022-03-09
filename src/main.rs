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

async fn get_metric(client: &TwitterClient, usernames: &Vec<String>, prefix: &str) -> String {
    let mut response = String::with_capacity(usernames.len() * 12);
    for username in usernames {
        let status: AccountStatus = client
            .get_user(username)
            .await
            .map(From::from)
            .unwrap_or_default();
        response += &format!("{}{} {:?}\n", prefix, username, status);
    }
    response
}

use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};

#[derive(Debug, Clone)]
struct State {
    pub client: TwitterClient,
    pub usernames: Vec<String>,
    pub prefix: String,
}

async fn metric(state: Data<State>) -> impl Responder {
    HttpResponse::Ok().body(get_metric(&state.client, &state.usernames, &state.prefix).await)
}

#[actix_web::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv().ok();

    let api_key = std::env::var("TWITTER_API_KEY")?;
    let api_secret = std::env::var("TWITTER_API_SECRET")?;
    let usernames = std::env::var("TWITTER_USERNAMES")?;
    let usernames = usernames
        .split(',')
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let client = TwitterClient::with_keys(api_key, api_secret).await?;

    let state = Data::new(State {
        client,
        usernames,
        prefix: "".into(),
    });

    let _ = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(web::resource("/metric").to(metric))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await?;

    Ok(())
}
