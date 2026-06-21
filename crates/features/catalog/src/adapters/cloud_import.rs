use std::sync::Arc;
use uuid::Uuid;
use storage::StorageService;
use crate::adapters::consts::{USER_AGENT, DROP_BOX_CONST, DROP_BOX_API_URL, DROP_BOX_CDN_API_URL, GOOGLE_DRIVE_CONST, GOOGLE_DRIVE_UPLOAD_API_URL};

pub struct CloudImporter {
    pub storage: Arc<dyn StorageService>,
    pub http:    reqwest::Client,
}

pub struct ImportedFile {
    pub storage_key: String,
    pub cdn_url:     String,
    pub size_bytes:  i64,
}

impl CloudImporter {

    pub fn new(storage: Arc<dyn StorageService>) -> Self {
        Self {
            storage,
            http: reqwest::Client::builder()
                .user_agent(USER_AGENT.to_string())
                .build()
                .expect("failed to build http client"),
        }
    }

    pub async fn import(
        &self,
        source_url: &str,
        user_id: Uuid,
        file_name: &str,
    ) -> anyhow::Result<ImportedFile> {
        let url = normalize_url(source_url);

        let resp = self.http
            .get(&url)
            .send()
            .await?
            .error_for_status()?;

        let content_type = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream")
            .to_string();

        let bytes = resp.bytes().await?;
        let size_bytes = bytes.len() as i64;

        let ext = file_extension(&content_type, file_name);
        let key = format!("books/{user_id}/{}/{file_name}.{ext}", Uuid::new_v4());

        let cdn_url = self.storage
            .upload(&key, bytes.to_vec(), &content_type)
            .await
            .map_err(|e| anyhow::anyhow!("storage upload failed: {e}"))?;

        Ok(ImportedFile { storage_key: key, cdn_url, size_bytes })
    }
}

fn normalize_url(url: &str) -> String {
    if url.contains(&DROP_BOX_CONST.to_string()) {
        return url
            .replace("dl=0", "dl=1")
            .replace(&DROP_BOX_API_URL.to_string(), &DROP_BOX_CDN_API_URL.to_string());
    }
    // Google Drive
    if url.contains(&GOOGLE_DRIVE_CONST.to_string()) {
        tracing::warn!("Google Drive URLs require OAuth — direct download may fail");
        if let Some(id) = extract_drive_id(url) {
            return format!("{GOOGLE_DRIVE_UPLOAD_API_URL}{id}");
        }
    }
    url.to_string()
}

fn extract_drive_id(url: &str) -> Option<&str> {
    // handles /file/d/{id}/view and /open?id={id}
    if let Some(start) = url.find("/file/d/") {
        let rest = &url[start + 8..];
        return Some(rest.split('/').next().unwrap_or(rest));
    }
    if let Some(start) = url.find("id=") {
        let rest = &url[start + 3..];
        return Some(rest.split('&').next().unwrap_or(rest));
    }
    None
}

fn file_extension<'a>(content_type: &'a str, file_name: &'a str) -> &'a str {
    // prefer extension from file name
    if file_name.ends_with(".epub") { return "epub"; }
    if file_name.ends_with(".pdf")  { return "pdf"; }
    // fall back to content type
    match content_type {
        ct if ct.contains("epub") => "epub",
        ct if ct.contains("pdf")  => "pdf",
        _                         => "bin",
    }
}
