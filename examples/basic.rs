//! Write a basic sitemap.xml.
//!
//! Run with: `cargo run --example basic`

use sitemap_writer::{SitemapChangeFreq, SitemapUrl, SitemapWriter};

fn main() {
    SitemapWriter::make(
        "sitemap.xml",
        vec![
            SitemapUrl::new("https://example.com/")
                .lastmod("2024-01-01")
                .changefreq(SitemapChangeFreq::DAILY)
                .priority(1.0),
            SitemapUrl::new("https://example.com/about/"),
        ],
    )
    .expect("failed to write sitemap.xml");
    println!("wrote sitemap.xml");
}
