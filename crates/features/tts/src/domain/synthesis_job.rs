use uuid::Uuid;

/// A single TTS-ready text chunk served to the client.
/// The client feeds this directly to the TTS provider.
#[derive(Debug, Clone)]
pub struct TextChunk {
    pub book_file_id: Uuid,
    pub chapter: i32,
    pub sequence: i32,
    pub text: String,
    pub char_count: i32,
}
