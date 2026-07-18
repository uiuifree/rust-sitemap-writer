use std::fmt::{Debug, Display, Formatter};

use crate::extensions::{SitemapAlternate, SitemapImage, SitemapNews, SitemapVideo};

/// Represents a single URL entry in a sitemap.
///
/// # Examples
///
/// ## Creating with the builder API
///
/// ```rust
/// use sitemap_writer::{SitemapUrl, SitemapChangeFreq};
///
/// let url = SitemapUrl::new("https://example.com/page")
///     .lastmod("2024-01-15")
///     .changefreq(SitemapChangeFreq::WEEKLY)
///     .priority(0.8);
/// ```
///
/// ## Creating with a struct literal
///
/// ```rust
/// use sitemap_writer::SitemapUrl;
///
/// let url = SitemapUrl {
///     loc: "https://example.com/page".to_string(),
///     lastmod: Some("2024-01-15".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default)]
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

    /// Images on the page, for the Google image sitemap extension.
    pub images: Vec<SitemapImage>,

    /// Videos on the page, for the Google video sitemap extension.
    pub videos: Vec<SitemapVideo>,

    /// News article metadata, for the Google news sitemap extension.
    pub news: Option<SitemapNews>,

    /// Alternate language/region versions of the page (`hreflang`).
    pub alternates: Vec<SitemapAlternate>,
}

impl SitemapUrl {
    /// Creates a new `SitemapUrl` with only the URL specified.
    ///
    /// All other fields are empty. Use the builder methods to set them.
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
    pub fn new(loc: impl Into<String>) -> SitemapUrl {
        SitemapUrl {
            loc: loc.into(),
            ..SitemapUrl::default()
        }
    }

    /// Sets the date of last modification, in W3C Datetime format.
    pub fn lastmod(mut self, lastmod: impl Into<String>) -> SitemapUrl {
        self.lastmod = Some(lastmod.into());
        self
    }

    /// Sets how frequently the page is likely to change.
    pub fn changefreq(mut self, changefreq: SitemapChangeFreq) -> SitemapUrl {
        self.changefreq = Some(changefreq);
        self
    }

    /// Sets the priority of this URL relative to other URLs (0.0 to 1.0).
    pub fn priority(mut self, priority: f32) -> SitemapUrl {
        self.priority = Some(priority);
        self
    }

    /// Adds an image to this URL entry. Can be called multiple times.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sitemap_writer::{SitemapUrl, SitemapImage};
    ///
    /// let url = SitemapUrl::new("https://example.com/")
    ///     .image(SitemapImage::new("https://example.com/photo.jpg"));
    /// assert_eq!(url.images.len(), 1);
    /// ```
    pub fn image(mut self, image: SitemapImage) -> SitemapUrl {
        self.images.push(image);
        self
    }

    /// Adds a video to this URL entry. Can be called multiple times.
    pub fn video(mut self, video: SitemapVideo) -> SitemapUrl {
        self.videos.push(video);
        self
    }

    /// Sets the news article metadata for this URL entry.
    pub fn news(mut self, news: SitemapNews) -> SitemapUrl {
        self.news = Some(news);
        self
    }

    /// Adds an alternate language/region link to this URL entry.
    /// Can be called multiple times.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sitemap_writer::{SitemapUrl, SitemapAlternate};
    ///
    /// let url = SitemapUrl::new("https://example.com/en/")
    ///     .alternate(SitemapAlternate::new("en", "https://example.com/en/"))
    ///     .alternate(SitemapAlternate::new("ja", "https://example.com/ja/"));
    /// assert_eq!(url.alternates.len(), 2);
    /// ```
    pub fn alternate(mut self, alternate: SitemapAlternate) -> SitemapUrl {
        self.alternates.push(alternate);
        self
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
