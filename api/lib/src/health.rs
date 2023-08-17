use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/version")]
pub async fn get_version(db: web::Data<sqlx::PgPool>) -> impl Responder {
    tracing::info!("Getting version");
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(db.get_ref())
        .await;
    match result {
        Ok(version) => HttpResponse::Ok().body(version),
        Err(e) => HttpResponse::InternalServerError().body(format!("ERROR: {e:?}")),
    }
}
