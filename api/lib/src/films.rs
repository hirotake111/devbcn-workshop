use actix_web::{web, HttpResponse};
use shared::models::{CreateFilm, Film};
use uuid::Uuid;

use crate::film_repository::FilmRepository;

pub fn service<R: FilmRepository>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/films")
            .route("", web::get().to(get_all::<R>))
            .route("/{film_id}", web::get().to(get::<R>))
            .route("", web::post().to(create::<R>))
            .route("", web::put().to(update::<R>))
            .route("/{film_id}", web::delete().to(delete::<R>)),
    );
}

async fn get_all<R: FilmRepository>(repository: web::Data<R>) -> HttpResponse {
    match repository.get_films().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(err) => {
            tracing::info!("Internal Server Error: {}", err);
            HttpResponse::NotFound().body("Not Found")
        }
    }
}

async fn get<R: FilmRepository>(
    film_id: web::Path<Uuid>,
    repository: web::Data<R>,
) -> HttpResponse {
    tracing::info!("Getting film: {film_id}");
    match repository.get_film(&film_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(err) => {
            tracing::info!("Internal Server Error: {}", err);
            HttpResponse::NotFound().body("Not Found")
        }
    }
}

async fn create<R: FilmRepository>(
    film: web::Json<CreateFilm>,
    repository: web::Data<R>,
) -> HttpResponse {
    tracing::info!("Creating film: {:?}", film);
    match repository.create_film(&film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {err}")),
    }
}

async fn update<R: FilmRepository>(
    film: web::Json<Film>,
    repository: web::Data<R>,
) -> HttpResponse {
    tracing::info!("Updating film: {film:?}");
    match repository.update_film(&film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {err}")),
    }
}

async fn delete<R: FilmRepository>(id: web::Path<Uuid>, repository: web::Data<R>) -> HttpResponse {
    tracing::info!("Deleting film: {id}");
    match repository.delete_film(&id).await {
        _ => HttpResponse::NoContent().finish(),
    }
}
