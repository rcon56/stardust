use core::cmp::Ordering;

use anyhow;
use serde::{Serialize, Deserialize, Serializer};
use time::{Date, Time};
use time::format_description::FormatItem;
use time::macros::{format_description as fd};
// use time::macros::format_description;

use super::page::Block;

const KIND: &str = "post";
const POST_DATE_FORMAT: &[FormatItem] = fd!("[month repr:short] [day padding:none] [year]");
const POST_TIME_FORMAT: &[FormatItem] = fd!("[hour]:[minute]:[second]");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Front {
    pub date: String,
    pub time: String,
    pub author: Option<String>,
    pub title: String,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, Serialize)]
pub struct Post {
    #[serde(serialize_with = "serialize_date")]
    pub date: Date,
    #[serde(serialize_with = "serialize_time")]
    pub time: Time,
    pub author: String,
    pub title: String,
    pub category: String,
    pub tags: Vec<String>,
    pub content: String,
    pub url: String,
}

impl Post {
    pub fn write(front: Front, url: String, content: String) -> anyhow::Result<Post> {
        Ok(Post {
            date: Date::parse(&front.date, &POST_DATE_FORMAT)?,
            time: Time::parse(&front.time, &POST_TIME_FORMAT)?,
            author: front.author.unwrap_or("Unnamed".to_string()),
            title: front.title,
            category: front.category.unwrap_or("uncategorized".to_string()),
            tags: front.tags.unwrap_or(vec![]),
            content: content,
            url: url,
        })
    }

    pub fn digest(&self) -> &str {
        // if self.content.len() < 100 { &self.content } else { &self.content[0..100] }
        "..."
    }

    pub fn date_str(&self) -> String {
        self.date.format(&POST_DATE_FORMAT).unwrap_or("Jan 1 1970".to_string())
    }

    pub fn time_str(&self) -> String {
        self.date.format(&POST_TIME_FORMAT).unwrap_or("00:00:00".to_string())
    }
}

impl Block for Post {
    fn kind(&self) -> &str {
        KIND
    }
}

impl PartialOrd for Post {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.date == other.date {
            return Some(self.time.cmp(&other.time));
        }
        Some(self.date.cmp(&other.date))
    }
}


fn serialize_date<S>(dt: &Date, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&dt.format(&POST_DATE_FORMAT).unwrap_or("Jan 1 1970".to_string()))
}

fn serialize_time<S>(dt: &Time, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&dt.format(&POST_TIME_FORMAT).unwrap_or("00:00:00".to_string()))
}

#[cfg(test)]
mod test {
    use super::*;
    use time::macros::{date, time};

    #[test]
    fn parse_date() -> time::Result<()> {

        assert_eq!(
            Date::parse("Feb 6 2022", &POST_DATE_FORMAT)?,
            date!(2022 - 2 - 6)
        );

        Ok(())
    }

    #[test]
    fn parse_time() -> time::Result<()> {

        assert_eq!(
            Time::parse("14:30:11", &POST_TIME_FORMAT)?,
            time!(14:30:11)
        );

        Ok(())
    }
}