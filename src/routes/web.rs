use std::sync::{Arc, Mutex};
use askama::Template;
use axum::{extract::{Form, Path, State}, response::Html};
use rusqlite::Connection;
use serde::Deserialize;
use crate::models::{Board, WorkItem};
use crate::services::{board, dashboard};
#[derive(Template)] #[template(path = "dashboard.html")] pub struct DashboardTemplate { pub title: String, pub domain: String, pub board_total: i64, pub item_total: i64 }
#[derive(Template)] #[template(path = "boards.html")] pub struct BoardsTemplate { pub title: String, pub label: String, pub boards: Vec<Board> }
#[derive(Template)] #[template(path = "board_detail.html")] pub struct BoardDetailTemplate { pub title: String, pub found: bool, pub board: Option<Board>, pub items: Vec<WorkItem> }
#[derive(Template)] #[template(path = "new_item.html")] pub struct NewItemTemplate { pub title: String, pub label: String, pub item_label: String }
#[derive(Deserialize)] pub struct NewItemForm { pub code:String, pub title:String, pub stage:String, pub priority:String, pub due_date:String, pub details:String }
pub async fn dashboard_page(State(conn): State<Arc<Mutex<Connection>>>) -> Html<String> { let summary = dashboard::load(&conn.lock().unwrap()); Html(DashboardTemplate{title:"Studio Board".into(), domain:"创意制作".into(), board_total:summary.board_total, item_total:summary.item_total}.render().unwrap()) }
pub async fn boards_page(State(conn): State<Arc<Mutex<Connection>>>) -> Html<String> { Html(BoardsTemplate{title:"Studio Board".into(), label:"制作板块".into(), boards: board::list(&conn.lock().unwrap())}.render().unwrap()) }
pub async fn board_detail_page(Path(code): Path<String>, State(conn): State<Arc<Mutex<Connection>>>) -> Html<String> { let guard = conn.lock().unwrap(); let html = if let Some((board_obj, items)) = board::detail(&guard, &code) { BoardDetailTemplate{title:"Studio Board".into(), found:true, board:Some(board_obj), items}.render().unwrap() } else { BoardDetailTemplate{title:"Studio Board".into(), found:false, board:None, items:vec![]}.render().unwrap() }; Html(html) }
pub async fn new_item_page() -> Html<String> { Html(NewItemTemplate{title:"Studio Board".into(), label:"制作板块".into(), item_label:"制作卡".into()}.render().unwrap()) }
pub async fn create_item(State(conn): State<Arc<Mutex<Connection>>>, Form(form): Form<NewItemForm>) -> Html<String> { board::create(&conn.lock().unwrap(), &form.code, &form.title, &form.stage, &form.priority, &form.due_date, &form.details); Html(format!("<meta http-equiv=refresh content="0;url=/boards/{}">", form.code)) }
