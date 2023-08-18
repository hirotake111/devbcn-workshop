use actix_web::{web, HttpResponse};

pub const API_VERSION: &str = "v0.0.1";

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

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health));
}

async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", API_VERSION))
        .append_header(("health-check", "OK"))
        .finish()
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;

    use super::*;

    #[actix_rt::test]
    async fn health_check_works() {
        let res = health().await;

        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);

        let data = res
            .headers()
            .get("health-check")
            .and_then(|h| h.to_str().ok());

        assert_eq!(data, Some("OK"));

        let data = res.headers().get("version").and_then(|h| h.to_str().ok());

        assert_eq!(data, Some(API_VERSION));
    }
}
