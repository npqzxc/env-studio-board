use std::sync::{Arc, Mutex};
use axum::{extract::{Path, State}, Json};
use rusqlite::Connection;
use serde_json::{json, Value};
use crate::services::{board, dashboard};
pub async fn dashboard(State(conn): State<Arc<Mutex<Connection>>>) -> Json<Value> { let summary = dashboard::load(&conn.lock().unwrap()); Json(json!(summary)) }
pub async fn boards(State(conn): State<Arc<Mutex<Connection>>>) -> Json<Value> { Json(json!({"items": board::list(&conn.lock().unwrap())})) }
pub async fn board_detail(Path(code): Path<String>, State(conn): State<Arc<Mutex<Connection>>>) -> Json<Value> { let guard = conn.lock().unwrap(); if let Some((board, items)) = board::detail(&guard, &code) { Json(json!({"board": board, "items": items})) } else { Json(json!({"error": "not_found"})) } }
