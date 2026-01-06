use std::fs::File;
use std::io::Write;

use crate::error::SitemapError;
use crate::sitemap_url::SitemapUrl;

/// A writer for generating XML sitemaps.
///
/// This struct provides methods to create sitemaps either by writing directly
/// to a file or by building a String.
///
/// # Examples
///
/// ## Writing to a file
///
/// ```rust,no_run
/// use sitemap_writer::{SitemapWriter, SitemapUrl};
///
/// let result = SitemapWriter::make("sitemap.xml", vec![
///     SitemapUrl::new("https://example.com/"),
///     SitemapUrl::new("https://example.com/about/"),
/// ]);
/// ```
///
/// ## Building as a String
///
/// ```rust
/// use sitemap_writer::{SitemapWriter, SitemapUrl};
///
/// let xml = SitemapWriter::build(vec![
///     SitemapUrl::new("https://example.com/"),
/// ]);
/// assert!(xml.contains("<loc>https://example.com/</loc>"));
/// ```
pub struct SitemapWriter {}

impl SitemapWriter {
    /// Creates a sitemap XML file at the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path where the sitemap will be written.
    /// * `urls` - A vector of [`SitemapUrl`] to include in the sitemap.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or a [`SitemapError`] if the file cannot
    /// be created or written to.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use sitemap_writer::{SitemapWriter, SitemapUrl, SitemapChangeFreq};
    ///
    /// let result = SitemapWriter::make("sitemap.xml", vec![
    ///     SitemapUrl {
    ///         loc: "https://example.com/".to_string(),
    ///         lastmod: Some("2024-01-01".to_string()),
    ///         changefreq: Some(SitemapChangeFreq::DAILY),
    ///         priority: Some(1.0),
    ///     },
    /// ]);
    /// assert!(result.is_ok());
    /// ```
    pub fn make(path: &str, urls: Vec<SitemapUrl>) -> Result<(), SitemapError> {
        let file = File::create(path);
        if file.is_err() {
            return Err(SitemapError::FileOpen(file.err().unwrap().to_string()));
        }
        let mut file = file.unwrap();
        write_text(&file, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
        write_text(
            &file,
            r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
        )?;

        for url in urls {
            let mut row = "<url>".to_string();
            row += format!("<loc>{}</loc>", html_escape::encode_text(url.loc.as_str())).as_str();
            if url.lastmod.is_some() {
                row += format!("<lastmod>{}</lastmod>", url.lastmod.unwrap_or_default()).as_str();
            }
            if url.changefreq.is_some() {
                row += format!(
                    "<changefreq>{}</changefreq>",
                    url.changefreq.unwrap().to_string()
                )
                .as_str();
            }
            if url.priority.is_some() {
                row += format!("<priority>{}</priority>", url.priority.unwrap()).as_str();
            }

            row += "</url>";
            write_text(&file, row.as_str())?;
        }
        write_text(&file, r#"</urlset> "#)?;
        match file.flush() {
            Ok(_) => Ok(()),
            Err(e) => Err(SitemapError::Write(e.to_string())),
        }
    }

    /// Builds a sitemap XML string from the provided URLs.
    ///
    /// This method is useful when you want to get the XML content without
    /// writing to a file, for example when serving the sitemap dynamically
    /// from a web server.
    ///
    /// # Arguments
    ///
    /// * `urls` - A vector of [`SitemapUrl`] to include in the sitemap.
    ///
    /// # Returns
    ///
    /// Returns the complete sitemap XML as a `String`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sitemap_writer::{SitemapWriter, SitemapUrl, SitemapChangeFreq};
    ///
    /// let xml = SitemapWriter::build(vec![
    ///     SitemapUrl {
    ///         loc: "https://example.com/".to_string(),
    ///         lastmod: Some("2024-01-01".to_string()),
    ///         changefreq: Some(SitemapChangeFreq::WEEKLY),
    ///         priority: Some(0.8),
    ///     },
    ///     SitemapUrl::new("https://example.com/blog/"),
    /// ]);
    ///
    /// // Use with a web framework
    /// // HttpResponse::Ok().content_type("application/xml").body(xml)
    /// ```
    pub fn build(urls: Vec<SitemapUrl>) -> String {
        let mut content = "".to_string();
        content.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        content.push_str(r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);
        for url in urls {
            let mut row = "<url>".to_string();
            row += format!("<loc>{}</loc>", html_escape::encode_text(url.loc.as_str())).as_str();
            if url.lastmod.is_some() {
                row += format!("<lastmod>{}</lastmod>", url.lastmod.unwrap_or_default()).as_str();
            }
            if url.changefreq.is_some() {
                row += format!(
                    "<changefreq>{}</changefreq>",
                    url.changefreq.unwrap().to_string()
                )
                .as_str();
            }
            if url.priority.is_some() {
                row += format!("<priority>{}</priority>", url.priority.unwrap()).as_str();
            }
            row += "</url>";
            content.push_str(&row);
        }

        content.push_str(r#"</urlset> "#);
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
