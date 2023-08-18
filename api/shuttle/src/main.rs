use actix_web::web;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_shared_db::Postgres;

#[shuttle_runtime::main]
async fn actix_web(
    #[Postgres] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    // let pool = Data::new(pool);
    let film_repository = api_lib::film_repository::PostgresFilmRepository::new(pool);
    let film_repository = web::Data::new(film_repository);

    // pool.execute(include_str!("../../db/schema.sql"))
    // .await
    // .map_err(CustomError::new)?;

    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.app_data(film_repository)
            .configure(api_lib::health::service)
            .configure(api_lib::films::service);
    };

    Ok(config.into())
}
