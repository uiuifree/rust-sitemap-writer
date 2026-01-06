use std::fmt::{Debug, Display, Formatter};

/// Represents a single URL entry in a sitemap.
///
/// # Examples
///
/// ## Creating with all fields
///
/// ```rust
/// use sitemap_writer::{SitemapUrl, SitemapChangeFreq};
///
/// let url = SitemapUrl {
///     loc: "https://example.com/page".to_string(),
///     lastmod: Some("2024-01-15".to_string()),
///     changefreq: Some(SitemapChangeFreq::WEEKLY),
///     priority: Some(0.8),
/// };
/// ```
///
/// ## Creating with only the URL
///
/// ```rust
/// use sitemap_writer::SitemapUrl;
///
/// let url = SitemapUrl::new("https://example.com/page");
/// ```
#[derive(Debug, Clone)]
pub struct SitemapUrl {
    /// The URL of the page. This is the only required field.
    ///
    /// Special characters like `<`, `>`, `&` will be automatically escaped.
    pub loc: String,

    /// The date of last modification of the page.
    ///
    /// Should be in W3C Datetime format (e.g., `2024-01-15` or `2024-01-15T12:00:00+00:00`).
    pub lastmod: Option<String>,

    /// How frequently the page is likely to change.
    ///
    /// This value provides general information to search engines and may not
    /// correlate exactly to how often they crawl the page.
    pub changefreq: Option<SitemapChangeFreq>,

    /// The priority of this URL relative to other URLs on your site.
    ///
    /// Valid values range from 0.0 to 1.0. The default priority of a page is 0.5.
    pub priority: Option<f32>,
}

impl Default for SitemapUrl {
    fn default() -> Self {
        SitemapUrl {
            loc: "".to_string(),
            lastmod: None,
            changefreq: None,
            priority: None,
        }
    }
}

impl SitemapUrl {
    /// Creates a new `SitemapUrl` with only the URL specified.
    ///
    /// All other fields (`lastmod`, `changefreq`, `priority`) will be `None`.
    ///
    /// # Arguments
    ///
    /// * `loc` - The URL of the page.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sitemap_writer::SitemapUrl;
    ///
    /// let url = SitemapUrl::new("https://example.com/about");
    /// assert_eq!(url.loc, "https://example.com/about");
    /// assert!(url.lastmod.is_none());
    /// ```
    pub fn new(loc: &str) -> SitemapUrl {
        SitemapUrl {
            loc: loc.to_string(),
            ..SitemapUrl::default()
        }
    }
}

/// Indicates how frequently the content at a URL is likely to change.
///
/// This value provides general information to search engines and may not
/// correlate exactly to how often they crawl the page.
///
/// # Examples
///
/// ```rust
/// use sitemap_writer::SitemapChangeFreq;
///
/// let freq = SitemapChangeFreq::DAILY;
/// assert_eq!(freq.to_string(), "daily");
/// ```
#[derive(Clone, PartialEq, Eq)]
pub enum SitemapChangeFreq {
    /// The page changes every time it is accessed.
    ALWAYS,
    /// The page changes hourly.
    HOURLY,
    /// The page changes daily.
    DAILY,
    /// The page changes weekly.
    WEEKLY,
    /// The page changes monthly.
    MONTHLY,
    /// The page changes yearly.
    YEARLY,
    /// The page is archived and will never change.
    NEVER,
}

impl Debug for SitemapChangeFreq {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}

impl Display for SitemapChangeFreq {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SitemapChangeFreq::ALWAYS => "always",
            SitemapChangeFreq::HOURLY => "hourly",
            SitemapChangeFreq::DAILY => "daily",
            SitemapChangeFreq::WEEKLY => "weekly",
            SitemapChangeFreq::MONTHLY => "monthly",
            SitemapChangeFreq::YEARLY => "yearly",
            SitemapChangeFreq::NEVER => "never",
        };
        write!(f, "{}", s)
    }
}
