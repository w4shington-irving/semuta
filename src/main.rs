mod model;
mod library;






fn main() {
    // let file_path = "song.flac";
    let music_dir = "/home/washington/Music";
    //let track = library::read::read_track(file_path);

    let tracks = library::scan::scan_files(music_dir);
    print!("{:#?}", tracks);
}