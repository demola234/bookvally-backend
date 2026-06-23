use aws_smithy_types::base64;
use storage::{CloudR2Storage, CloudR2StorageConfig, StorageService};

fn load_env() {
    dotenvy::from_filename(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join(".dev.env"),
    )
    .ok();
}

fn config() -> CloudR2StorageConfig {
    load_env();
    CloudR2StorageConfig {
        endpoint: std::env::var("APP__STORAGE__ENDPOINT").expect("APP__STORAGE__ENDPOINT not set"),
        bucket: std::env::var("APP__STORAGE__BUCKET").expect("APP__STORAGE__BUCKET not set"),
        region: std::env::var("APP__STORAGE__REGION").unwrap_or_else(|_| "auto".into()),
        access_key_id: std::env::var("APP__STORAGE__ACCESS_KEY_ID")
            .expect("APP__STORAGE__ACCESS_KEY_ID not set"),
        secret_access_key: std::env::var("APP__STORAGE__SECRET_ACCESS_KEY")
            .expect("APP__STORAGE__SECRET_ACCESS_KEY not set"),
        public_url: std::env::var("APP__STORAGE__PUBLIC_URL").ok(),
    }
}

#[tokio::test]
async fn test_upload_and_delete() {
    let storage = CloudR2Storage::new(&config()).await.unwrap();

    let key = "test/image.png";
    let base64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAIAAACQd1PeAAAADElEQVQImWNgYGAAAAAEAAGjChXjAAAAAElFTkSuQmCC";

    let bytes = base64::decode(base64).unwrap();

    let cdn_url = storage.upload(key, bytes, "text/plain").await.unwrap();
    println!("cdn_url: {cdn_url}");
    assert!(cdn_url.contains(key));

    storage.delete(key).await.unwrap();
    println!("deleted: {key}");
}

#[tokio::test]
async fn test_presign_upload() {
    let storage = CloudR2Storage::new(&config()).await.unwrap();

    let result = storage
        .presign_upload("test/avatar.jpg", "image/jpeg", 300)
        .await
        .unwrap();

    println!("upload_url: {}", result.upload_url);
    println!("cdn_url:    {}", result.cdn_url);

    assert!(result.upload_url.contains("test/avatar.jpg"));
    assert!(result.cdn_url.contains("test/avatar.jpg"));
}

#[tokio::test]
async fn test_download() {
    let storage = CloudR2Storage::new(&config()).await.unwrap();
    let key = "test/download.txt";

    storage
        .upload(key, b"Hello, world!".to_vec(), "text/plain")
        .await
        .unwrap();
    let bytes = storage.download(key).await.unwrap();

    assert_eq!(bytes, b"Hello, world!");
    storage.delete(key).await.unwrap();
}
