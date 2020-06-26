use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,

    pub twitter_consumer_key: String,
    pub twitter_consumer_secret: String,

    pub twitter_callback_url: String,
}

fn default_port() -> u16 {
    8080
}
