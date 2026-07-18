use std::fs::File;
use std::io::Write as _;
use std::path::Path;

use flate2::Compression;
use flate2::write::GzEncoder;

use crate::error::SitemapError;

pub(crate) fn write_gzip(path: &Path, content: &str) -> Result<(), SitemapError> {
    let file = File::create(path).map_err(|e| SitemapError::FileOpen(e.to_string()))?;
    let mut encoder = GzEncoder::new(file, Compression::default());
    encoder
        .write_all(content.as_bytes())
        .map_err(|e| SitemapError::Write(e.to_string()))?;
    encoder
        .finish()
        .map_err(|e| SitemapError::Write(e.to_string()))?;
    Ok(())
}
