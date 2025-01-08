use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Expense {
    pub id: i32,
    pub amount: f64,
    pub description: String,
    pub date: String,
    pub category: String,
    pub user_id: i32,
}