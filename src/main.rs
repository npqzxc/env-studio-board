mod config; mod db; mod models; mod routes; mod seed; mod services { pub mod dashboard; pub mod board; }
use std::sync::{Arc, Mutex};
use axum::{routing::{get, post}, Router};
use tower_http::services::ServeDir;
#[tokio::main]
async fn main() { let conn = db::open().expect("db"); seed::ensure_seed(&conn); let shared = Arc::new(Mutex::new(conn)); let app = Router::new().route("/", get(routes::web::dashboard_page)).route("/boards", get(routes::web::boards_page)).route("/boards/:code", get(routes::web::board_detail_page)).route("/items/new", get(routes::web::new_item_page)).route("/items", post(routes::web::create_item)).route("/api/dashboard", get(routes::api::dashboard)).route("/api/boards", get(routes::api::boards)).route("/api/boards/:code", get(routes::api::board_detail)).nest_service("/static", ServeDir::new("static")).with_state(shared); let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap(); axum::serve(listener, app).await.unwrap(); }
