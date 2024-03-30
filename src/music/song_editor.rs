use std::{fs::File, io::{BufReader, Read}};

use rfd::FileDialog;

use crate::music::Song;

use super::Serializable;

pub struct SongEditor {
    pub loaded_songs: Vec<Song>
}

impl SongEditor {
    pub fn new() -> Self {
        SongEditor { loaded_songs: vec![Song::default()] }
    }
    pub fn load_song(&mut self) -> Result<(), &'static str> {
        let file = FileDialog::new()
            .add_filter("songs", &["song"])
            .set_directory("/")
            .pick_file();
        if let Some(file_path) = file {
            // If there is a file then deserialize it and load it into memory
            if let Ok(f) = File::open(&file_path) {
                let mut buf_reader = BufReader::new(f);
                let mut serialized_data: Vec<u8> = Vec::new();
                if let Err(_err) = buf_reader.read_to_end(&mut serialized_data) {
                    return Err("could not read from file!");
                }
                // Attempt to deserialize song from file
                match Song::deserialize(&serialized_data) {
                    Ok(song) => {
                        self.loaded_songs.push(song);
                        println!("Loaded Song!");
                        Ok(())
                    }
                    Err(err) => {
                        Err(err)
                    }
                }
            }
            else {
                Err("failed to open file")
            }
        }
        else {
            Err("No files selected or file failed to open!")
        }
    }
}