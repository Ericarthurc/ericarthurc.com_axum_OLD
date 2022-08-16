use askama::Template;
use axum::{extract::Path, response::IntoResponse};
use std::collections::HashMap;

use super::HtmlTemplate;
use crate::{
    errors::AppError,
    parsers::{get_meta_by_series_vec, get_series_index_vec, meta::Meta},
};

#[derive(Template)]
#[template(path = "series_index.html.j2")]
struct SeriesIndexTemplate {
    series_index: Vec<String>,
}
pub async fn series_index_handler() -> Result<impl IntoResponse, AppError> {
    let series_vec = get_series_index_vec().await?;
    let template = SeriesIndexTemplate {
        series_index: series_vec,
    };
    Ok(HtmlTemplate(template))
}

#[derive(Template)]
#[template(path = "series.html.j2")]
struct SeriesTemplate {
    series: String,
    series_meta: Vec<Meta>,
}
pub async fn series_handler(
    Path(params): Path<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let series = params.get("series").unwrap();

    let series_meta = get_meta_by_series_vec(series).await?;

    let template = SeriesTemplate {
        series: series.clone(),
        series_meta,
    };
    Ok(HtmlTemplate(template))
}
