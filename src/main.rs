use actix_web::{web, HttpServer};
use cashio_api::controllers::{expenses_init, incomes_init};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        actix_web::App::new()
            // set prefix route for expenses
            .configure(|_cfg| {
                web::scope("/api")
                    .configure(expenses_init)
                    .configure(incomes_init);
            })
    })
    .run()
    .await
}
