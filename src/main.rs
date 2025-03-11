use actix_web::{web, App, HttpServer};
use cashio_api::controllers::{accounts_init, expenses_init, incomes_init};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // set prefix route for expenses
            .configure(|_cfg| {
                web::scope("/api")
                    .configure(expenses_init)
                    .configure(incomes_init)
                    .configure(accounts_init);
            })
    })
    .bind("127.0.0.1:8080")?
    .workers(4)
    .run()
    .await
}
