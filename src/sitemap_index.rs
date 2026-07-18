use std::fmt::Write as _;
use std::path::Path;

use crate::error::SitemapError;
use crate::sitemap_writer::write_file;

/// Represents a single sitemap entry in a sitemap index.
///
/// A sitemap index is used when you have more than 50,000 URLs or your sitemap
/// exceeds 50MB. It references multiple sitemap files.
///
/// # Examples
///
/// ## Creating with all fields
///
/// ```rust
/// use sitemap_writer::SitemapIndex;
///
/// let sitemap = SitemapIndex {
///     loc: "https://example.com/sitemap1.xml".to_string(),
///     lastmod: Some("2024-01-15".to_string()),
/// };
/// ```
///
/// ## Creating with only the URL
///
/// ```rust
/// use sitemap_writer::SitemapIndex;
///
/// let sitemap = SitemapIndex::new("https://example.com/sitemap1.xml");
/// ```
#[derive(Debug, Clone, Default)]
pub struct SitemapIndex {
    /// The URL of the sitemap file. This is a required field.
    ///
    /// Special characters like `<`, `>`, `&` will be automatically escaped.
    pub loc: String,

    /// The date of last modification of the sitemap file.
    ///
    /// Should be in W3C Datetime format (e.g., `2024-01-15` or `2024-01-15T12:00:00+00:00`).
    pub lastmod: Option<String>,
}

impl SitemapIndex {
    /// Creates a new `SitemapIndex` with only the URL specified.
    ///
    /// The `lastmod` field will be `None`.
    ///
    /// # Arguments
    ///
    /// * `loc` - The URL of the sitemap file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sitemap_writer::SitemapIndex;
    ///
    /// let sitemap = SitemapIndex::new("https://example.com/sitemap1.xml");
    /// assert_eq!(sitemap.loc, "https://example.com/sitemap1.xml");
    /// assert!(sitemap.lastmod.is_none());
    /// ```
    pub fn new(loc: impl Into<String>) -> SitemapIndex {
        SitemapIndex {
            loc: loc.into(),
            ..SitemapIndex::default()
        }
    }

    /// Sets the date of last modification, in W3C Datetime format.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sitemap_writer::SitemapIndex;
    ///
    /// let sitemap = SitemapIndex::new("https://example.com/sitemap1.xml")
    ///     .lastmod("2024-01-15");
    /// assert_eq!(sitemap.lastmod.as_deref(), Some("2024-01-15"));
    /// ```
    pub fn lastmod(mut self, lastmod: impl Into<String>) -> SitemapIndex {
        self.lastmod = Some(lastmod.into());
        self
    }
}

/// A writer for generating XML sitemap index files.
///
/// Use this when your site has more than 50,000 URLs or when you want to
/// organize sitemaps by category (e.g., products, blog posts, pages).
///
/// # Examples
///
/// ## Writing to a file
///
/// ```rust,no_run
/// use sitemap_writer::{SitemapIndexWriter, SitemapIndex};
///
/// let result = SitemapIndexWriter::make("sitemap_index.xml", vec![
///     SitemapIndex::new("https://example.com/sitemap1.xml"),
///     SitemapIndex::new("https://example.com/sitemap2.xml"),
/// ]);
/// ```
///
/// ## Building as a String
///
/// ```rust
/// use sitemap_writer::{SitemapIndexWriter, SitemapIndex};
///
/// let xml = SitemapIndexWriter::build(vec![
///     SitemapIndex {
///         loc: "https://example.com/sitemap1.xml".to_string(),
///         lastmod: Some("2024-01-01".to_string()),
///     },
///     SitemapIndex::new("https://example.com/sitemap2.xml"),
/// ]);
/// assert!(xml.contains("<sitemapindex"));
/// ```
pub struct SitemapIndexWriter {}

impl SitemapIndexWriter {
    /// Creates a sitemap index XML file at the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path where the sitemap index will be written.
    /// * `sitemaps` - The [`SitemapIndex`] entries to include in the index.
    ///
    /// # Errors
    ///
    /// Returns [`SitemapError::FileOpen`] if the file cannot be created, or
    /// [`SitemapError::Write`] if writing fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use sitemap_writer::{SitemapIndexWriter, SitemapIndex};
    ///
    /// let result = SitemapIndexWriter::make("sitemap_index.xml", vec![
    ///     SitemapIndex {
    ///         loc: "https://example.com/sitemap1.xml".to_string(),
    ///         lastmod: Some("2024-01-01".to_string()),
    ///     },
    ///     SitemapIndex::new("https://example.com/sitemap2.xml"),
    /// ]);
    /// assert!(result.is_ok());
    /// ```
    pub fn make(
        path: impl AsRef<Path>,
        sitemaps: impl IntoIterator<Item = SitemapIndex>,
    ) -> Result<(), SitemapError> {
        write_file(path.as_ref(), &Self::build(sitemaps))
    }

    /// Creates a gzip-compressed sitemap index XML file at the specified path.
    ///
    /// The path is used as-is; pass a name ending in `.xml.gz` by convention.
    ///
    /// Requires the `gzip` feature.
    ///
    /// # Errors
    ///
    /// Returns [`SitemapError::FileOpen`] if the file cannot be created, or
    /// [`SitemapError::Write`] if compressing or writing fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use sitemap_writer::{SitemapIndexWriter, SitemapIndex};
    ///
    /// let result = SitemapIndexWriter::make_gzip("sitemap_index.xml.gz", vec![
    ///     SitemapIndex::new("https://example.com/sitemap1.xml.gz"),
    /// ]);
    /// assert!(result.is_ok());
    /// ```
    #[cfg(feature = "gzip")]
    pub fn make_gzip(
        path: impl AsRef<Path>,
        sitemaps: impl IntoIterator<Item = SitemapIndex>,
    ) -> Result<(), SitemapError> {
        crate::gzip::write_gzip(path.as_ref(), &Self::build(sitemaps))
    }

    /// Builds a sitemap index XML string from the provided sitemaps.
    ///
    /// This method is useful when you want to get the XML content without
    /// writing to a file, for example when serving the sitemap index dynamically
    /// from a web server.
    ///
    /// # Arguments
    ///
    /// * `sitemaps` - A vector of [`SitemapIndex`] to include in the index.
    ///
    /// # Returns
    ///
    /// Returns the complete sitemap index XML as a `String`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sitemap_writer::{SitemapIndexWriter, SitemapIndex};
    ///
    /// let xml = SitemapIndexWriter::build(vec![
    ///     SitemapIndex {
    ///         loc: "https://example.com/sitemap1.xml".to_string(),
    ///         lastmod: Some("2024-01-01".to_string()),
    ///     },
    ///     SitemapIndex::new("https://example.com/sitemap2.xml"),
    /// ]);
    ///
    /// // Use with a web framework
    /// // HttpResponse::Ok().content_type("application/xml").body(xml)
    /// ```
    pub fn build(sitemaps: impl IntoIterator<Item = SitemapIndex>) -> String {
        let mut content = String::new();
        content.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        content.push_str(r#"<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);
        for sitemap in sitemaps {
            content.push_str("<sitemap>");
            let _ = write!(
                content,
                "<loc>{}</loc>",
                html_escape::encode_text(sitemap.loc.as_str())
            );
            if let Some(lastmod) = &sitemap.lastmod {
                let _ = write!(content, "<lastmod>{}</lastmod>", lastmod);
            }
            content.push_str("</sitemap>");
        }
        content.push_str("</sitemapindex>");
        content
    }
}
