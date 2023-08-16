use actix_web::{
    get,
    web::{Data, ServiceConfig},
    HttpResponse, Responder,
};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use shuttle_shared_db::Postgres;
use sqlx::Executor;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/version")]
async fn get_version(db: Data<sqlx::PgPool>) -> impl Responder {
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(db.get_ref())
        .await;
    match result {
        Ok(version) => HttpResponse::Ok().body(version),
        Err(e) => HttpResponse::InternalServerError().body(format!("ERROR: {e:?}")),
    }
}

#[shuttle_runtime::main]
async fn actix_web(
    #[Postgres] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let pool = Data::new(pool);

    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(pool).service(hello_world).service(get_version);
    };

    Ok(config.into())
}
