use askama::Template;
use axum::{extract, response::IntoResponse, response::Response};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::env;
use tower_cookies::{Cookie, Cookies};

use super::HtmlTemplate;
use crate::{errors::AppError, utilities};

#[derive(Template)]
#[template(path = "admin.html.j2")]
struct AdminTemplate {}

pub async fn admin_handler() -> Result<impl IntoResponse, AppError> {
    let template = AdminTemplate {};
    Ok(HtmlTemplate(template))
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct LoginInput {
    password: String,
    pin: String,
}

pub async fn admin_login_handler(
    extract::Json(login_payload): extract::Json<LoginInput>,
    cookies: Cookies,
) -> Result<Response, AppError> {
    if login_payload.password == env::var("ADMIN_PASSWORD").unwrap()
        && login_payload.pin == env::var("ADMIN_PIN").unwrap()
    {
        let token =
            utilities::jwt::generate_jwt(login_payload.password, login_payload.pin).unwrap();

        let cookie = Cookie::build("token_r", token)
            .expires(None)
            .secure(false)
            .http_only(true)
            .path("/admin")
            .finish();

        cookies.add(cookie);
        return Ok((StatusCode::OK).into_response());
    }
    Ok((StatusCode::UNAUTHORIZED).into_response())
}
