use askama::Template;
use axum::response::IntoResponse;

use super::HtmlTemplate;
use crate::errors::AppError;

#[derive(Template)]
#[template(path = "about.html.j2")]
struct AboutTemplate {}

pub async fn about_handler() -> Result<impl IntoResponse, AppError> {
    let template = AboutTemplate {};
    Ok(HtmlTemplate(template))
}
