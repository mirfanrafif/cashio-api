use actix_web::{HttpResponse, Responder, get};

use crate::models::income::Income;  

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_incomes);
}

#[get("/incomes")]
async fn get_incomes() -> impl Responder {
    let incomes = vec![
        Income {
            id: 1,
            description: "Income 1".to_string(),
            amount: 100.0,
            date: "2021-01-01".to_string(),
            category: "Salary".to_string(),
            user_id: 1,
        },
        Income {
            id: 2,
            description: "Income 2".to_string(),
            amount: 200.0,
            date: "2021-01-02".to_string(),
            category: "Bonus".to_string(),
            user_id: 1,
        },
    ];

    HttpResponse::Ok().json(incomes)
}