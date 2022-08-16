use async_fs;
use chrono::NaiveDate;
use futures_lite::stream::StreamExt;

use self::meta::Meta;
use self::parsers::{markdown_parser, meta_parser};
use crate::errors::AppError;

pub mod meta;
pub mod parsers;

const BLOGEXTENSION: &str = "md";
const BLOGFOLDER: &str = "blog_posts";

pub async fn get_file_contents(file_name: &str) -> Result<String, AppError> {
    let file_content =
        async_fs::read_to_string(format!("./{BLOGFOLDER}/{}.{BLOGEXTENSION}", file_name)).await?;

    Ok(file_content)
}

pub async fn get_meta_and_markdown(file_name: &str) -> Result<(Meta, String), AppError> {
    let file_contents = get_file_contents(file_name).await?;
    let meta = meta_parser(&file_contents, file_name).await?;
    let mark = markdown_parser(&file_contents).await?;

    Ok((meta, mark))
}

pub async fn get_blog_index_vec() -> Result<Vec<Meta>, AppError> {
    let mut meta_vec: Vec<Meta> = vec![];

    let mut files = async_fs::read_dir(format!("./{BLOGFOLDER}")).await?;

    while let Some(file) = files.next().await {
        let file_name = &file
            .unwrap()
            .file_name()
            .to_str()
            .unwrap()
            .to_string()
            .split(format!(".{BLOGEXTENSION}").as_str())
            .collect::<Vec<&str>>()[0]
            .to_string();
        meta_vec.push(meta_parser(&get_file_contents(&file_name).await?, &file_name).await?);
    }

    meta_vec.sort_by(|a, b| {
        let a_date = NaiveDate::parse_from_str(&a.date, "%B %d, %Y").unwrap();
        let b_date = NaiveDate::parse_from_str(&b.date, "%B %d, %Y").unwrap();
        b_date.cmp(&a_date)
    });

    Ok(meta_vec)
}

pub async fn get_series_index_vec() -> Result<Vec<String>, AppError> {
    let mut series_vec: Vec<String> = vec![];

    let meta_vec = get_blog_index_vec().await?;
    meta_vec.iter().for_each(|meta| {
        if meta.series != "" {
            series_vec.push(meta.series.to_string())
        }
    });

    series_vec.sort();
    series_vec.dedup();

    Ok(series_vec)
}

pub async fn get_meta_by_series_vec(series: &str) -> Result<Vec<Meta>, AppError> {
    let series_meta: Vec<Meta>;
    let meta_vec = get_blog_index_vec().await?;

    series_meta = meta_vec
        .into_iter()
        .filter(|meta| meta.series == series)
        .collect();

    if series_meta.len() == 0 {
        Err(AppError::Empty("empty".to_string()))
    } else {
        Ok(series_meta)
    }
}

pub async fn get_meta_by_category_vec(category: &str) -> Result<Vec<Meta>, AppError> {
    let category_meta: Vec<Meta>;
    let meta_vec = get_blog_index_vec().await?;

    category_meta = meta_vec
        .into_iter()
        .filter(|meta| meta.tags.contains(&category.to_string()))
        .collect();

    if category_meta.len() == 0 {
        Err(AppError::Empty("empty".to_string()))
    } else {
        Ok(category_meta)
    }
}
