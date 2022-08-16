use askama::Template;
use axum::{extract::Path, response::IntoResponse};
use std::collections::HashMap;

use super::HtmlTemplate;
use crate::{
    errors::AppError,
    parsers::{get_blog_index_vec, get_meta_and_markdown, meta::Meta},
};

#[derive(Template)]
#[template(path = "index.html.j2")]
struct BlogIndexTemplate {
    blog_index: Vec<Meta>,
}

pub async fn blog_index_handler() -> Result<impl IntoResponse, AppError> {
    let meta_vec = get_blog_index_vec().await?;
    let template = BlogIndexTemplate {
        blog_index: meta_vec,
    };
    Ok(HtmlTemplate(template))
}

#[derive(Template)]
#[template(path = "blog_post.html.j2")]
struct BlogTemplate {
    markdown: String,
    meta: Meta,
}

pub async fn blog_handler(
    Path(params): Path<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let blog = params.get("blog");
    match blog {
        Some(blog) => {
            let (meta, mark) = get_meta_and_markdown(blog).await?;
            let template = BlogTemplate {
                markdown: mark,
                meta,
            };
            Ok(HtmlTemplate(template))
        }
        None => Err(AppError::Param(String::from("missing parameter"))),
    }
}
