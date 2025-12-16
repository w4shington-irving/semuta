use tui_tree_widget::{TreeItem};
use std::io;
use crate::{db::{self, get_albums_by_artist_id, get_tracks_by_album_id}};


use crate::ui::NodeId;

pub fn build_tree() -> io::Result<Vec<TreeItem<'static, NodeId>>> {
    let artists = db::get_artists();
    let mut artists_vec = Vec::new();
    for artist in artists.unwrap() {
        let albums = get_albums_by_artist_id(artist.id);
        let mut albums_vec = Vec::new();
        for album in albums.unwrap() {
            let mut tracks_vec = Vec::new();
            let tracks = get_tracks_by_album_id(album.id);
            for track in tracks.unwrap() {
                tracks_vec.push(TreeItem::new_leaf(NodeId::Track(track.id),
                track.title));
            }
            albums_vec.push(
                TreeItem::new(NodeId::Album(album.id),
                album.title.clone(), 
                tracks_vec)?);
        }
        artists_vec.push(TreeItem::new(NodeId::Artist(artist.id),
        artist.name.clone(),
        albums_vec)?);
    }

    Ok(artists_vec)
}
