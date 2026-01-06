//! # sitemap-writer
//!
//! A simple and lightweight Rust library for generating XML sitemaps.
//!
//! ## Features
//!
//! - Simple API for creating sitemaps
//! - Automatic XML escaping for special characters
//! - Support for all sitemap properties (`loc`, `lastmod`, `changefreq`, `priority`)
//! - Support for Sitemap Index (for large sites with 50,000+ URLs)
//! - Write directly to file or build as String
//!
//! ## Quick Start
//!
//! ```rust
//! use sitemap_writer::{SitemapWriter, SitemapUrl, SitemapChangeFreq};
//!
//! // Build sitemap as String
//! let xml = SitemapWriter::build(vec![
//!     SitemapUrl {
//!         loc: "https://example.com/".to_string(),
//!         lastmod: Some("2024-01-01".to_string()),
//!         changefreq: Some(SitemapChangeFreq::DAILY),
//!         priority: Some(1.0),
//!     },
//!     SitemapUrl::new("https://example.com/about/"),
//! ]);
//! ```
//!
//! ## Writing to a File
//!
//! ```rust,no_run
//! use sitemap_writer::{SitemapWriter, SitemapUrl};
//!
//! let result = SitemapWriter::make("sitemap.xml", vec![
//!     SitemapUrl::new("https://example.com/"),
//! ]);
//! assert!(result.is_ok());
//! ```
//!
//! ## Sitemap Index
//!
//! For large sites with more than 50,000 URLs, use Sitemap Index to reference multiple sitemaps:
//!
//! ```rust
//! use sitemap_writer::{SitemapIndexWriter, SitemapIndex};
//!
//! let xml = SitemapIndexWriter::build(vec![
//!     SitemapIndex {
//!         loc: "https://example.com/sitemap1.xml".to_string(),
//!         lastmod: Some("2024-01-01".to_string()),
//!     },
//!     SitemapIndex::new("https://example.com/sitemap2.xml"),
//! ]);
//! ```

mod error;
mod sitemap_index;
mod sitemap_url;
mod sitemap_writer;

pub use error::SitemapError;
pub use sitemap_index::{SitemapIndex, SitemapIndexWriter};
pub use sitemap_url::{SitemapChangeFreq, SitemapUrl};
pub use sitemap_writer::SitemapWriter;

#[cfg(test)]
mod tests {
    use crate::{SitemapChangeFreq, SitemapIndex, SitemapIndexWriter, SitemapUrl, SitemapWriter};

    #[test]
    fn test_make() {
        let res = SitemapWriter::make("test.xml", vec![]);
        assert!(res.is_ok());

        let res = SitemapWriter::make(
            "test.xml",
            vec![
                SitemapUrl {
                    loc: "https://example.com/".to_string(),
                    lastmod: Some("2021-01-01".to_string()),
                    changefreq: Some(SitemapChangeFreq::ALWAYS),
                    priority: Some(1.0),
                },
                SitemapUrl::new("https://example.com/contact/"),
                SitemapUrl::new("https://example.com/contact/?test=1"),
                SitemapUrl::new("https://example.com/contact/?test=<>"),
            ],
        );
        assert!(res.is_ok());
    }

    #[test]
    fn test_build() {
        let xml = SitemapWriter::build(vec![SitemapUrl::new("https://example.com/")]);
        assert!(xml.contains("<loc>https://example.com/</loc>"));
        assert!(xml.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
    }

    #[test]
    fn test_xml_escaping() {
        let xml = SitemapWriter::build(vec![SitemapUrl::new("https://example.com/?a=1&b=2")]);
        assert!(xml.contains("&amp;"));
    }

    #[test]
    fn test_changefreq_display() {
        assert_eq!(SitemapChangeFreq::ALWAYS.to_string(), "always");
        assert_eq!(SitemapChangeFreq::DAILY.to_string(), "daily");
        assert_eq!(SitemapChangeFreq::NEVER.to_string(), "never");
    }

    #[test]
    fn test_sitemap_url_new() {
        let url = SitemapUrl::new("https://example.com/test");
        assert_eq!(url.loc, "https://example.com/test");
        assert!(url.lastmod.is_none());
        assert!(url.changefreq.is_none());
        assert!(url.priority.is_none());
    }

    #[test]
    fn test_sitemap_index_build() {
        let xml = SitemapIndexWriter::build(vec![
            SitemapIndex {
                loc: "https://example.com/sitemap1.xml".to_string(),
                lastmod: Some("2024-01-01".to_string()),
            },
            SitemapIndex::new("https://example.com/sitemap2.xml"),
        ]);
        assert!(xml.contains("<sitemapindex"));
        assert!(xml.contains("</sitemapindex>"));
        assert!(xml.contains("<loc>https://example.com/sitemap1.xml</loc>"));
        assert!(xml.contains("<lastmod>2024-01-01</lastmod>"));
        assert!(xml.contains("<loc>https://example.com/sitemap2.xml</loc>"));
    }

    #[test]
    fn test_sitemap_index_make() {
        let res = SitemapIndexWriter::make(
            "test_index.xml",
            vec![
                SitemapIndex::new("https://example.com/sitemap1.xml"),
                SitemapIndex {
                    loc: "https://example.com/sitemap2.xml".to_string(),
                    lastmod: Some("2024-01-15".to_string()),
                },
            ],
        );
        assert!(res.is_ok());
    }

    #[test]
    fn test_sitemap_index_new() {
        let sitemap = SitemapIndex::new("https://example.com/sitemap.xml");
        assert_eq!(sitemap.loc, "https://example.com/sitemap.xml");
        assert!(sitemap.lastmod.is_none());
    }

    #[test]
    fn test_sitemap_index_xml_escaping() {
        let xml = SitemapIndexWriter::build(vec![SitemapIndex::new(
            "https://example.com/sitemap.xml?a=1&b=2",
        )]);
        assert!(xml.contains("&amp;"));
    }
}
