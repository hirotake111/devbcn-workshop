use actix_web::{web, HttpResponse};

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

async fn get_all() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn get() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn create() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn update() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn delete() -> HttpResponse {
    HttpResponse::Ok().finish()
}
