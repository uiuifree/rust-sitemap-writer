## features
Simple Write Sitemap

## Using 
```
[dependencies]
sitemap-writer="0.1"
```

# Examples
```rust
use sitemap::{SitemapWriter,SitemapUrl,SitemapChangeFreq};
let write = SitemapWriter::make("test.xml", vec![
  SitemapUrl{
     loc: "https://example.com/".to_string(),
     lastmod: Some("2021-01-01".to_string()),
     changefreq: Some(SitemapChangeFreq::ALWAYS),
     priority: Some(1.0),
 },
 SitemapUrl::new("https://example.com/contact/"),
 SitemapUrl::new("https://example.com/contact/?test=1"),
 SitemapUrl::new("https://example.com/contact/?test=<>"),
]);
assert!(write.is_ok())
```
# Output XML
```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
      <loc>https://example.com/</loc>
      <lastmod>2021-01-01</lastmod>
      <changefreq>always</changefreq>
      <priority>1</priority>
  </url>
  <url>
      <loc>https://example.com/contact/</loc>
  </url>
  <url>
      <loc>https://example.com/contact/?test=1</loc>
  </url>
  <url>
      <loc>https://example.com/contact/?test=&lt;&gt;</loc>
  </url>
</urlset>
```