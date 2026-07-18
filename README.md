# sitemap-writer

[![Crates.io](https://img.shields.io/crates/v/sitemap-writer.svg)](https://crates.io/crates/sitemap-writer)
[![Documentation](https://docs.rs/sitemap-writer/badge.svg)](https://docs.rs/sitemap-writer)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A simple and lightweight Rust library for generating XML sitemaps.

## Features

- Simple builder API for creating sitemaps
- Automatic XML escaping for special characters
- Support for all sitemap properties (`loc`, `lastmod`, `changefreq`, `priority`)
- Support for Sitemap Index (for large sites with 50,000+ URLs)
- Explicit splitting into multiple sitemaps plus an index (`ChunkedSitemapWriter`)
- Google extensions: image, video, news, and hreflang alternate links
- Optional gzip output (`gzip` feature)
- Write directly to file or build as String
- No heavy dependencies

## Why sitemap-writer?

- **Covers the whole protocol**: every sitemaps.org element, sitemap index,
  splitting at the 50,000-URL limit, and the Google
  image/video/news/hreflang extensions.
- **Lightweight**: one tiny required dependency (`html-escape`); gzip support
  is opt-in.
- **Predictable output**: deterministic single-line XML, byte-stable across
  versions as long as you don't use new features, verified by exact-match
  tests.
- **Works anywhere**: write files for static hosting, or build a `String` and
  serve it from any web framework (axum, actix-web, Rocket, ...).

## Installation

```sh
cargo add sitemap-writer
```

With gzip support:

```sh
cargo add sitemap-writer --features gzip
```

Or in `Cargo.toml`:

```toml
[dependencies]
sitemap-writer = { version = "2.0", features = ["gzip"] }
```

## Quick Start

```rust
use sitemap_writer::{SitemapWriter, SitemapUrl, SitemapChangeFreq};

fn main() {
    let result = SitemapWriter::make("sitemap.xml", vec![
        SitemapUrl::new("https://example.com/")
            .lastmod("2024-01-01")
            .changefreq(SitemapChangeFreq::DAILY)
            .priority(1.0),
        SitemapUrl::new("https://example.com/about/"),
    ]);

    assert!(result.is_ok());
}
```

Runnable examples for every feature live in [`examples/`](examples/):

```sh
cargo run --example basic
cargo run --example google_extensions
cargo run --example split_large_sitemap
cargo run --example gzip --features gzip
```

## API Reference

### SitemapWriter

#### `SitemapWriter::make(path, urls)` - Write to File

`path` accepts anything that implements `AsRef<Path>`, and `urls` accepts any
`IntoIterator` of `SitemapUrl` (a `Vec`, an iterator, etc.):

```rust
use sitemap_writer::{SitemapWriter, SitemapUrl};

let result = SitemapWriter::make(
    "sitemap.xml",
    (1..=100).map(|i| SitemapUrl::new(format!("https://example.com/page/{}", i))),
);
```

#### `SitemapWriter::build(urls)` - Build as String

```rust
use sitemap_writer::{SitemapWriter, SitemapUrl};

let xml = SitemapWriter::build(vec![
    SitemapUrl::new("https://example.com/"),
    SitemapUrl::new("https://example.com/about/"),
]);
// e.g. serve it from a web framework:
// HttpResponse::Ok().content_type("application/xml").body(xml)
```

#### `SitemapWriter::make_gzip(path, urls)` - Write as .xml.gz (feature `gzip`)

```rust
use sitemap_writer::{SitemapWriter, SitemapUrl};

let result = SitemapWriter::make_gzip("sitemap.xml.gz", vec![
    SitemapUrl::new("https://example.com/"),
]);
```

### SitemapUrl

| Field | Type | Description |
|-------|------|-------------|
| `loc` | `String` | **Required.** The URL of the page. |
| `lastmod` | `Option<String>` | The date of last modification (W3C Datetime). |
| `changefreq` | `Option<SitemapChangeFreq>` | How frequently the page changes. |
| `priority` | `Option<f32>` | Priority relative to other URLs (0.0 to 1.0). |
| `images` | `Vec<SitemapImage>` | Google image extension entries. |
| `videos` | `Vec<SitemapVideo>` | Google video extension entries. |
| `news` | `Option<SitemapNews>` | Google news extension entry. |
| `alternates` | `Vec<SitemapAlternate>` | hreflang alternate links. |

All optional fields can be set with builder methods of the same name:

```rust
use sitemap_writer::{SitemapUrl, SitemapChangeFreq};

let url = SitemapUrl::new("https://example.com/page")
    .lastmod("2024-01-15")
    .changefreq(SitemapChangeFreq::WEEKLY)
    .priority(0.8);
```

### SitemapChangeFreq

| Value | Description |
|-------|-------------|
| `ALWAYS` | Changes every access |
| `HOURLY` | Changes hourly |
| `DAILY` | Changes daily |
| `WEEKLY` | Changes weekly |
| `MONTHLY` | Changes monthly |
| `YEARLY` | Changes yearly |
| `NEVER` | Archived content |

### Output Example

The actual output is a single line; it is formatted here for readability:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>https://example.com/</loc>
    <lastmod>2024-01-01</lastmod>
    <changefreq>daily</changefreq>
    <priority>1</priority>
  </url>
  <url>
    <loc>https://example.com/about/</loc>
  </url>
</urlset>
```

## Sitemap Index

For large sites with more than 50,000 URLs, use Sitemap Index:

```rust
use sitemap_writer::{SitemapIndexWriter, SitemapIndex};

let result = SitemapIndexWriter::make("sitemap_index.xml", vec![
    SitemapIndex::new("https://example.com/sitemap1.xml").lastmod("2024-01-01"),
    SitemapIndex::new("https://example.com/sitemap2.xml"),
]);
```

`SitemapIndexWriter::build(sitemaps)` returns the XML as a `String`, and
`SitemapIndexWriter::make_gzip(path, sitemaps)` (feature `gzip`) writes a
compressed index.

### SitemapIndex

| Field | Type | Description |
|-------|------|-------------|
| `loc` | `String` | **Required.** The URL of the sitemap file. |
| `lastmod` | `Option<String>` | The date of last modification (W3C Datetime). |

## Splitting Large Sitemaps

`ChunkedSitemapWriter` splits a URL list into chunks of 50,000 (the protocol
limit) and generates the sitemap index for you. Splitting only happens through
this writer; `SitemapWriter` never splits on its own.

```rust
use sitemap_writer::{ChunkedSitemapWriter, SitemapUrl};

// Writes public/sitemaps/sitemap1.xml, sitemap2.xml, ... and
// sitemap_index.xml whose <loc> entries point at
// https://example.com/sitemaps/sitemap1.xml, ...
let result = ChunkedSitemapWriter::new("https://example.com/sitemaps/").make(
    "public/sitemaps",
    (0..100_000).map(|i| SitemapUrl::new(format!("https://example.com/page/{}", i))),
);
```

- `chunk_size(n)` overrides the URLs-per-sitemap limit (default 50,000).
- `build(urls)` returns a `ChunkedSitemap { sitemaps: Vec<String>, index: String }`
  instead of writing files.
- `make_gzip(dir, urls)` (feature `gzip`) writes `.xml.gz` files and points the
  index at them.

## Google Extensions

Image, video, news, and hreflang alternate links can be attached to each URL.
The corresponding XML namespaces are declared on `<urlset>` automatically, and
only when used — sitemaps without extensions are byte-identical to those from
v1.

```rust
use sitemap_writer::{
    SitemapWriter, SitemapUrl, SitemapImage, SitemapVideo, SitemapNews, SitemapAlternate,
};

let xml = SitemapWriter::build(vec![
    SitemapUrl::new("https://example.com/article")
        .image(SitemapImage::new("https://example.com/photo.jpg"))
        .video(
            SitemapVideo::new(
                "https://example.com/thumbnail.jpg", // thumbnail_loc (required)
                "Video title",                       // title (required)
                "Video description",                 // description (required)
            )
            .content_loc("https://example.com/video.mp4")
            .duration(600),
        )
        .news(SitemapNews::new(
            "The Example Times", // publication name
            "en",                // publication language
            "2024-01-15",        // publication date
            "Article title",     // article title
        ))
        .alternate(SitemapAlternate::new("ja", "https://example.com/ja/article"))
        .alternate(SitemapAlternate::new("x-default", "https://example.com/article")),
]);
```

Note: Google requires a video to have either `content_loc` or `player_loc`;
this library does not validate that requirement.

## Migrating from 1.x

- `SitemapUrl` gained the `images`, `videos`, `news`, and `alternates` fields.
  Struct literal construction now needs `..Default::default()`:

  ```rust
  use sitemap_writer::SitemapUrl;

  let url = SitemapUrl {
      loc: "https://example.com/".to_string(),
      lastmod: Some("2024-01-01".to_string()),
      ..Default::default()
  };
  ```

  Or switch to the builder API: `SitemapUrl::new("https://example.com/").lastmod("2024-01-01")`.
- `SitemapError` gained a `CreateDir` variant; exhaustive `match`es need a new arm.
- `make`/`build` now take `impl AsRef<Path>` / `impl IntoIterator`. Ordinary
  calls with `&str` and `Vec` compile unchanged.
- Output XML is unchanged (byte-identical) as long as no extensions are used.

## FAQ

### How do I generate a sitemap with more than 50,000 URLs in Rust?

Use [`ChunkedSitemapWriter`](#splitting-large-sitemaps). It splits the URLs at
the protocol limit into `sitemap1.xml`, `sitemap2.xml`, ... and writes the
sitemap index for you.

### How do I serve a sitemap dynamically from axum / actix-web / Rocket?

Build the XML as a `String` with `SitemapWriter::build` and return it with the
`application/xml` content type:

```rust
// axum
async fn sitemap() -> impl IntoResponse {
    let xml = SitemapWriter::build(load_urls());
    ([(header::CONTENT_TYPE, "application/xml")], xml)
}
```

### Does it validate URLs, dates, or priorities?

No. The library escapes XML special characters but writes your values as-is;
validation is left to the caller by design.

### Is async supported?

Building a sitemap is pure in-memory string generation, so there is nothing to
await — call `SitemapWriter::build` from any context. If your runtime requires
it, wrap the file-writing `make` methods in `spawn_blocking`.

### How do I tell search engines where my sitemap is?

Add `Sitemap: https://example.com/sitemap.xml` to your `robots.txt`, or submit
it in Google Search Console / Bing Webmaster Tools.

## License

MIT License

## Author

[uiuifree](https://github.com/uiuifree)
