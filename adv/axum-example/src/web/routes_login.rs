use crate::{web, Error, Result};
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: Implement real db auth logic here

    if payload.username != "lokesh" || payload.pwd != "pass" {
        return Err(Error::LoginFail);
    }

    // NOTE: Could implement a real auth-token generation/signature
    // instead of hardcoding, but this is just for example
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    // create success body
    let body = Json(json!({
        "result":{
        "success":true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
