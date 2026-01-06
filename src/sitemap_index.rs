use std::fs::File;
use std::io::Write;

use crate::error::SitemapError;

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
#[derive(Debug, Clone)]
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

impl Default for SitemapIndex {
    fn default() -> Self {
        SitemapIndex {
            loc: "".to_string(),
            lastmod: None,
        }
    }
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
    pub fn new(loc: &str) -> SitemapIndex {
        SitemapIndex {
            loc: loc.to_string(),
            ..SitemapIndex::default()
        }
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
    /// * `sitemaps` - A vector of [`SitemapIndex`] to include in the index.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or a [`SitemapError`] if the file cannot
    /// be created or written to.
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
    pub fn make(path: &str, sitemaps: Vec<SitemapIndex>) -> Result<(), SitemapError> {
        let file = File::create(path);
        if file.is_err() {
            return Err(SitemapError::FileOpen(file.err().unwrap().to_string()));
        }
        let mut file = file.unwrap();
        write_text(&file, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
        write_text(
            &file,
            r#"<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
        )?;

        for sitemap in sitemaps {
            let mut row = "<sitemap>".to_string();
            row += format!(
                "<loc>{}</loc>",
                html_escape::encode_text(sitemap.loc.as_str())
            )
            .as_str();
            if let Some(lastmod) = sitemap.lastmod {
                row += format!("<lastmod>{}</lastmod>", lastmod).as_str();
            }
            row += "</sitemap>";
            write_text(&file, row.as_str())?;
        }
        write_text(&file, r#"</sitemapindex>"#)?;
        match file.flush() {
            Ok(_) => Ok(()),
            Err(e) => Err(SitemapError::Write(e.to_string())),
        }
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
    pub fn build(sitemaps: Vec<SitemapIndex>) -> String {
        let mut content = String::new();
        content.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        content.push_str(r#"<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);

        for sitemap in sitemaps {
            let mut row = "<sitemap>".to_string();
            row += format!(
                "<loc>{}</loc>",
                html_escape::encode_text(sitemap.loc.as_str())
            )
            .as_str();
            if let Some(lastmod) = sitemap.lastmod {
                row += format!("<lastmod>{}</lastmod>", lastmod).as_str();
            }
            row += "</sitemap>";
            content.push_str(&row);
        }

        content.push_str(r#"</sitemapindex>"#);
        content
    }
}

fn write_text(mut file: &File, str: &str) -> Result<(), SitemapError> {
    let f = file.write(str.as_bytes());
    if f.is_err() {
        return Err(SitemapError::Write(f.err().unwrap().to_string()));
    }
    Ok(())
}
