
struct TimeLineEntry {
    pub title: String,
    pub date: String,   
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TimeLine {
    pub year: u8,
    pub month: u8,
    pub entries: Vec<TimeLineEntry>,
}