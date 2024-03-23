use crate::music::Song;

pub struct SongEditor {
    pub loaded_songs: Vec<Song>
}

impl SongEditor {
    pub fn new() -> Self {
        SongEditor { loaded_songs: vec![Song::default()] }
    }
}