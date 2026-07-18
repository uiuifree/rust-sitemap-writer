use std::path::Path;

use crate::error::SitemapError;
use crate::sitemap_index::{SitemapIndex, SitemapIndexWriter};
use crate::sitemap_url::SitemapUrl;
use crate::sitemap_writer::{SitemapWriter, write_file};

const DEFAULT_CHUNK_SIZE: usize = 50_000;

/// A writer that splits a large URL list into multiple sitemaps plus a
/// sitemap index.
///
/// The sitemap protocol limits a single sitemap to 50,000 URLs. This writer
/// splits the URLs into chunks of [`chunk_size`](ChunkedSitemapWriter::chunk_size)
/// (50,000 by default), names them `sitemap1.xml`, `sitemap2.xml`, ..., and
/// generates a `sitemap_index.xml` that references them.
///
/// Splitting only happens through this writer; [`SitemapWriter`] never splits
/// on its own.
///
/// # Examples
///
/// ```rust,no_run
/// use sitemap_writer::{ChunkedSitemapWriter, SitemapUrl};
///
/// // Writes public/sitemaps/sitemap1.xml, sitemap2.xml, ... and
/// // sitemap_index.xml whose entries point at
/// // https://example.com/sitemaps/sitemap1.xml, ...
/// let result = ChunkedSitemapWriter::new("https://example.com/sitemaps/").make(
///     "public/sitemaps",
///     (0..100_000).map(|i| SitemapUrl::new(format!("https://example.com/page/{}", i))),
/// );
/// assert!(result.is_ok());
/// ```
#[derive(Debug, Clone)]
pub struct ChunkedSitemapWriter {
    base_url: String,
    chunk_size: usize,
}

/// The result of [`ChunkedSitemapWriter::build`]: the split sitemaps and the
/// sitemap index as XML strings.
#[derive(Debug, Clone)]
pub struct ChunkedSitemap {
    /// The sitemap XML strings, in `sitemap1.xml`, `sitemap2.xml`, ... order.
    pub sitemaps: Vec<String>,
    /// The `sitemap_index.xml` XML string referencing all sitemaps.
    pub index: String,
}

impl ChunkedSitemapWriter {
    /// Creates a new `ChunkedSitemapWriter`.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The public URL under which the sitemap files will be
    ///   served (e.g. `https://example.com/sitemaps/`). It is used to build
    ///   the `<loc>` entries of the sitemap index. A trailing slash is
    ///   optional.
    pub fn new(base_url: impl Into<String>) -> ChunkedSitemapWriter {
        ChunkedSitemapWriter {
            base_url: base_url.into(),
            chunk_size: DEFAULT_CHUNK_SIZE,
        }
    }

    /// Sets the maximum number of URLs per sitemap (default: 50,000).
    ///
    /// # Panics
    ///
    /// Panics if `chunk_size` is 0.
    pub fn chunk_size(mut self, chunk_size: usize) -> ChunkedSitemapWriter {
        assert!(chunk_size > 0, "chunk_size must be greater than 0");
        self.chunk_size = chunk_size;
        self
    }

    /// Builds the split sitemaps and the sitemap index as XML strings.
    ///
    /// An empty URL list produces one empty sitemap referenced by the index.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sitemap_writer::{ChunkedSitemapWriter, SitemapUrl};
    ///
    /// let chunked = ChunkedSitemapWriter::new("https://example.com/")
    ///     .chunk_size(2)
    ///     .build(vec![
    ///         SitemapUrl::new("https://example.com/1"),
    ///         SitemapUrl::new("https://example.com/2"),
    ///         SitemapUrl::new("https://example.com/3"),
    ///     ]);
    /// assert_eq!(chunked.sitemaps.len(), 2);
    /// assert!(chunked.index.contains("<loc>https://example.com/sitemap2.xml</loc>"));
    /// ```
    pub fn build(&self, urls: impl IntoIterator<Item = SitemapUrl>) -> ChunkedSitemap {
        self.build_chunks(urls, "xml")
    }

    /// Writes the split sitemaps and the sitemap index into the specified
    /// directory as `sitemap1.xml`, `sitemap2.xml`, ... and
    /// `sitemap_index.xml`.
    ///
    /// The directory is created with `create_dir_all` if it does not exist.
    ///
    /// # Errors
    ///
    /// Returns [`SitemapError::CreateDir`] if the directory cannot be
    /// created, [`SitemapError::FileOpen`] if a file cannot be created, or
    /// [`SitemapError::Write`] if writing fails.
    pub fn make(
        &self,
        dir: impl AsRef<Path>,
        urls: impl IntoIterator<Item = SitemapUrl>,
    ) -> Result<(), SitemapError> {
        let dir = dir.as_ref();
        let chunked = self.build(urls);
        std::fs::create_dir_all(dir).map_err(|e| SitemapError::CreateDir(e.to_string()))?;
        for (i, sitemap) in chunked.sitemaps.iter().enumerate() {
            write_file(&dir.join(format!("sitemap{}.xml", i + 1)), sitemap)?;
        }
        write_file(&dir.join("sitemap_index.xml"), &chunked.index)
    }

    /// Writes the split sitemaps and the sitemap index into the specified
    /// directory as gzip-compressed `sitemap1.xml.gz`, `sitemap2.xml.gz`, ...
    /// and `sitemap_index.xml.gz`. The index `<loc>` entries also point at
    /// the `.xml.gz` files.
    ///
    /// The directory is created with `create_dir_all` if it does not exist.
    ///
    /// Requires the `gzip` feature.
    ///
    /// # Errors
    ///
    /// Returns [`SitemapError::CreateDir`] if the directory cannot be
    /// created, [`SitemapError::FileOpen`] if a file cannot be created, or
    /// [`SitemapError::Write`] if compressing or writing fails.
    #[cfg(feature = "gzip")]
    pub fn make_gzip(
        &self,
        dir: impl AsRef<Path>,
        urls: impl IntoIterator<Item = SitemapUrl>,
    ) -> Result<(), SitemapError> {
        let dir = dir.as_ref();
        let chunked = self.build_chunks(urls, "xml.gz");
        std::fs::create_dir_all(dir).map_err(|e| SitemapError::CreateDir(e.to_string()))?;
        for (i, sitemap) in chunked.sitemaps.iter().enumerate() {
            crate::gzip::write_gzip(&dir.join(format!("sitemap{}.xml.gz", i + 1)), sitemap)?;
        }
        crate::gzip::write_gzip(&dir.join("sitemap_index.xml.gz"), &chunked.index)
    }

    fn build_chunks(
        &self,
        urls: impl IntoIterator<Item = SitemapUrl>,
        ext: &str,
    ) -> ChunkedSitemap {
        let mut sitemaps = Vec::new();
        let mut urls = urls.into_iter();
        loop {
            let chunk: Vec<SitemapUrl> = urls.by_ref().take(self.chunk_size).collect();
            if chunk.is_empty() && !sitemaps.is_empty() {
                break;
            }
            let is_last = chunk.len() < self.chunk_size;
            sitemaps.push(SitemapWriter::build(chunk));
            if is_last {
                break;
            }
        }
        let index = SitemapIndexWriter::build(
            (1..=sitemaps.len()).map(|n| SitemapIndex::new(self.file_url(n, ext))),
        );
        ChunkedSitemap { sitemaps, index }
    }

    fn file_url(&self, n: usize, ext: &str) -> String {
        let separator = if self.base_url.ends_with('/') {
            ""
        } else {
            "/"
        };
        format!("{}{}sitemap{}.{}", self.base_url, separator, n, ext)
    }
}
