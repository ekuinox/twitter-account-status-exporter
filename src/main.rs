mod metric;
mod resources;
mod state;
mod twitter;

use crate::{resources::get_metric, state::State, twitter::TwitterClient};
use actix_web::{web::Data, App, HttpServer};
use anyhow::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv().ok();

    let host_address = std::env::var("HOST_ADDRESS")?;
    let host_port = std::env::var("HOST_PORT")?.parse::<u16>()?;
    let prefix = std::env::var("METRIC_PREFIX").unwrap_or_default();
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
        prefix,
    });

    let _ = HttpServer::new(move || App::new().app_data(state.clone()).service(get_metric))
        .bind((host_address.as_str(), host_port))?
        .run()
        .await?;

    Ok(())
}
