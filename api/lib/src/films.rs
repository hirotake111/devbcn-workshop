use actix_web::{web, HttpResponse};
use shared::models::{CreateFilm, Film};
use uuid::Uuid;

use crate::film_repository::FilmRepository;

type Repository = web::Data<Box<dyn FilmRepository>>;

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/films")
            .route("", web::get().to(get_all))
            .route("/{film_id}", web::get().to(get))
            .route("", web::post().to(create))
            .route("", web::put().to(update))
            .route("/{film_id}", web::delete().to(delete)),
    );
}

async fn get_all(repository: Repository) -> HttpResponse {
    match repository.get_films().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(err) => {
            tracing::info!("Internal Server Error: {}", err);
            HttpResponse::NotFound().body("Not Found")
        }
    }
}

async fn get(film_id: web::Path<Uuid>, repository: Repository) -> HttpResponse {
    tracing::info!("Getting film: {film_id}");
    match repository.get_film(&film_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(err) => {
            tracing::info!("Internal Server Error: {}", err);
            HttpResponse::NotFound().body("Not Found")
        }
    }
}

async fn create(film: web::Json<CreateFilm>, repository: Repository) -> HttpResponse {
    tracing::info!("Creating film: {:?}", film);
    match repository.create_film(&film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {err}")),
    }
}

async fn update(film: web::Json<Film>, repository: Repository) -> HttpResponse {
    tracing::info!("Updating film: {film:?}");
    match repository.update_film(&film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {err}")),
    }
}

async fn delete(id: web::Path<Uuid>, repository: Repository) -> HttpResponse {
    tracing::info!("Deleting film: {id}");
    match repository.delete_film(&id).await {
        _ => HttpResponse::NoContent().finish(),
    }
}
