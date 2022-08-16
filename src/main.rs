use askama::Template;
use axum::{
    handler::Handler,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service, post},
    Router,
};
use dotenv::dotenv;
use std::{env, net::SocketAddr};
use tower_cookies::CookieManagerLayer;
use tower_http::services::{fs::ServeDir, ServeFile};

use crate::handlers::{
    about::about_handler,
    admin::{admin_handler, admin_login_handler},
    blog::{blog_handler, blog_index_handler},
    category::category_handler,
    series::{series_handler, series_index_handler},
    HtmlTemplate,
};

mod errors;
mod handlers;
mod parsers;
mod utilities;

#[tokio::main]
async fn main() {
    // Panic if .env is missing
    dotenv().expect("missing .env file");

    let admin_routes = Router::new()
        .route("/", get(admin_handler))
        .route("/login", post(admin_login_handler));

    let app = Router::new()
        .fallback(handler_404.into_service())
        .route(
            "/sw.js",
            get_service(ServeFile::new("./public/js/sw.js")).handle_error(
                |error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                },
            ),
        )
        .route(
            "/robots.txt",
            get_service(ServeFile::new("./public/robots.txt")).handle_error(
                |error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                },
            ),
        )
        .route(
            "/favicon.ico",
            get_service(ServeFile::new("./public/favicon.ico")).handle_error(
                |error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                },
            ),
        )
        .route("/", get(blog_index_handler))
        .route("/blog/:blog", get(blog_handler))
        .route("/category/:category", get(category_handler))
        .route("/series", get(series_index_handler))
        .route("/series/:series", get(series_handler))
        .route("/about", get(about_handler))
        .nest("/admin", admin_routes)
        .layer(CookieManagerLayer::new())
        .nest(
            "/public",
            get_service(ServeDir::new("./public/")).handle_error(
                |error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                },
            ),
        );

    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        env::var("HTTP_PORT").unwrap().parse::<u16>().unwrap(),
    ));
    println!("Server: {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Template)]
#[template(path = "not_found.html.j2")]
struct NotFoundTemplate {}
async fn handler_404() -> impl IntoResponse {
    let template = NotFoundTemplate {};
    HtmlTemplate(template)
}
