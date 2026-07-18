//! Split 120,000 URLs into sitemap1.xml, sitemap2.xml, sitemap3.xml plus a
//! sitemap_index.xml that references them (50,000 URLs per file).
//!
//! Run with: `cargo run --example split_large_sitemap`

use sitemap_writer::{ChunkedSitemapWriter, SitemapUrl};

fn main() {
    ChunkedSitemapWriter::new("https://example.com/sitemaps/")
        .make(
            "sitemaps",
            (1..=120_000).map(|i| SitemapUrl::new(format!("https://example.com/page/{}", i))),
        )
        .expect("failed to write sitemaps");
    println!("wrote sitemaps/sitemap1.xml ... sitemaps/sitemap_index.xml");
}
