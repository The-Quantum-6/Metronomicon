use axum::{extract::{Path, State}, http::StatusCode, Json};
use sqlx::PgPool;
use super::dto::{Post, NewPost};

async fn list_tests(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<test>>, StatusCode> {
    let tests = sqlx::query_as!::<_, test>(
        "SELECT id, content FROM test",
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(tests))
}

async fn create_test(
    State(pool): State<PgPool>,
    axum::Form(body): axum::Form<newTestContent>,
) -> Result<Redirect, StatusCode> {
    sqlx::query!(
        "INSERT INTO test (content) VALUES ($1)",
    )
    .bind(body.content)
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/"))
}

async fn get_test(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<test>, StatusCode> {
    let single_test = sqlx::query_as!(
        test,
        "SELECT id, content FROM test WHERE id = $1",
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    single_test.map(Json).ok_or(StatusCode::NOT_FOUND)
}

async fn delete_test(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!("DELETE FROM test WHERE id = $1", id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

async fn update_test(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    axum::Form(body): axum::Form<newTestContent>,
) -> Result<Redirect, StatusCode> {
    let result = sqlx::query!("UPDATE test SET content = $1 WHERE id = $2")
    .bind(body.content)
    .bind(body.id)
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
            Err(StatusCode::NOT_FOUND)
        } else {
            Ok(Redirect::to("/"))
        }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(3, 1+2);
  }
}