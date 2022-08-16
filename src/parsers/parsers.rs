use comrak::{markdown_to_html, ComrakOptions};

use super::meta::Meta;
use crate::errors::AppError;

pub async fn markdown_parser(file_contents: &String) -> Result<String, AppError> {
    let mut options = ComrakOptions::default();
    options.extension.autolink = true;
    options.extension.header_ids = Some(String::from(""));
    options.extension.front_matter_delimiter = Some("---".to_owned());
    options.render.unsafe_ = true;

    Ok(markdown_to_html(&file_contents, &options))
}

pub async fn meta_parser(file_contents: &String, file_name: &str) -> Result<Meta, AppError> {
    Ok(Meta::new(&file_contents, file_name))
}
