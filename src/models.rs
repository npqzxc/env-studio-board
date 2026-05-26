use serde::Serialize;
#[derive(Debug, Clone, Serialize)] pub struct Board { pub code: String, pub name: String, pub summary: String, pub status: String, pub owner: String }
#[derive(Debug, Clone, Serialize)] pub struct WorkItem { pub id: i64, pub board_code: String, pub title: String, pub stage: String, pub priority: String, pub due_date: String }
