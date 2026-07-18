//! Build a sitemap with the Google extensions: image, video, news, and
//! hreflang alternate links. The XML namespaces are declared automatically.
//!
//! Run with: `cargo run --example google_extensions`

use sitemap_writer::{
    SitemapAlternate, SitemapImage, SitemapNews, SitemapUrl, SitemapVideo, SitemapWriter,
};

fn main() {
    let xml = SitemapWriter::build(vec![
        SitemapUrl::new("https://example.com/article")
            .image(SitemapImage::new("https://example.com/photo.jpg"))
            .video(
                SitemapVideo::new(
                    "https://example.com/thumbnail.jpg",
                    "Video title",
                    "Video description",
                )
                .content_loc("https://example.com/video.mp4")
                .duration(600),
            )
            .news(SitemapNews::new(
                "The Example Times",
                "en",
                "2024-01-15",
                "Article title",
            ))
            .alternate(SitemapAlternate::new(
                "ja",
                "https://example.com/ja/article",
            ))
            .alternate(SitemapAlternate::new(
                "x-default",
                "https://example.com/article",
            )),
    ]);
    println!("{}", xml);
}
