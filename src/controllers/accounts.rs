use actix_web::{get, HttpResponse, Responder};
use diesel::{query_dsl::methods::SelectDsl, RunQueryDsl, SelectableHelper};

use crate::models::accounts::Account;

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_accounts);
}

#[get("/accounts")]
async fn get_accounts() -> impl Responder {
    use crate::schema::accounts::dsl::*;

    let connection = &mut crate::utils::connection::establish_connection();

    let result = accounts
        .select(Account::as_select())
        .load(connection)
        .expect("Error loading accounts");

    HttpResponse::Ok().json(result)
}
