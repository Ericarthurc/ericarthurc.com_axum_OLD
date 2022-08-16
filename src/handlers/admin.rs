use askama::Template;
use axum::{extract, response::IntoResponse, response::Response};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

use super::HtmlTemplate;
use crate::errors::AppError;

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
    username: String,
    password: String,
}

pub async fn admin_login_handler(
    extract::Json(login_payload): extract::Json<LoginInput>,
    cookies: Cookies,
) -> Result<Response, AppError> {
    // println!("{:?}", login_payload);
    if login_payload.password == "password1234" {
        return Ok((StatusCode::OK).into_response());
    }
    Ok((StatusCode::FORBIDDEN).into_response())
}
