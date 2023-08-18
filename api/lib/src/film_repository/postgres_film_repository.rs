use super::*;
use shared::models::{CreateFilm, Film};

pub struct PostgresFilmRepository {
    pool: sqlx::PgPool,
}

impl PostgresFilmRepository {
    fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl FilmRepository for PostgresFilmRepository {
    async fn get_films(&self) -> FilmResult<Vec<Film>> {
        sqlx::query_as::<_, Film>(
            r#"
      SELECT id, title, director, year, poster, created_at, updated_at
      FROM films
      "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_film(&self, id: &Uuid) -> FilmResult<Film> {
        sqlx::query_as::<_, Film>(
            r#"
      SELECT id, title, director, year, poster, created_at, updated_at
      FROM films
      WHERE id = $1
      "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn create_film(&self, film: &CreateFilm) -> FilmResult<Film> {
        sqlx::query_as::<_, Film>(
            r#"
        INSERT INTO films (title, director, year, poster)
        VALUES ($1, $2, $3, $4)
          RETURNING id, title , director, year, poster, created_at, updated_at
      "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
    async fn update_film(&self, film: &Film) -> FilmResult<Film> {
        sqlx::query_as::<_, Film>(
            r#"
            UPDATE films
            SET title = $2, director = $3, year = $4, poster = $5
            WHERE id = $1
            RETURNING id, title , director, year, poster, created_at, updated_at
      "#,
        )
        .bind(film.id)
        .bind(&film.title)
        .bind(&film.director)
        .bind(film.year as i16)
        .bind(&film.poster)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_film(&self, id: &Uuid) -> FilmResult<Uuid> {
        sqlx::query_scalar::<_, Uuid>(
            r#"
        DELETE FROM films
        WHERE id = $1
        RETURNING id
      "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
