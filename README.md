# sitemap-writer

[![Crates.io](https://img.shields.io/crates/v/sitemap-writer.svg)](https://crates.io/crates/sitemap-writer)
[![Documentation](https://docs.rs/sitemap-writer/badge.svg)](https://docs.rs/sitemap-writer)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A simple and lightweight Rust library for generating XML sitemaps.

## Features

- Simple API for creating sitemaps
- Automatic XML escaping for special characters
- Support for all sitemap properties (`loc`, `lastmod`, `changefreq`, `priority`)
- Support for Sitemap Index (for large sites with 50,000+ URLs)
- Write directly to file or build as String
- No heavy dependencies

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sitemap-writer = "1.0"
```

## Quick Start

```rust
use sitemap_writer::{SitemapWriter, SitemapUrl, SitemapChangeFreq};

fn main() {
    let result = SitemapWriter::make("sitemap.xml", vec![
        SitemapUrl {
            loc: "https://example.com/".to_string(),
            lastmod: Some("2024-01-01".to_string()),
            changefreq: Some(SitemapChangeFreq::DAILY),
            priority: Some(1.0),
        },
        SitemapUrl::new("https://example.com/about/"),
    ]);

    assert!(result.is_ok());
}
```

## API Reference

### SitemapWriter

#### `SitemapWriter::make(path, urls)` - Write to File

```rust
use sitemap_writer::{SitemapWriter, SitemapUrl, SitemapChangeFreq};

let result = SitemapWriter::make("sitemap.xml", vec![
    SitemapUrl {
        loc: "https://example.com/".to_string(),
        lastmod: Some("2024-01-01".to_string()),
        changefreq: Some(SitemapChangeFreq::DAILY),
        priority: Some(1.0),
    },
    SitemapUrl::new("https://example.com/contact/"),
]);
```

#### `SitemapWriter::build(urls)` - Build as String

```rust
use sitemap_writer::{SitemapWriter, SitemapUrl};

let xml = SitemapWriter::build(vec![
    SitemapUrl::new("https://example.com/"),
    SitemapUrl::new("https://example.com/about/"),
]);
```

### SitemapUrl

| Field | Type | Description |
|-------|------|-------------|
| `loc` | `String` | **Required.** The URL of the page. |
| `lastmod` | `Option<String>` | The date of last modification (YYYY-MM-DD). |
| `changefreq` | `Option<SitemapChangeFreq>` | How frequently the page changes. |
| `priority` | `Option<f32>` | Priority relative to other URLs (0.0 to 1.0). |

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

### SitemapIndexWriter

#### `SitemapIndexWriter::make(path, sitemaps)` - Write to File

```rust
use sitemap_writer::{SitemapIndexWriter, SitemapIndex};

let result = SitemapIndexWriter::make("sitemap_index.xml", vec![
    SitemapIndex {
        loc: "https://example.com/sitemap1.xml".to_string(),
        lastmod: Some("2024-01-01".to_string()),
    },
    SitemapIndex::new("https://example.com/sitemap2.xml"),
]);
```

#### `SitemapIndexWriter::build(sitemaps)` - Build as String

```rust
use sitemap_writer::{SitemapIndexWriter, SitemapIndex};

let xml = SitemapIndexWriter::build(vec![
    SitemapIndex::new("https://example.com/sitemap1.xml"),
    SitemapIndex::new("https://example.com/sitemap2.xml"),
]);
```

### SitemapIndex

| Field | Type | Description |
|-------|------|-------------|
| `loc` | `String` | **Required.** The URL of the sitemap file. |
| `lastmod` | `Option<String>` | The date of last modification (YYYY-MM-DD). |

### Output Example

```xml
<?xml version="1.0" encoding="UTF-8"?>
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <sitemap>
    <loc>https://example.com/sitemap1.xml</loc>
    <lastmod>2024-01-01</lastmod>
  </sitemap>
  <sitemap>
    <loc>https://example.com/sitemap2.xml</loc>
  </sitemap>
</sitemapindex>
```

## License

MIT License

## Author

[uiuifree](https://github.com/uiuifree)
