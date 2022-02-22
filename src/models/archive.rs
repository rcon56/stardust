
use serde::{Serialize, Serializer};
use time::Month;

use super::entry::Entry;
use super::page::Block;


#[derive(Debug, Clone, Serialize)]
pub struct Archive {
    pub year: i32,
    #[serde(serialize_with = "serialize_month")]
    pub month: Month,
    pub entries: Vec<Entry>,
}

impl Block for Archive {
    fn kind(&self) -> &str {
        "archive"
    }
}

fn serialize_month<S>(m: &Month, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&m.to_string())
}