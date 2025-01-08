use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Income {
    pub id: i32,
    pub description: String,
    pub amount: f64,
    pub date: String,
    pub category: String,
    pub user_id: i32,
}