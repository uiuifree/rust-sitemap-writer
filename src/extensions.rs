/// An image entry for the Google image sitemap extension (`<image:image>`).
///
/// Attach images to a [`SitemapUrl`](crate::SitemapUrl) with
/// [`SitemapUrl::image`](crate::SitemapUrl::image). Google allows up to
/// 1,000 images per page.
///
/// # Examples
///
/// ```rust
/// use sitemap_writer::SitemapImage;
///
/// let image = SitemapImage::new("https://example.com/photo.jpg");
/// assert_eq!(image.loc, "https://example.com/photo.jpg");
/// ```
#[derive(Debug, Clone)]
pub struct SitemapImage {
    /// The URL of the image.
    ///
    /// Special characters like `<`, `>`, `&` will be automatically escaped.
    pub loc: String,
}

impl SitemapImage {
    /// Creates a new `SitemapImage` with the image URL.
    pub fn new(loc: impl Into<String>) -> SitemapImage {
        SitemapImage { loc: loc.into() }
    }
}

/// A video entry for the Google video sitemap extension (`<video:video>`).
///
/// `thumbnail_loc`, `title`, and `description` are required by Google and are
/// taken by [`SitemapVideo::new`]. In addition, Google requires either
/// `content_loc` or `player_loc` to be set; this library does not validate
/// that requirement.
///
/// # Examples
///
/// ```rust
/// use sitemap_writer::SitemapVideo;
///
/// let video = SitemapVideo::new(
///     "https://example.com/thumbnail.jpg",
///     "Video title",
///     "Video description",
/// )
/// .content_loc("https://example.com/video.mp4")
/// .duration(600);
/// ```
#[derive(Debug, Clone)]
pub struct SitemapVideo {
    /// The URL of the video thumbnail image. Required by Google.
    pub thumbnail_loc: String,
    /// The title of the video. Required by Google.
    pub title: String,
    /// The description of the video. Required by Google.
    pub description: String,
    /// The URL of the actual video media file.
    ///
    /// Google requires either this or `player_loc`.
    pub content_loc: Option<String>,
    /// The URL of a player for the video.
    ///
    /// Google requires either this or `content_loc`.
    pub player_loc: Option<String>,
    /// The duration of the video in seconds (0 to 28800).
    pub duration: Option<u32>,
    /// The date after which the video is no longer available,
    /// in W3C Datetime format.
    pub expiration_date: Option<String>,
    /// The date the video was first published, in W3C Datetime format.
    pub publication_date: Option<String>,
}

impl SitemapVideo {
    /// Creates a new `SitemapVideo` with the fields required by Google.
    ///
    /// All optional fields are `None`. Google additionally requires either
    /// [`content_loc`](SitemapVideo::content_loc) or
    /// [`player_loc`](SitemapVideo::player_loc) to be set.
    pub fn new(
        thumbnail_loc: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> SitemapVideo {
        SitemapVideo {
            thumbnail_loc: thumbnail_loc.into(),
            title: title.into(),
            description: description.into(),
            content_loc: None,
            player_loc: None,
            duration: None,
            expiration_date: None,
            publication_date: None,
        }
    }

    /// Sets the URL of the actual video media file.
    pub fn content_loc(mut self, content_loc: impl Into<String>) -> SitemapVideo {
        self.content_loc = Some(content_loc.into());
        self
    }

    /// Sets the URL of a player for the video.
    pub fn player_loc(mut self, player_loc: impl Into<String>) -> SitemapVideo {
        self.player_loc = Some(player_loc.into());
        self
    }

    /// Sets the duration of the video in seconds (0 to 28800).
    pub fn duration(mut self, duration: u32) -> SitemapVideo {
        self.duration = Some(duration);
        self
    }

    /// Sets the date after which the video is no longer available,
    /// in W3C Datetime format.
    pub fn expiration_date(mut self, expiration_date: impl Into<String>) -> SitemapVideo {
        self.expiration_date = Some(expiration_date.into());
        self
    }

    /// Sets the date the video was first published, in W3C Datetime format.
    pub fn publication_date(mut self, publication_date: impl Into<String>) -> SitemapVideo {
        self.publication_date = Some(publication_date.into());
        self
    }
}

/// A news entry for the Google news sitemap extension (`<news:news>`).
///
/// All fields are required by Google. A news entry describes a single
/// article, so a [`SitemapUrl`](crate::SitemapUrl) holds at most one
/// `SitemapNews`.
///
/// # Examples
///
/// ```rust
/// use sitemap_writer::SitemapNews;
///
/// let news = SitemapNews::new("The Example Times", "en", "2024-01-15", "Article title");
/// assert_eq!(news.publication_name, "The Example Times");
/// ```
#[derive(Debug, Clone)]
pub struct SitemapNews {
    /// The name of the news publication.
    pub publication_name: String,
    /// The language of the publication (ISO 639 code, e.g. `en` or `ja`).
    pub publication_language: String,
    /// The article publication date in W3C Datetime format.
    pub publication_date: String,
    /// The title of the news article.
    pub title: String,
}

impl SitemapNews {
    /// Creates a new `SitemapNews`. All fields are required by Google.
    pub fn new(
        publication_name: impl Into<String>,
        publication_language: impl Into<String>,
        publication_date: impl Into<String>,
        title: impl Into<String>,
    ) -> SitemapNews {
        SitemapNews {
            publication_name: publication_name.into(),
            publication_language: publication_language.into(),
            publication_date: publication_date.into(),
            title: title.into(),
        }
    }
}

/// An alternate language/region link (`<xhtml:link rel="alternate">`)
/// for multilingual sitemaps.
///
/// # Examples
///
/// ```rust
/// use sitemap_writer::SitemapAlternate;
///
/// let alternate = SitemapAlternate::new("ja", "https://example.com/ja/");
/// assert_eq!(alternate.hreflang, "ja");
/// ```
#[derive(Debug, Clone)]
pub struct SitemapAlternate {
    /// The language/region code (e.g. `en`, `ja`, `x-default`).
    pub hreflang: String,
    /// The URL of the alternate version of the page.
    pub href: String,
}

impl SitemapAlternate {
    /// Creates a new `SitemapAlternate`.
    pub fn new(hreflang: impl Into<String>, href: impl Into<String>) -> SitemapAlternate {
        SitemapAlternate {
            hreflang: hreflang.into(),
            href: href.into(),
        }
    }
}
