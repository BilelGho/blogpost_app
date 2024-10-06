use axum::{Extension, Json};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use futures::future::OptionFuture;
use reqwest::StatusCode;

use crate::image_service::{drop_image, persist_image_from_url, store_image_in_filesystem};
use crate::error::CustomError;
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
) -> Result<Blogpost, CustomError> {
    let sql = "INSERT INTO BLOGPOST (content, username, created_at, user_image_uuid, post_image_uuid) VALUES (?, ?, ?, ?, ?)";
    sqlx::query(sql)
        .bind(&payload.content)
        .bind(&payload.username)
        .bind(&payload.created_at)
        .bind(&payload.user_image_uuid)
        .bind(&payload.post_image_uuid)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

   Ok(payload)
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

    let result = create_new_blogpost(blogpost, pool).await?;
    Ok((StatusCode::CREATED,Json(result)))
}

#[cfg(test)]
mod tests {
    use axum::{Extension, Json};
    use reqwest::StatusCode;
    use sqlx::SqlitePool;

    use crate::models::blogpost::{Blogpost, BlogpostRequest};
    
    #[sqlx::test]
    async fn test_get_all_blogposts(pool: SqlitePool) {
        let response = super::get_all(Extension(pool)).await.unwrap();
        assert_eq!(response.0, StatusCode::OK);
        assert!(response.1 .0.is_empty());
    }

    #[sqlx::test]
    async fn test_create_new_blogpost(pool: SqlitePool) {
        let content = "Test content";
        let username = "Test user";

        let blogpost = Blogpost {
            id: 0,
            content: content.to_string(),
            username: username.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            user_image_uuid: None,
            post_image_uuid: None,
        };
        let response = super::create_new_blogpost(blogpost, Extension(pool.clone())).await.unwrap();
        assert_eq!(response.content, content);
        assert_eq!(response.username, username);

        let all_blogposts = sqlx::query_as::<_, Blogpost>("SELECT * FROM BLOGPOST")
            .fetch_all(&pool)
            .await
            .unwrap();
        assert_eq!(all_blogposts.len(), 1);
        assert_eq!(all_blogposts[0].content, content);
    }

    #[sqlx::test]
    async fn test_process_create_blogpost_request(pool: SqlitePool) {
        let payload = BlogpostRequest {
            content: "Test content".to_string(),
            username: "Test user".to_string(),
            post_image: None,
            user_image_url: None,
        };
        let (status_code, Json(blogpost)) = super::process_create_blogpost_request(Extension(pool), Json(payload)).await.unwrap();
        assert_eq!(status_code, StatusCode::CREATED);
        assert_eq!(blogpost.content, "Test content");
    }


    
}