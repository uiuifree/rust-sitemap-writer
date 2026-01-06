use std::fmt::{Display, Formatter};

/// Errors that can occur when writing a sitemap.
#[derive(Debug)]
pub enum SitemapError {
    /// Failed to open or create the file.
    FileOpen(String),
    /// Failed to write to the file.
    Write(String),
}

impl Display for SitemapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SitemapError::FileOpen(msg) => write!(f, "Failed to open file: {}", msg),
            SitemapError::Write(msg) => write!(f, "Failed to write: {}", msg),
        }
    }
}

impl std::error::Error for SitemapError {}
