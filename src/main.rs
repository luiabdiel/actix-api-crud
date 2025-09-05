use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use dotenv::dotenv;
use sqlx::{Pool, Postgres};

mod databases {
    pub mod postgres_connection;
}
mod services;

#[derive(Clone)]
pub struct AppState {
    postgres_client: Pool<Postgres>,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = databases::postgres_connection::start_connection().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                postgres_client: pool.clone(),
            }))
            .service(index)
            .configure(services::users::services::users_routes)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
