use actix_web::{get, HttpResponse, Responder};

// #[get("/version")]
// async fn get_version(db: web::Data<sqlx::PgPool>) -> impl Responder {
// tracing::info!("Getting version");
// let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
// .fetch_one(db.get_ref())
// .await;
// match result {
// Ok(version) => HttpResponse::Ok().body(version),
// Err(e) => HttpResponse::InternalServerError().body(format!("ERROR: {e:?}")),
// }
// }

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
        .append_header(("version", "v0.0.1"))
        .finish()
}
