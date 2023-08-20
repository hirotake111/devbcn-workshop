use std::path::PathBuf;

use actix_web::web;
use api_lib::film_repository::PostgresFilmRepository;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_shared_db::Postgres;

#[shuttle_runtime::main]
async fn actix_web(
    #[Postgres] pool: sqlx::PgPool,
    #[shuttle_static_folder::StaticFolder(folder = "static")] static_folder: PathBuf,
) -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    let film_repository = PostgresFilmRepository::new(pool);
    // let film_repository: web::Data<Box<dyn FilmRepository>> =
    // web::Data::new(Box::new(film_repository));
    let film_repository = web::Data::new(film_repository);

    // pool.execute(include_str!("../../db/schema.sql"))
    // .await
    // .map_err(CustomError::new)?;

    let config = move |cfg: &mut web::ServiceConfig| {
        // cfg.app_data(film_repository)
        // .configure(api_lib::health::service)
        // .configure(api_lib::films::service::<PostgresFilmRepository>);
        cfg.service(
            web::scope("/api")
                .app_data(film_repository)
                .configure(api_lib::health::service)
                .configure(api_lib::films::service::<PostgresFilmRepository>),
        )
        .service(
            actix_files::Files::new("/", static_folder)
                .show_files_listing()
                .index_file("index.html"),
        );
    };

    Ok(config.into())
}
