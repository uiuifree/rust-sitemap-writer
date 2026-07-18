//! # sitemap-writer
//!
//! A simple and lightweight Rust library for generating XML sitemaps.
//!
//! ## Features
//!
//! - Simple builder API for creating sitemaps
//! - Automatic XML escaping for special characters
//! - Support for all sitemap properties (`loc`, `lastmod`, `changefreq`, `priority`)
//! - Support for Sitemap Index (for large sites with 50,000+ URLs)
//! - Explicit splitting into multiple sitemaps plus an index ([`ChunkedSitemapWriter`])
//! - Google extensions: image, video, news, and hreflang alternate links
//! - Optional gzip output (`gzip` feature)
//! - Write directly to file or build as String
//!
//! ## Quick Start
//!
//! ```rust
//! use sitemap_writer::{SitemapWriter, SitemapUrl, SitemapChangeFreq};
//!
//! // Build sitemap as String
//! let xml = SitemapWriter::build(vec![
//!     SitemapUrl::new("https://example.com/")
//!         .lastmod("2024-01-01")
//!         .changefreq(SitemapChangeFreq::DAILY)
//!         .priority(1.0),
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
//!     SitemapIndex::new("https://example.com/sitemap1.xml").lastmod("2024-01-01"),
//!     SitemapIndex::new("https://example.com/sitemap2.xml"),
//! ]);
//! ```
//!
//! ## Splitting Large Sitemaps
//!
//! [`ChunkedSitemapWriter`] splits a URL list into chunks of 50,000 (the
//! protocol limit) and generates the sitemap index for you:
//!
//! ```rust,no_run
//! use sitemap_writer::{ChunkedSitemapWriter, SitemapUrl};
//!
//! let result = ChunkedSitemapWriter::new("https://example.com/sitemaps/").make(
//!     "public/sitemaps",
//!     (0..100_000).map(|i| SitemapUrl::new(format!("https://example.com/page/{}", i))),
//! );
//! assert!(result.is_ok());
//! ```
//!
//! ## Google Extensions
//!
//! Image, video, news, and hreflang alternate links can be attached to each
//! URL. The corresponding XML namespaces are declared automatically:
//!
//! ```rust
//! use sitemap_writer::{SitemapWriter, SitemapUrl, SitemapImage, SitemapAlternate};
//!
//! let xml = SitemapWriter::build(vec![
//!     SitemapUrl::new("https://example.com/")
//!         .image(SitemapImage::new("https://example.com/photo.jpg"))
//!         .alternate(SitemapAlternate::new("ja", "https://example.com/ja/")),
//! ]);
//! assert!(xml.contains("xmlns:image"));
//! ```
//!
//! ## Gzip Output
//!
//! With the `gzip` feature enabled (`sitemap-writer = { version = "2.0", features = ["gzip"] }`),
//! `make_gzip` methods write `.xml.gz` files.

#![warn(missing_docs)]

mod chunked_sitemap_writer;
mod error;
mod extensions;
#[cfg(feature = "gzip")]
mod gzip;
mod sitemap_index;
mod sitemap_url;
mod sitemap_writer;

