//! Write a gzip-compressed sitemap.xml.gz.
//!
//! Run with: `cargo run --example gzip --features gzip`

use sitemap_writer::{SitemapUrl, SitemapWriter};

fn main() {
    SitemapWriter::make_gzip(
        "sitemap.xml.gz",
        vec![SitemapUrl::new("https://example.com/")],
    )
    .expect("failed to write sitemap.xml.gz");
    println!("wrote sitemap.xml.gz");
}
