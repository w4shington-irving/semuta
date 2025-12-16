
use tui_tree_widget::{TreeItem};
use std::io;
use crate::{db::{self, get_albums_by_artist_id, get_tracks_by_album_id}};

type NodeId = u32;


pub fn build_tree() -> io::Result<Vec<TreeItem<'static, NodeId>>> {
    let artists = db::get_artists();
    let mut artists_vec = Vec::new();
    let mut i: NodeId = 0;
    for artist in artists.unwrap() {
        
        let albums = get_albums_by_artist_id(artist.id);
        let mut albums_vec = Vec::new();
        for album in albums.unwrap() {
            
            let tracks = get_tracks_by_album_id(album.id);
            let mut tracks_vec = Vec::new();
            for track in tracks.unwrap() {
                i += 1;
                tracks_vec.push(TreeItem::new_leaf(i, track.title));
            }
            i += 1;
            albums_vec.push(TreeItem::new(i, album.title.clone(), tracks_vec)?);
        }
        i += 1;
        artists_vec.push(TreeItem::new(i, artist.name.clone(), albums_vec)?);
    }
    Ok(artists_vec)
}
