#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Track {
    pub title: String,
    pub id: i64,
    pub album_name: String,
    pub album_id: i64,
    pub artist_name: String,    
    pub artist_id: i64,
    pub track_number: Option<u32>, // None if track is not part of an album
    pub duration_secs: u32,
    pub path: String,
}

