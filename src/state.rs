use crate::twitter::TwitterClient;

#[derive(Debug, Clone)]
pub struct State {
    pub client: TwitterClient,
    pub usernames: Vec<String>,
}
