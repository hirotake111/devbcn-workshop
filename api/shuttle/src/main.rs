use actix_web::web::{Data, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use shuttle_shared_db::Postgres;
use sqlx::Executor;

#[shuttle_runtime::main]
async fn actix_web(
    #[Postgres] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let pool = Data::new(pool);

    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(pool)
            .configure(api_lib::health::service)
            .configure(api_lib::films::service);
    };

    Ok(config.into())
}
