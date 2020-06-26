use super::app::AppState;
use actix_web::{get, http, web, HttpResponse};
use serde_derive::Deserialize;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index).service(callback);
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> HttpResponse {
    let request_token =
        egg_mode::auth::request_token(&data.twitter_consumer_token, &data.twitter_callback_url)
            .await
            .unwrap();

    let auth_url = egg_mode::auth::authorize_url(&request_token);

    HttpResponse::Found()
        .header(http::header::LOCATION, auth_url)
        .finish()
}

#[derive(Deserialize)]
#[serde(untagged)]
enum CallbackQuery {
    Authorized(AuthorizedCallbackQuery),
    Denied(DeniedCallbackQuery),
}

#[derive(Deserialize)]
struct AuthorizedCallbackQuery {
    oauth_token: String,
    oauth_verifier: String,
}

#[derive(Deserialize)]
struct DeniedCallbackQuery {
    #[allow(dead_code)]
    denied: String,
}

#[get("/callback")]
async fn callback(web::Query(query): web::Query<CallbackQuery>) -> HttpResponse {
    match query {
        CallbackQuery::Authorized(authorized_query) => authorized_callback(authorized_query).await,
        CallbackQuery::Denied(denied_query) => denied_callback(denied_query).await,
    }
}

async fn authorized_callback(query: AuthorizedCallbackQuery) -> HttpResponse {
    let dummy_con_token = egg_mode::KeyPair::new("", "");
    let request_token = egg_mode::KeyPair::new(query.oauth_token, "");

    let (token, user_id, screen_name) =
        egg_mode::auth::access_token(dummy_con_token, &request_token, &query.oauth_verifier)
            .await
            .unwrap();

    let access_token = match token {
        egg_mode::auth::Token::Access {
            consumer: _,
            access,
        } => access,
        egg_mode::auth::Token::Bearer(_) => panic!("Access token expected, but got bearer token"),
    };

    let body = format!(
        "User ID: {}\nScreen name: {}\nAccess Token: {}\nAccess Token Secret: {}",
        user_id, screen_name, access_token.key, access_token.secret
    );

    HttpResponse::Ok().body(body)
}

async fn denied_callback(_query: DeniedCallbackQuery) -> HttpResponse {
    HttpResponse::Unauthorized().body("OAuth denied.")
}
