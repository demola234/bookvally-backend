use async_trait::async_trait;
use std::io::Cursor;
use std::sync::Arc;
use storage::StorageService;

use crate::application::ports::{FileParser, ParsedBook};
use crate::domain::book_file::BookFormat;

pub struct BookFileParser {
    pub storage: Arc<dyn StorageService>,
}

#[async_trait]
impl FileParser for BookFileParser {
    async fn parse(&self, storage_key: &str, format: BookFormat) -> anyhow::Result<ParsedBook> {
        let bytes = self
            .storage
            .download(storage_key)
            .await
            .map_err(|e| anyhow::anyhow!("download failed: {e}"))?;

        match format {
            BookFormat::Epub => parse_epub(bytes),
            BookFormat::Pdf => parse_pdf(bytes),
        }
    }
}

fn parse_epub(bytes: Vec<u8>) -> anyhow::Result<ParsedBook> {
    let cursor = Cursor::new(bytes);
    let mut doc = epub::doc::EpubDoc::from_reader(cursor)
        .map_err(|e| anyhow::anyhow!("epub parse error: {e}"))?;

    let title = doc.mdata("title").map(|m| m.value.clone());
    let author = doc.mdata("creator").map(|m| m.value.clone());
    let language = doc.mdata("language").map(|m| m.value.clone());

    let cover_bytes = doc.get_cover().map(|(data, _mime)| data);

    Ok(ParsedBook {
        title,
        author,
        page_count: None,
        language,
        cover_bytes,
    })
}

fn parse_pdf(bytes: Vec<u8>) -> anyhow::Result<ParsedBook> {
    let doc =
        lopdf::Document::load_mem(&bytes).map_err(|e| anyhow::anyhow!("pdf parse error: {e}"))?;

    let page_count = doc.get_pages().len() as i32;

    let (title, author) = match doc.trailer.get(b"Info") {
        Ok(info_ref) => {
            let info = doc
                .get_object(info_ref.as_reference()?)
                .ok()
                .and_then(|o| o.as_dict().ok());

            let title = info
                .as_ref()
                .and_then(|d| d.get(b"Title").ok())
                .and_then(|o| o.as_str().ok())
                .map(|s| String::from_utf8_lossy(s).into_owned());

            let author = info
                .as_ref()
                .and_then(|d| d.get(b"Author").ok())
                .and_then(|o| o.as_str().ok())
                .map(|s| String::from_utf8_lossy(s).into_owned());

            (title, author)
        }
        Err(_) => (None, None),
    };

    Ok(ParsedBook {
        title,
        author,
        page_count: Some(page_count),
        language: None,
        cover_bytes: None,
    })
}
