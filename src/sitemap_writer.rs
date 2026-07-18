use std::fmt::Write as _;
use std::fs::File;
use std::io::Write as _;
use std::path::Path;

use crate::error::SitemapError;
use crate::extensions::{SitemapNews, SitemapVideo};
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
    /// * `urls` - The [`SitemapUrl`] entries to include in the sitemap.
    ///
    /// # Errors
    ///
    /// Returns [`SitemapError::FileOpen`] if the file cannot be created, or
    /// [`SitemapError::Write`] if writing fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use sitemap_writer::{SitemapWriter, SitemapUrl, SitemapChangeFreq};
    ///
    /// let result = SitemapWriter::make("sitemap.xml", vec![
    ///     SitemapUrl::new("https://example.com/")
    ///         .lastmod("2024-01-01")
    ///         .changefreq(SitemapChangeFreq::DAILY)
    ///         .priority(1.0),
    /// ]);
    /// assert!(result.is_ok());
    /// ```
    pub fn make(
        path: impl AsRef<Path>,
        urls: impl IntoIterator<Item = SitemapUrl>,
    ) -> Result<(), SitemapError> {
        write_file(path.as_ref(), &Self::build(urls))
    }

    /// Creates a gzip-compressed sitemap XML file at the specified path.
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
    /// use sitemap_writer::{SitemapWriter, SitemapUrl};
    ///
    /// let result = SitemapWriter::make_gzip("sitemap.xml.gz", vec![
    ///     SitemapUrl::new("https://example.com/"),
    /// ]);
    /// assert!(result.is_ok());
    /// ```
    #[cfg(feature = "gzip")]
    pub fn make_gzip(
        path: impl AsRef<Path>,
        urls: impl IntoIterator<Item = SitemapUrl>,
    ) -> Result<(), SitemapError> {
        crate::gzip::write_gzip(path.as_ref(), &Self::build(urls))
    }

    /// Builds a sitemap XML string from the provided URLs.
    ///
    /// This method is useful when you want to get the XML content without
    /// writing to a file, for example when serving the sitemap dynamically
    /// from a web server.
    ///
    /// Namespace declarations for the Google extensions (image, video, news,
    /// hreflang) are added to the `<urlset>` tag only when at least one entry
    /// uses the corresponding extension.
    ///
    /// # Arguments
    ///
    /// * `urls` - The [`SitemapUrl`] entries to include in the sitemap.
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
    ///     SitemapUrl::new("https://example.com/")
    ///         .lastmod("2024-01-01")
    ///         .changefreq(SitemapChangeFreq::WEEKLY)
    ///         .priority(0.8),
    ///     SitemapUrl::new("https://example.com/blog/"),
    /// ]);
    ///
    /// // Use with a web framework
    /// // HttpResponse::Ok().content_type("application/xml").body(xml)
    /// ```
    pub fn build(urls: impl IntoIterator<Item = SitemapUrl>) -> String {
        let urls: Vec<SitemapUrl> = urls.into_iter().collect();
        let mut content = String::new();
        content.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        content.push_str(r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9""#);
        if urls.iter().any(|url| !url.images.is_empty()) {
            content.push_str(r#" xmlns:image="http://www.google.com/schemas/sitemap-image/1.1""#);
        }
        if urls.iter().any(|url| !url.videos.is_empty()) {
            content.push_str(r#" xmlns:video="http://www.google.com/schemas/sitemap-video/1.1""#);
        }
        if urls.iter().any(|url| url.news.is_some()) {
            content.push_str(r#" xmlns:news="http://www.google.com/schemas/sitemap-news/0.9""#);
        }
        if urls.iter().any(|url| !url.alternates.is_empty()) {
            content.push_str(r#" xmlns:xhtml="http://www.w3.org/1999/xhtml""#);
        }
        content.push('>');
        for url in &urls {
            write_url(&mut content, url);
        }
        content.push_str("</urlset>");
        content
    }
}

fn write_url(content: &mut String, url: &SitemapUrl) {
    content.push_str("<url>");
    let _ = write!(content, "<loc>{}</loc>", html_escape::encode_text(&url.loc));
    if let Some(lastmod) = &url.lastmod {
        let _ = write!(content, "<lastmod>{}</lastmod>", lastmod);
    }
    if let Some(changefreq) = &url.changefreq {
        let _ = write!(content, "<changefreq>{}</changefreq>", changefreq);
    }
    if let Some(priority) = url.priority {
        let _ = write!(content, "<priority>{}</priority>", priority);
    }
    for image in &url.images {
        let _ = write!(
            content,
            "<image:image><image:loc>{}</image:loc></image:image>",
            html_escape::encode_text(&image.loc)
        );
    }
    for video in &url.videos {
        write_video(content, video);
    }
    if let Some(news) = &url.news {
        write_news(content, news);
    }
    for alternate in &url.alternates {
        let _ = write!(
            content,
            r#"<xhtml:link rel="alternate" hreflang="{}" href="{}"/>"#,
            html_escape::encode_double_quoted_attribute(&alternate.hreflang),
            html_escape::encode_double_quoted_attribute(&alternate.href)
        );
    }
    content.push_str("</url>");
}

fn write_video(content: &mut String, video: &SitemapVideo) {
    content.push_str("<video:video>");
    let _ = write!(
        content,
        "<video:thumbnail_loc>{}</video:thumbnail_loc><video:title>{}</video:title><video:description>{}</video:description>",
        html_escape::encode_text(&video.thumbnail_loc),
        html_escape::encode_text(&video.title),
        html_escape::encode_text(&video.description)
    );
    if let Some(content_loc) = &video.content_loc {
        let _ = write!(
            content,
            "<video:content_loc>{}</video:content_loc>",
            html_escape::encode_text(content_loc)
        );
    }
    if let Some(player_loc) = &video.player_loc {
        let _ = write!(
            content,
            "<video:player_loc>{}</video:player_loc>",
            html_escape::encode_text(player_loc)
        );
    }
    if let Some(duration) = video.duration {
        let _ = write!(content, "<video:duration>{}</video:duration>", duration);
    }
    if let Some(expiration_date) = &video.expiration_date {
        let _ = write!(
            content,
            "<video:expiration_date>{}</video:expiration_date>",
            expiration_date
        );
    }
    if let Some(publication_date) = &video.publication_date {
        let _ = write!(
            content,
            "<video:publication_date>{}</video:publication_date>",
            publication_date
        );
    }
    content.push_str("</video:video>");
}

fn write_news(content: &mut String, news: &SitemapNews) {
    let _ = write!(
        content,
        "<news:news><news:publication><news:name>{}</news:name><news:language>{}</news:language></news:publication><news:publication_date>{}</news:publication_date><news:title>{}</news:title></news:news>",
        html_escape::encode_text(&news.publication_name),
        html_escape::encode_text(&news.publication_language),
        html_escape::encode_text(&news.publication_date),
        html_escape::encode_text(&news.title)
    );
}

pub(crate) fn write_file(path: &Path, content: &str) -> Result<(), SitemapError> {
    let mut file = File::create(path).map_err(|e| SitemapError::FileOpen(e.to_string()))?;
    file.write_all(content.as_bytes())
        .map_err(|e| SitemapError::Write(e.to_string()))
}
