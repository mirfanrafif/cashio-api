use actix_web::{get, HttpResponse, Responder};

use crate::models::expense::Expense;

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_expenses);
}

#[get("/expenses")]
async fn get_expenses() -> impl Responder {
    let expenses = vec![
        Expense {
            id: 1,
            description: "Expense 1".to_string(),
            amount: 100.0,
            date: "2021-01-01".to_string(),
            category: "Food".to_string(),
            user_id: 1,
        },
        Expense {
            id: 2,
            description: "Expense 2".to_string(),
            amount: 200.0,
            date: "2021-01-02".to_string(),
            category: "Transport".to_string(),
            user_id: 1,
        },
    ];

    HttpResponse::Ok().json(expenses)
}
