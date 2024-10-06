
use std::{fs::{self, File}, io::Write};

use axum::extract::Path;
use tracing::error;

use crate::error::CustomError;

pub fn store_image_in_filesystem(file_content: &[u8]) -> Result<String, CustomError> {
    let filename = uuid::Uuid::new_v4().to_string();

    let mut file = File::create(format!("images/{}.png", filename)).map_err(|_| CustomError::InternalServerError)?;
    file.write_all(&file_content).map_err(|_| CustomError::InternalServerError)?;
    Ok(filename)
}


pub async fn persist_image_from_url(url: &str) -> Result<String, CustomError> {
    let image_content = reqwest::get(url).await.map_err(|err| {
        error!("Error downloading image from URL: {}", err);
        CustomError::ImageNotFound})?.bytes().await.map(|bytes| bytes.to_vec()).map_err(|_| CustomError::InternalServerError)?;
    store_image_in_filesystem(&image_content)
}

pub async fn get_image(Path(uuid):Path<String>) -> Result<Vec<u8>, CustomError> {
    let image = fs::read(format!("images/{}.png", uuid)).map_err(|_| CustomError::BadRequest)?;
    Ok(image)
}

pub fn drop_image(uuid: &str) -> Result<(), CustomError> {
    fs::remove_file(format!("images/{}.png", uuid)).map_err(|_| CustomError::InternalServerError)
}

#[cfg(test)]
mod tests{
    #[tokio::test]
    async fn test_store_image_in_filesystem() {

        let file_content = b"test image content";
        let result = super::store_image_in_filesystem(file_content);
        println!("{:?}", result);
        assert!(result.is_ok());

        let filename = result.unwrap();
        let file_path = format!("images/{}.png", filename);
        assert!(std::path::Path::new(&file_path).exists());
        assert!(std::fs::read(&file_path).unwrap() == file_content);

    }

    #[tokio::test]
    async fn test_persist_image_from_url() {
        let url = "https://via.placeholder.com/150";
        let result = super::persist_image_from_url(url).await;
        assert!(result.is_ok());

        // Check if the image was downloaded and stored in the filesystem
        let filename = result.unwrap();
        let file_path = format!("images/{}.png", filename);
        assert!(std::path::Path::new(&file_path).exists());

        //check if the file has the same content as the image from the URL
        let image_content = reqwest::get(url).await.unwrap().bytes().await.unwrap();
        assert!(std::fs::read(&file_path).unwrap() == image_content);
    }

    #[tokio::test]
    async fn test_get_image() {
        let url = "https://via.placeholder.com/150";
        let filename = super::persist_image_from_url(url).await.unwrap();
        let result = super::get_image(axum::extract::Path(filename)).await.unwrap();
        assert!(result == reqwest::get(url).await.unwrap().bytes().await.unwrap().to_vec());
    }

    #[tokio::test]
    async fn test_drop_image() {
        let image_bytes = b"test image content";
        let filename = super::store_image_in_filesystem(image_bytes).unwrap();
        let result = super::drop_image(&filename);
        assert!(result.is_ok());
        assert!(!std::path::Path::new(&format!("images/{}.png", filename)).exists()); 
    }

}