pub use chunked_sitemap_writer::{ChunkedSitemap, ChunkedSitemapWriter};
pub use error::SitemapError;
pub use extensions::{SitemapAlternate, SitemapImage, SitemapNews, SitemapVideo};
pub use sitemap_index::{SitemapIndex, SitemapIndexWriter};
pub use sitemap_url::{SitemapChangeFreq, SitemapUrl};
pub use sitemap_writer::SitemapWriter;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        ChunkedSitemapWriter, SitemapAlternate, SitemapChangeFreq, SitemapError, SitemapImage,
        SitemapIndex, SitemapIndexWriter, SitemapNews, SitemapUrl, SitemapVideo, SitemapWriter,
    };

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(name)
    }

    #[test]
    fn test_make() {
        let path = temp_path("sitemap_writer_test_make.xml");
        let path = path.to_str().unwrap();

        let res = SitemapWriter::make(path, vec![]);
        assert!(res.is_ok());

        let urls = vec![
            SitemapUrl {
                loc: "https://example.com/".to_string(),
                lastmod: Some("2021-01-01".to_string()),
                changefreq: Some(SitemapChangeFreq::ALWAYS),
                priority: Some(1.0),
                ..Default::default()
            },
            SitemapUrl::new("https://example.com/contact/"),
            SitemapUrl::new("https://example.com/contact/?test=1"),
            SitemapUrl::new("https://example.com/contact/?test=<>"),
        ];
        let res = SitemapWriter::make(path, urls.clone());
        assert!(res.is_ok());

        let written = std::fs::read_to_string(path).unwrap();
        assert_eq!(written, SitemapWriter::build(urls));
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_make_with_pathbuf() {
        let path = temp_path("sitemap_writer_test_make_pathbuf.xml");
        let res = SitemapWriter::make(&path, vec![SitemapUrl::new("https://example.com/")]);
        assert!(res.is_ok());
        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_make_file_open_error() {
        let res = SitemapWriter::make("/nonexistent_dir/sitemap.xml", vec![]);
        let err = res.unwrap_err();
        assert!(matches!(err, SitemapError::FileOpen(_)));
        assert!(err.to_string().starts_with("Failed to open file:"));
    }

    #[test]
    fn test_build() {
        let xml = SitemapWriter::build(vec![
            SitemapUrl {
                loc: "https://example.com/".to_string(),
                lastmod: Some("2024-01-01".to_string()),
                changefreq: Some(SitemapChangeFreq::DAILY),
                priority: Some(0.8),
                ..Default::default()
            },
            SitemapUrl::new("https://example.com/about/"),
        ]);
        assert_eq!(
            xml,
            concat!(
                r#"<?xml version="1.0" encoding="UTF-8"?>"#,
                r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
                "<url><loc>https://example.com/</loc><lastmod>2024-01-01</lastmod>",
                "<changefreq>daily</changefreq><priority>0.8</priority></url>",
                "<url><loc>https://example.com/about/</loc></url>",
                "</urlset>"
            )
        );
    }

    #[test]
    fn test_build_with_builder_matches_literal() {
        let by_builder = SitemapWriter::build(vec![
            SitemapUrl::new("https://example.com/")
                .lastmod("2024-01-01")
                .changefreq(SitemapChangeFreq::DAILY)
                .priority(0.8),
        ]);
        let by_literal = SitemapWriter::build(vec![SitemapUrl {
            loc: "https://example.com/".to_string(),
            lastmod: Some("2024-01-01".to_string()),
            changefreq: Some(SitemapChangeFreq::DAILY),
            priority: Some(0.8),
            ..Default::default()
        }]);
        assert_eq!(by_builder, by_literal);
    }

    #[test]
    fn test_build_from_iterator() {
        let xml = SitemapWriter::build(
            (1..=2).map(|i| SitemapUrl::new(format!("https://example.com/{}", i))),
        );
        assert!(xml.contains("<loc>https://example.com/1</loc>"));
        assert!(xml.contains("<loc>https://example.com/2</loc>"));
    }

    #[test]
    fn test_xml_escaping() {
        let xml = SitemapWriter::build(vec![SitemapUrl::new("https://example.com/?a=1&b=<2>")]);
        assert!(xml.contains("<loc>https://example.com/?a=1&amp;b=&lt;2&gt;</loc>"));
    }

    #[test]
    fn test_changefreq_display() {
        assert_eq!(SitemapChangeFreq::ALWAYS.to_string(), "always");
        assert_eq!(SitemapChangeFreq::HOURLY.to_string(), "hourly");
        assert_eq!(SitemapChangeFreq::DAILY.to_string(), "daily");
        assert_eq!(SitemapChangeFreq::WEEKLY.to_string(), "weekly");
        assert_eq!(SitemapChangeFreq::MONTHLY.to_string(), "monthly");
        assert_eq!(SitemapChangeFreq::YEARLY.to_string(), "yearly");
        assert_eq!(SitemapChangeFreq::NEVER.to_string(), "never");
    }

    #[test]
    fn test_changefreq_debug() {
        assert_eq!(format!("{:?}", SitemapChangeFreq::DAILY), "daily");
    }

    #[test]
    fn test_sitemap_url_new() {
        let url = SitemapUrl::new("https://example.com/test");
        assert_eq!(url.loc, "https://example.com/test");
        assert!(url.lastmod.is_none());
        assert!(url.changefreq.is_none());
        assert!(url.priority.is_none());
        assert!(url.images.is_empty());
        assert!(url.videos.is_empty());
        assert!(url.news.is_none());
        assert!(url.alternates.is_empty());
    }

    #[test]
    fn test_sitemap_url_builder() {
        let url = SitemapUrl::new("https://example.com/")
            .lastmod("2024-01-01")
            .changefreq(SitemapChangeFreq::WEEKLY)
            .priority(0.5)
            .image(SitemapImage::new("https://example.com/a.png"))
            .image(SitemapImage::new("https://example.com/b.png"))
            .video(SitemapVideo::new(
                "https://example.com/thumb.jpg",
                "title",
                "description",
            ))
            .news(SitemapNews::new("Times", "en", "2024-01-01", "headline"))
            .alternate(SitemapAlternate::new("ja", "https://example.com/ja/"));
        assert_eq!(url.lastmod.as_deref(), Some("2024-01-01"));
        assert_eq!(url.changefreq, Some(SitemapChangeFreq::WEEKLY));
        assert_eq!(url.priority, Some(0.5));
        assert_eq!(url.images.len(), 2);
        assert_eq!(url.images[1].loc, "https://example.com/b.png");
        assert_eq!(url.videos.len(), 1);
        assert_eq!(url.news.as_ref().unwrap().title, "headline");
        assert_eq!(url.alternates.len(), 1);
    }

    #[test]
    fn test_sitemap_video_builder() {
        let video = SitemapVideo::new("https://example.com/thumb.jpg", "title", "description")
            .content_loc("https://example.com/video.mp4")
            .player_loc("https://example.com/player")
            .duration(600)
            .expiration_date("2025-01-01")
            .publication_date("2024-01-01");
        assert_eq!(video.thumbnail_loc, "https://example.com/thumb.jpg");
        assert_eq!(video.title, "title");
        assert_eq!(video.description, "description");
        assert_eq!(
            video.content_loc.as_deref(),
            Some("https://example.com/video.mp4")
        );
        assert_eq!(
            video.player_loc.as_deref(),
            Some("https://example.com/player")
        );
        assert_eq!(video.duration, Some(600));
        assert_eq!(video.expiration_date.as_deref(), Some("2025-01-01"));
        assert_eq!(video.publication_date.as_deref(), Some("2024-01-01"));
    }

    #[test]
    fn test_sitemap_news_new() {
        let news = SitemapNews::new("Times", "en", "2024-01-15", "headline");
        assert_eq!(news.publication_name, "Times");
        assert_eq!(news.publication_language, "en");
        assert_eq!(news.publication_date, "2024-01-15");
        assert_eq!(news.title, "headline");
    }

    #[test]
    fn test_build_with_image() {
        let xml = SitemapWriter::build(vec![
            SitemapUrl::new("https://example.com/")
                .image(SitemapImage::new("https://example.com/a.png"))
                .image(SitemapImage::new("https://example.com/b.png")),
        ]);
        assert_eq!(
            xml,
            concat!(
                r#"<?xml version="1.0" encoding="UTF-8"?>"#,
                r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:image="http://www.google.com/schemas/sitemap-image/1.1">"#,
                "<url><loc>https://example.com/</loc>",
                "<image:image><image:loc>https://example.com/a.png</image:loc></image:image>",
                "<image:image><image:loc>https://example.com/b.png</image:loc></image:image>",
                "</url></urlset>"
            )
        );
    }

    #[test]
    fn test_build_with_video_minimal() {
        let xml = SitemapWriter::build(vec![SitemapUrl::new("https://example.com/").video(
            SitemapVideo::new("https://example.com/thumb.jpg", "title", "description"),
        )]);
        assert_eq!(
            xml,
            concat!(
                r#"<?xml version="1.0" encoding="UTF-8"?>"#,
                r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:video="http://www.google.com/schemas/sitemap-video/1.1">"#,
                "<url><loc>https://example.com/</loc>",
                "<video:video><video:thumbnail_loc>https://example.com/thumb.jpg</video:thumbnail_loc>",
                "<video:title>title</video:title>",
                "<video:description>description</video:description></video:video>",
                "</url></urlset>"
            )
        );
    }

    #[test]
    fn test_build_with_video_full() {
        let xml = SitemapWriter::build(vec![
            SitemapUrl::new("https://example.com/").video(
                SitemapVideo::new("https://example.com/thumb.jpg", "title", "description")
                    .content_loc("https://example.com/video.mp4")
                    .player_loc("https://example.com/player")
                    .duration(600)
                    .expiration_date("2025-01-01")
                    .publication_date("2024-01-01"),
            ),
        ]);
        assert!(xml.contains(concat!(
            "<video:video><video:thumbnail_loc>https://example.com/thumb.jpg</video:thumbnail_loc>",
            "<video:title>title</video:title>",
            "<video:description>description</video:description>",
            "<video:content_loc>https://example.com/video.mp4</video:content_loc>",
            "<video:player_loc>https://example.com/player</video:player_loc>",
            "<video:duration>600</video:duration>",
            "<video:expiration_date>2025-01-01</video:expiration_date>",
            "<video:publication_date>2024-01-01</video:publication_date>",
            "</video:video>"
        )));
    }

    #[test]
    fn test_build_with_news() {
        let xml = SitemapWriter::build(vec![SitemapUrl::new("https://example.com/article").news(
            SitemapNews::new("The Example Times", "en", "2024-01-15", "headline"),
        )]);
        assert_eq!(
            xml,
            concat!(
                r#"<?xml version="1.0" encoding="UTF-8"?>"#,
                r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:news="http://www.google.com/schemas/sitemap-news/0.9">"#,
                "<url><loc>https://example.com/article</loc>",
                "<news:news><news:publication><news:name>The Example Times</news:name>",
                "<news:language>en</news:language></news:publication>",
                "<news:publication_date>2024-01-15</news:publication_date>",
                "<news:title>headline</news:title></news:news>",
                "</url></urlset>"
            )
        );
    }

    #[test]
    fn test_build_with_alternates() {
        let xml = SitemapWriter::build(vec![
            SitemapUrl::new("https://example.com/en/")
                .alternate(SitemapAlternate::new(
                    "en",
                    "https://example.com/en/?a=1&b=2",
                ))
                .alternate(SitemapAlternate::new("x-default", "https://example.com/")),
        ]);
        assert_eq!(
            xml,
            concat!(
                r#"<?xml version="1.0" encoding="UTF-8"?>"#,
                r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">"#,
                "<url><loc>https://example.com/en/</loc>",
                r#"<xhtml:link rel="alternate" hreflang="en" href="https://example.com/en/?a=1&amp;b=2"/>"#,
                r#"<xhtml:link rel="alternate" hreflang="x-default" href="https://example.com/"/>"#,
                "</url></urlset>"
            )
        );
    }

    #[test]
    fn test_build_with_all_extensions() {
        let xml = SitemapWriter::build(vec![
            SitemapUrl::new("https://example.com/")
                .image(SitemapImage::new("https://example.com/a.png"))
                .video(SitemapVideo::new(
                    "https://example.com/thumb.jpg",
                    "title",
                    "description",
                ))
                .news(SitemapNews::new("Times", "en", "2024-01-15", "headline"))
                .alternate(SitemapAlternate::new("ja", "https://example.com/ja/")),
        ]);
        assert!(xml.starts_with(concat!(
            r#"<?xml version="1.0" encoding="UTF-8"?>"#,
            r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9""#,
            r#" xmlns:image="http://www.google.com/schemas/sitemap-image/1.1""#,
            r#" xmlns:video="http://www.google.com/schemas/sitemap-video/1.1""#,
            r#" xmlns:news="http://www.google.com/schemas/sitemap-news/0.9""#,
            r#" xmlns:xhtml="http://www.w3.org/1999/xhtml">"#
        )));
        let image_pos = xml.find("<image:image>").unwrap();
        let video_pos = xml.find("<video:video>").unwrap();
        let news_pos = xml.find("<news:news>").unwrap();
        let alternate_pos = xml.find("<xhtml:link").unwrap();
        assert!(image_pos < video_pos);
        assert!(video_pos < news_pos);
        assert!(news_pos < alternate_pos);
    }

    #[test]
    fn test_extension_text_escaping() {
        let xml = SitemapWriter::build(vec![SitemapUrl::new("https://example.com/").video(
            SitemapVideo::new("https://example.com/thumb.jpg", "a < b & c", "desc"),
        )]);
        assert!(xml.contains("<video:title>a &lt; b &amp; c</video:title>"));
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
        assert_eq!(
            xml,
            concat!(
                r#"<?xml version="1.0" encoding="UTF-8"?>"#,
                r#"<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
                "<sitemap><loc>https://example.com/sitemap1.xml</loc>",
                "<lastmod>2024-01-01</lastmod></sitemap>",
                "<sitemap><loc>https://example.com/sitemap2.xml</loc></sitemap>",
                "</sitemapindex>"
            )
        );
    }

    #[test]
    fn test_sitemap_index_make() {
        let path = temp_path("sitemap_writer_test_index_make.xml");
        let path = path.to_str().unwrap();

        let sitemaps = vec![
            SitemapIndex::new("https://example.com/sitemap1.xml"),
            SitemapIndex::new("https://example.com/sitemap2.xml").lastmod("2024-01-15"),
        ];
        let res = SitemapIndexWriter::make(path, sitemaps.clone());
        assert!(res.is_ok());

        let written = std::fs::read_to_string(path).unwrap();
        assert_eq!(written, SitemapIndexWriter::build(sitemaps));
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_sitemap_index_make_file_open_error() {
        let res = SitemapIndexWriter::make("/nonexistent_dir/sitemap_index.xml", vec![]);
        let err = res.unwrap_err();
        assert!(matches!(err, SitemapError::FileOpen(_)));
    }

    #[test]
    fn test_sitemap_index_new() {
        let sitemap = SitemapIndex::new("https://example.com/sitemap.xml");
        assert_eq!(sitemap.loc, "https://example.com/sitemap.xml");
        assert!(sitemap.lastmod.is_none());
    }

    #[test]
    fn test_sitemap_index_builder() {
        let sitemap = SitemapIndex::new("https://example.com/sitemap.xml").lastmod("2024-01-15");
        assert_eq!(sitemap.lastmod.as_deref(), Some("2024-01-15"));
    }

    #[test]
    fn test_sitemap_index_xml_escaping() {
        let xml = SitemapIndexWriter::build(vec![SitemapIndex::new(
            "https://example.com/sitemap.xml?a=1&b=2",
        )]);
        assert!(xml.contains("<loc>https://example.com/sitemap.xml?a=1&amp;b=2</loc>"));
    }

    #[test]
    fn test_chunked_build() {
        let urls: Vec<SitemapUrl> = (1..=5)
            .map(|i| SitemapUrl::new(format!("https://example.com/page{}", i)))
            .collect();
        let chunked = ChunkedSitemapWriter::new("https://example.com/sm/")
            .chunk_size(2)
            .build(urls.clone());
        assert_eq!(chunked.sitemaps.len(), 3);
        assert_eq!(
            chunked.sitemaps[0],
            SitemapWriter::build(urls[0..2].to_vec())
        );
        assert_eq!(
            chunked.sitemaps[2],
            SitemapWriter::build(urls[4..5].to_vec())
        );
        assert_eq!(
            chunked.index,
            concat!(
                r#"<?xml version="1.0" encoding="UTF-8"?>"#,
                r#"<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
                "<sitemap><loc>https://example.com/sm/sitemap1.xml</loc></sitemap>",
                "<sitemap><loc>https://example.com/sm/sitemap2.xml</loc></sitemap>",
                "<sitemap><loc>https://example.com/sm/sitemap3.xml</loc></sitemap>",
                "</sitemapindex>"
            )
        );
    }

    #[test]
    fn test_chunked_build_exact_multiple() {
        let urls: Vec<SitemapUrl> = (1..=4)
            .map(|i| SitemapUrl::new(format!("https://example.com/page{}", i)))
            .collect();
        let chunked = ChunkedSitemapWriter::new("https://example.com/")
            .chunk_size(2)
            .build(urls);
        assert_eq!(chunked.sitemaps.len(), 2);
    }

    #[test]
    fn test_chunked_build_base_url_without_slash() {
        let chunked = ChunkedSitemapWriter::new("https://example.com")
            .build(vec![SitemapUrl::new("https://example.com/")]);
        assert!(
            chunked
                .index
                .contains("<loc>https://example.com/sitemap1.xml</loc>")
        );
    }

    #[test]
    fn test_chunked_build_default_chunk_size() {
        let urls = (0..50_001).map(|i| SitemapUrl::new(format!("https://example.com/{}", i)));
        let chunked = ChunkedSitemapWriter::new("https://example.com/").build(urls);
        assert_eq!(chunked.sitemaps.len(), 2);
        assert_eq!(chunked.sitemaps[0].matches("<url>").count(), 50_000);
        assert_eq!(chunked.sitemaps[1].matches("<url>").count(), 1);
    }

    #[test]
    fn test_chunked_build_empty() {
        let chunked = ChunkedSitemapWriter::new("https://example.com/").build(Vec::new());
        assert_eq!(chunked.sitemaps.len(), 1);
        assert_eq!(chunked.sitemaps[0], SitemapWriter::build(Vec::new()));
        assert!(
            chunked
                .index
                .contains("<loc>https://example.com/sitemap1.xml</loc>")
        );
    }

    #[test]
    #[should_panic(expected = "chunk_size must be greater than 0")]
    fn test_chunked_chunk_size_zero_panics() {
        let _ = ChunkedSitemapWriter::new("https://example.com/").chunk_size(0);
    }

    #[test]
    fn test_chunked_make() {
        let dir = temp_path("sitemap_writer_test_chunked").join("nested");
        let urls: Vec<SitemapUrl> = (1..=3)
            .map(|i| SitemapUrl::new(format!("https://example.com/page{}", i)))
            .collect();
        let writer = ChunkedSitemapWriter::new("https://example.com/").chunk_size(2);
        writer.make(&dir, urls.clone()).unwrap();

        let chunked = writer.build(urls);
        for (i, sitemap) in chunked.sitemaps.iter().enumerate() {
            let written =
                std::fs::read_to_string(dir.join(format!("sitemap{}.xml", i + 1))).unwrap();
            assert_eq!(&written, sitemap);
        }
        let written_index = std::fs::read_to_string(dir.join("sitemap_index.xml")).unwrap();
        assert_eq!(written_index, chunked.index);
        std::fs::remove_dir_all(temp_path("sitemap_writer_test_chunked")).unwrap();
    }

    #[test]
    fn test_chunked_make_create_dir_error() {
        let file_path = temp_path("sitemap_writer_test_not_a_dir");
        std::fs::write(&file_path, "x").unwrap();
        let res = ChunkedSitemapWriter::new("https://example.com/")
            .make(&file_path, vec![SitemapUrl::new("https://example.com/")]);
        let err = res.unwrap_err();
        assert!(matches!(err, SitemapError::CreateDir(_)));
        assert!(err.to_string().starts_with("Failed to create directory:"));
        std::fs::remove_file(&file_path).unwrap();
    }

    #[test]
    fn test_error_display() {
        assert_eq!(
            SitemapError::FileOpen("denied".to_string()).to_string(),
            "Failed to open file: denied"
        );
        assert_eq!(
            SitemapError::Write("disk full".to_string()).to_string(),
            "Failed to write: disk full"
        );
        assert_eq!(
            SitemapError::CreateDir("denied".to_string()).to_string(),
            "Failed to create directory: denied"
        );
    }

    #[cfg(feature = "gzip")]
    fn read_gzip(path: &std::path::Path) -> String {
        use std::io::Read as _;
        let mut decoder = flate2::read::GzDecoder::new(std::fs::File::open(path).unwrap());
        let mut xml = String::new();
        decoder.read_to_string(&mut xml).unwrap();
        xml
    }

    #[test]
    #[cfg(feature = "gzip")]
    fn test_make_gzip() {
        let path = temp_path("sitemap_writer_test_make.xml.gz");
        let urls = vec![SitemapUrl::new("https://example.com/")];
        SitemapWriter::make_gzip(&path, urls.clone()).unwrap();
        assert_eq!(read_gzip(&path), SitemapWriter::build(urls));
        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    #[cfg(feature = "gzip")]
    fn test_make_gzip_file_open_error() {
        let res = SitemapWriter::make_gzip("/nonexistent_dir/sitemap.xml.gz", vec![]);
        assert!(matches!(res.unwrap_err(), SitemapError::FileOpen(_)));
    }

    #[test]
    #[cfg(feature = "gzip")]
    fn test_sitemap_index_make_gzip() {
        let path = temp_path("sitemap_writer_test_index_make.xml.gz");
        let sitemaps = vec![SitemapIndex::new("https://example.com/sitemap1.xml.gz")];
        SitemapIndexWriter::make_gzip(&path, sitemaps.clone()).unwrap();
        assert_eq!(read_gzip(&path), SitemapIndexWriter::build(sitemaps));
        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    #[cfg(feature = "gzip")]
    fn test_chunked_make_gzip() {
        let dir = temp_path("sitemap_writer_test_chunked_gzip");
        let urls: Vec<SitemapUrl> = (1..=3)
            .map(|i| SitemapUrl::new(format!("https://example.com/page{}", i)))
            .collect();
        let writer = ChunkedSitemapWriter::new("https://example.com/").chunk_size(2);
        writer.make_gzip(&dir, urls.clone()).unwrap();

        let chunked = writer.build(urls);
        for (i, sitemap) in chunked.sitemaps.iter().enumerate() {
            let written = read_gzip(&dir.join(format!("sitemap{}.xml.gz", i + 1)));
            assert_eq!(&written, sitemap);
        }
        let written_index = read_gzip(&dir.join("sitemap_index.xml.gz"));
        assert!(written_index.contains("<loc>https://example.com/sitemap1.xml.gz</loc>"));
        assert!(written_index.contains("<loc>https://example.com/sitemap2.xml.gz</loc>"));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
