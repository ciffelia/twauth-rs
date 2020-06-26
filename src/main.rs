mod app;
mod config;
mod handler;

use actix_web::{middleware, App, HttpServer};
use app::AppState;
use config::Config;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let config = envy::from_env::<Config>().unwrap();

    let addr = format!("0.0.0.0:{}", config.port);
    let consumer_token =
        egg_mode::KeyPair::new(config.twitter_consumer_key, config.twitter_consumer_secret);
    let callback_url = config.twitter_callback_url;

    println!("Starting server on http://127.0.0.1:{}/", config.port);

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                twitter_consumer_token: consumer_token.clone(),
                twitter_callback_url: callback_url.clone(),
            })
            .wrap(middleware::Logger::default())
            .configure(handler::config)
    })
    .bind(addr)?
    .run()
    .await
}
