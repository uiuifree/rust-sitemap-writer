use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::Write;

pub enum SitemapError {
    FileOpen(String),
    Write(String),
}

pub struct SitemapWriter {}

impl SitemapWriter {
    pub fn make(path: &str, urls: Vec<SitemapUrl>)
                -> Result<(), SitemapError>
    {
        let file = File::create(path);
        if file.is_err() {
            return Err(SitemapError::FileOpen(file.err().unwrap().to_string()));
        }
        let file = file.unwrap();
        write_text(&file, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
        write_text(&file, r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#)?;

        for url in urls {
            let mut row = "<url>".to_string();
            row += format!("<loc>{}</loc>", html_escape::encode_text(url.loc.as_str())).as_str();
            if url.lastmod.is_some() {
                row += format!("<lastmod>{}</lastmod>", url.lastmod.unwrap_or_default()).as_str();
            }
            if url.changefreq.is_some() {
                row += format!("<changefreq>{}</changefreq>", url.changefreq.unwrap().to_string()).as_str();
            }
            if url.priority.is_some() {
                row += format!("<priority>{}</priority>", url.priority.unwrap()).as_str();
            }

            row += "</url>";
            write_text(&file, row.as_str())?;
        }
        write_text(&file, r#"</urlset> "#)?;
        Ok(())
    }
}


fn write_text(mut file: &File, str: &str) -> Result<(), SitemapError> {
    let f = file.write(str.as_bytes());
    if f.is_err() {
        return Err(SitemapError::Write(f.err().unwrap().to_string()));
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct SitemapUrl {
    pub loc: String,
    pub lastmod: Option<String>,
    pub changefreq: Option<SitemapChangeFreq>,
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
    pub fn new(loc: &str) -> SitemapUrl {
        SitemapUrl {
            loc: loc.to_string(),
            ..SitemapUrl::default()
        }
    }
}

#[derive(Clone)]
pub enum SitemapChangeFreq {
    ALWAYS,
    HOURLY,
    DAILY,
    WEEKLY,
    MONTHLY,
    YEARLY,
    NEVER,
}

impl Debug for SitemapChangeFreq {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}

impl SitemapChangeFreq {
    fn to_string(&self) -> String {
        match self {
            SitemapChangeFreq::ALWAYS => { "always".to_string() }
            SitemapChangeFreq::HOURLY => { "hourly".to_string() }
            SitemapChangeFreq::DAILY => { "daily".to_string() }
            SitemapChangeFreq::WEEKLY => { "weekly".to_string() }
            SitemapChangeFreq::MONTHLY => { "monthly".to_string() }
            SitemapChangeFreq::YEARLY => { "yearly".to_string() }
            SitemapChangeFreq::NEVER => { "never".to_string() }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{SitemapChangeFreq, SitemapUrl, SitemapWriter};

    #[test]
    fn test_make() {
        let res = SitemapWriter::make("test.xml", vec![]);
        assert!(res.is_ok());

        let res = SitemapWriter::make("test.xml", vec![
            SitemapUrl {
                loc: "https://example.com/".to_string(),
                lastmod: Some("2021-01-01".to_string()),
                changefreq: Some(SitemapChangeFreq::ALWAYS),
                priority: Some(1.0),
            },
            SitemapUrl::new("https://example.com/contact/"),
            SitemapUrl::new("https://example.com/contact/?test=1"),
            SitemapUrl::new("https://example.com/contact/?test=<>"),
        ]);
        assert!(res.is_ok());
    }
}
