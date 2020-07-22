use std::fmt;

#[derive(Debug)]
pub struct Bookmark {
    pub id: i64,
    pub url: String,
    pub title: String,
    pub tags: Vec<String>,
}

impl Bookmark {
    pub fn new(id: i64, title: String, url: String, tags: Vec<String>) -> Self {
        Bookmark { id, title, url, tags }
    }
}

impl fmt::Display for Bookmark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let space = "    ";
        let bookmark = format!("{} {}\n{}{}\n", self.id, self.title, space, self.url);
        if self.tags.is_empty() {
            write!(f, "{}", bookmark)?;
        } else {
            write!(f, "{}{}[ {} ]\n", bookmark, space, self.tags.join(", "))?;
        }
        Ok(())
    }
}