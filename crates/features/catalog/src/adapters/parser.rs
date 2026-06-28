use async_trait::async_trait;
use std::io::Cursor;
use std::sync::Arc;
use storage::StorageService;

use crate::application::ports::{FileParser, ParsedBook, ParsedChunk};
use crate::domain::book_file::BookFormat;

// Target chunk size in characters stays within ElevenLabs
const CHUNK_TARGET_CHARS: usize = 900;

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

// PDF
fn parse_pdf(bytes: Vec<u8>) -> anyhow::Result<ParsedBook> {
    let doc =
        lopdf::Document::load_mem(&bytes).map_err(|e| anyhow::anyhow!("pdf parse error: {e}"))?;

    let page_count = doc.get_pages().len() as i32;
    let (title, author) = extract_pdf_metadata(&doc);

    // Extract text page-by-page then normalize and chunk.
    // PDFs have no reliable chapter structure so everything is chapter 0.
    let page_numbers: Vec<u32> = (1..=page_count as u32).collect();
    let raw_pages = doc.extract_text_chunks(&page_numbers);

    let mut all_paragraphs: Vec<String> = Vec::new();
    for page_result in raw_pages.into_iter().flatten() {
        let normalized = normalize_for_tts(&page_result);
        all_paragraphs.extend(split_paragraphs(&normalized));
    }

    let chunks = build_chunks(0, &all_paragraphs);

    Ok(ParsedBook {
        title,
        author,
        page_count: Some(page_count),
        language: None,
        cover_bytes: None,
        chunks,
    })
}

fn extract_pdf_metadata(doc: &lopdf::Document) -> (Option<String>, Option<String>) {
    let info_ref = match doc.trailer.get(b"Info") {
        Ok(r) => r,
        Err(_) => return (None, None),
    };
    let obj_id = match info_ref.as_reference() {
        Ok(id) => id,
        Err(_) => return (None, None),
    };
    let info = doc.get_object(obj_id).ok().and_then(|o| o.as_dict().ok());

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

// EPUB

fn parse_epub(bytes: Vec<u8>) -> anyhow::Result<ParsedBook> {
    let cursor = Cursor::new(bytes);
    let mut doc = epub::doc::EpubDoc::from_reader(cursor)
        .map_err(|e| anyhow::anyhow!("epub parse error: {e}"))?;

    let title = doc.mdata("title").map(|m| m.value.clone());
    let author = doc.mdata("creator").map(|m| m.value.clone());
    let language = doc.mdata("language").map(|m| m.value.clone());
    let cover_bytes = doc.get_cover().map(|(data, _mime)| data);

    let spine_len = doc.spine.len();
    let mut all_chunks: Vec<ParsedChunk> = Vec::new();

    for chapter_idx in 0..spine_len {
        if let Some((html, _mime)) = doc.get_current_str() {
            let plain = strip_html_tags(&html);
            let normalized = normalize_for_tts(&plain);
            let paragraphs = split_paragraphs(&normalized);
            all_chunks.extend(build_chunks(chapter_idx as i32, &paragraphs));
        }
        doc.go_next();
    }

    Ok(ParsedBook {
        title,
        author,
        page_count: None,
        language,
        cover_bytes,
        chunks: all_chunks,
    })
}

// HTML stripping

/// Minimal HTML tag stripper — no external deps required.
fn strip_html_tags(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;

    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                result.push(' '); // space where block tags create implicit breaks
            }
            _ if !in_tag => result.push(c),
            _ => {}
        }
    }

    result
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&nbsp;", " ")
        .replace("&#160;", " ")
}

// ── Text normalization ────────────────────────────────────────────────────────

fn normalize_for_tts(raw: &str) -> String {
    // Fix ligatures and smart quotes (common PDF encoding artifacts)
    let text = raw
        .replace('\u{FB01}', "fi") // ﬁ
        .replace('\u{FB02}', "fl") // ﬂ
        .replace('\u{FB00}', "ff") // ﬀ
        .replace('\u{FB03}', "ffi") // ﬃ
        .replace('\u{FB04}', "ffl") // ﬄ
        .replace('\u{2019}', "'") // right single quotation mark
        .replace('\u{2018}', "'") // left single quotation mark
        .replace('\u{201C}', "\"") // left double quotation mark
        .replace('\u{201D}', "\""); // right double quotation mark

    let text = dehyphenate(&text);
    let text = strip_page_numbers(&text);
    collapse_whitespace(&text)
}

/// "to-\nward" → "toward": drop hyphen + newline when next char is lowercase.
fn dehyphenate(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        if chars[i] == '-' {
            let mut j = i + 1;
            while j < len && chars[j] == ' ' {
                j += 1;
            }
            if j < len && chars[j] == '\n' {
                j += 1;
                while j < len && chars[j] == ' ' {
                    j += 1;
                }
                if j < len && chars[j].is_lowercase() {
                    i = j; // drop hyphen + surrounding whitespace + newline
                    continue;
                }
            }
        }
        result.push(chars[i]);
        i += 1;
    }

    result
}

/// Drop lines that are only digits (page numbers mid-text).
fn strip_page_numbers(text: &str) -> String {
    text.lines()
        .filter(|line| {
            let t = line.trim();
            t.is_empty() || !t.chars().all(|c| c.is_ascii_digit())
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Collapse runs of spaces/tabs; preserve paragraph breaks (double newline).
fn collapse_whitespace(text: &str) -> String {
    text.split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|l| l.split_whitespace().collect::<Vec<_>>().join(" "))
                .filter(|l| !l.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .filter(|block| !block.is_empty())
        .collect::<Vec<_>>()
        .join("\n\n")
}

// ── Chunking ──────────────────────────────────────────────────────────────────

fn split_paragraphs(text: &str) -> Vec<String> {
    text.split("\n\n")
        .map(|s| s.trim().to_owned())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Group paragraphs into ~CHUNK_TARGET_CHARS chunks without splitting mid-paragraph.
fn build_chunks(chapter: i32, paragraphs: &[String]) -> Vec<ParsedChunk> {
    let mut chunks: Vec<ParsedChunk> = Vec::new();
    let mut current = String::new();
    let mut sequence = 0;

    for para in paragraphs {
        if !current.is_empty() && current.len() + para.len() + 2 > CHUNK_TARGET_CHARS {
            flush_chunk(&mut chunks, &mut current, chapter, &mut sequence);
        }

        if !current.is_empty() {
            current.push_str("\n\n");
        }
        current.push_str(para);

        // Single oversized paragraph gets its own chunk immediately.
        if current.len() >= CHUNK_TARGET_CHARS {
            flush_chunk(&mut chunks, &mut current, chapter, &mut sequence);
        }
    }

    if !current.trim().is_empty() {
        flush_chunk(&mut chunks, &mut current, chapter, &mut sequence);
    }

    chunks
}

fn flush_chunk(
    chunks: &mut Vec<ParsedChunk>,
    current: &mut String,
    chapter: i32,
    sequence: &mut i32,
) {
    let text = current.trim().to_owned();
    let char_count = text.chars().count() as i32;
    chunks.push(ParsedChunk {
        chapter,
        sequence: *sequence,
        text,
        char_count,
    });
    *sequence += 1;
    current.clear();
}
