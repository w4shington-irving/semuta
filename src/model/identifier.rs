
#[derive(Debug, Clone)]
pub enum ArtistIdentifier<'a> {
    Id(i64),
    Name(&'a str),
}

#[derive(Debug, Clone)]
pub enum AlbumIdentifier<'a> {
    Id(i64),
    Name{name: &'a str, artist_id: i64},
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum TrackIdentifier<'a> {
    Id(i64),
    Name{name: &'a str, album_id: i64},
}