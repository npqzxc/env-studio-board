use rusqlite::Connection;
use serde::Serialize;
#[derive(Serialize)] pub struct DashboardSummary { pub board_total: i64, pub item_total: i64 }
pub fn load(conn: &Connection) -> DashboardSummary { let board_total = conn.query_row("SELECT COUNT(*) FROM boards", [], |row| row.get(0)).unwrap_or(0); let item_total = conn.query_row("SELECT COUNT(*) FROM work_items", [], |row| row.get(0)).unwrap_or(0); DashboardSummary { board_total, item_total } }
