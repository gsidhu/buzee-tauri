use std::io::Read;
use std::path::Path;

use diesel::prelude::*;

/// Returns cached OCR text for a file whose identity matches `file_hash`.
/// Returns `None` when there is no cache hit.
pub fn get_cached_ocr(file_hash: &str, conn: &mut diesel::SqliteConnection) -> Option<String> {
    diesel::sql_query(
        "SELECT text FROM ocr_cache WHERE file_hash = ?1".to_string(),
    )
    .bind::<diesel::sql_types::Text, _>(file_hash)
    .get_result::<OcrCacheRow>(conn)
    .ok()
    .map(|row| row.text)
}

#[derive(QueryableByName)]
struct OcrCacheRow {
    #[diesel(sql_type = diesel::sql_types::Text)]
    text: String,
}

/// Stores a successful OCR result in the cache.
pub fn store_ocr_result(
    file_hash: &str,
    text: &str,
    page_count: i32,
    language: Option<&str>,
    conn: &mut diesel::SqliteConnection,
) {
    let _ = diesel::sql_query(
        "INSERT OR REPLACE INTO ocr_cache (file_hash, text, page_count, language_tag, created_at) VALUES (?1, ?2, ?3, ?4, ?5)"
            .to_string(),
    )
    .bind::<diesel::sql_types::Text, _>(file_hash)
    .bind::<diesel::sql_types::Text, _>(text)
    .bind::<diesel::sql_types::Integer, _>(page_count)
    .bind::<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>(language)
    .bind::<diesel::sql_types::BigInt, _>(chrono::Utc::now().timestamp())
    .execute(conn);
}

/// Computes a fast identity hash for a file: SHA-256 of the first 64 KB of
/// content + file size + modification timestamp.  Fast enough to run on every
/// file without noticeable overhead.
pub fn compute_file_hash(path: &Path) -> std::io::Result<String> {
    use sha2::{Digest, Sha256};

    let meta = std::fs::metadata(path)?;
    let mut hasher = Sha256::new();

    // Hash the first 64 KB of content : enough to distinguish documents
    // without reading the entire file.
    if let Ok(mut file) = std::fs::File::open(path) {
        let mut buf = [0u8; 65536];
        let n = file.read(&mut buf).unwrap_or(0);
        hasher.update(&buf[..n]);
    }

    hasher.update(meta.len().to_le_bytes());
    let mtime = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0);
    hasher.update(mtime.to_le_bytes());

    Ok(format!("{:x}", hasher.finalize()))
}
