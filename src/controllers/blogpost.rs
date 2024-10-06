use axum::{Extension, Json};
use reqwest::StatusCode;

use crate::{error::CustomError};
use crate::models::blogpost::{Blogpost, BlogpostRequest};

pub async fn get_all(Extension(pool): Extension<sqlx::SqlitePool>) -> Result<(StatusCode,Json<Vec<Blogpost>>), CustomError> {
    let sql = "SELECT * FROM BLOGPOST";
    
    let blogposts = sqlx::query_as::<_, Blogpost>(sql)
        .fetch_all(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError);

    Ok((StatusCode::OK,Json(blogposts?)))
}

pub async fn create_new_blogpost(
    payload: Blogpost,
    Extension(pool): Extension<sqlx::SqlitePool>,
) -> Blogpost {
    let sql = "INSERT INTO BLOGPOST (content, username, created_at, user_image_uuid, post_image_uuid) VALUES (?, ?, ?, ?, ?)";
    sqlx::query(sql)
        .bind(&payload.content)
        .bind(&payload.username)
        .bind(&payload.created_at)
        .bind(&payload.user_image_uuid)
        .bind(&payload.post_image_uuid)
        .execute(&pool)
        .await
        .unwrap();

   payload
}

pub async fn process_create_blogpost_request(pool: Extension<sqlx::SqlitePool>, Json(payload): Json<BlogpostRequest>) -> Result<(StatusCode,Json<Blogpost>), CustomError> {
    let post_image_uuid = payload
        .post_image
        .as_ref()
        .map(|base64_encoded_file| {
            store_image_in_filesystem(&BASE64_STANDARD.decode(base64_encoded_file).unwrap())
        })
        .transpose()?;

    let user_image_uuid = OptionFuture::from(payload
        .user_image_url
        .as_ref()
        .map(|url| persist_image_from_url(url)))
        .await
        .transpose()?;

    let blogpost = Blogpost {
        id: 0,
        content: payload.content,
        username: payload.username,
        created_at: chrono::Utc::now().to_rfc3339(),
        user_image_uuid,
        post_image_uuid,
    };

    let result = create_new_blogpost(blogpost, pool).await;
    Ok((StatusCode::CREATED,Json(result)))
}

#[cfg(test)]
mod tests {
    use axum::Extension;
    use reqwest::StatusCode;
    use sqlx::SqlitePool;
    
    #[sqlx::test]
    async fn test_get_all_blogposts(pool: SqlitePool) {
        let response = super::get_all(Extension(pool)).await.unwrap();
        assert_eq!(response.0, StatusCode::OK);
        assert!(response.1 .0.is_empty());
    }
}