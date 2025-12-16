#[derive(Debug, Clone)]
pub struct Track {
    pub title: String,
    pub album: String,
    pub artist: String,
    pub track_number: Option<u32>,
    pub duration_secs: u32,
    pub path: String,
    pub id: i64,
}
