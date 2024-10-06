use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Blogpost {
    pub id: i32,
    pub content: String,
    pub username: String,
    pub created_at: String,
    pub user_image_uuid: Option<String>,
    pub post_image_uuid: Option<String>,
}


#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct BlogpostRequest {
    pub content: String,
    pub username: String,
    pub user_image_url: Option<String>,
    pub post_image: Option<String>,
}