use std::fs::File;
use::std::io::{self, Write, Read, BufReader};
use super::{song::Song, serializable::Serializable};
use crate::wav::WavOptions;
use rfd::FileDialog;

///
pub struct SongEditor {
    loaded_songs: Vec<Song>
}

impl SongEditor {
    pub fn new() -> Self {
        SongEditor { loaded_songs: vec![Song::default()] }
    }

    pub fn ui(&mut self) {
        println!("Hello! Welcome to Song Maker!");
        'ui: loop {
            self.show_songs_ui();
            println!("Select one of the options listed below:");
            println!("\t1. Load Song");
            println!("\t2. Save Song");
            println!("\t3. Create Song");
            println!("\t4. Edit Song");
            println!("\t5. Export Song to wav file");
            println!("\t6. Exit Song Maker");
            let mut option = String::new();
            loop {
                io::stdin().read_line(&mut option).expect("Could not read user input");
                match option.trim() {
                    "1" => {
                        self.load_ui();
                        break;
                    }
                    "2" => {
                        self.save_ui();
                        break;
                    }
                    "3" => {
                        self.create_ui();
                        break;
                    }
                    "4" => {
                        self.edit_ui();
                        break;
                    }
                    "5" => {
                        self.compile_ui();
                        break;
                    }
                    "6" | "q" | "Q" => {
                        break 'ui;
                    }
                    _ => {
                        println!("option not recognized! press q to quit or select another option!");
                    }
                }
            }
        }
        println!("Goodbye from Song Maker!")
    }

    fn show_songs_ui(&mut self) {
        println!("Available Songs:");
        for (index, song) in self.loaded_songs.iter().enumerate() {
            println!("\t{index}. {}", song.name());
        }
    }

    fn create_ui(&mut self) {
        print!("Song name: ");
        io::stdout().flush().expect("Failed to flush stdout! Exiting!");
        let mut song_name = String::new();
        io::stdin().read_line(&mut song_name).expect("Failed to read song name!");
        let song_name = song_name.trim().to_string();
        if let Some((index, _song)) = self.loaded_songs.iter().enumerate().find(|(_index, song)| song.name().as_str() == song_name) {
            println!("A song already exists with this name would you like to overwrite it? (y/n)");
            let mut overwrite = String::new();
            io::stdin().read_line(&mut overwrite).expect("Failed to read user input!");
            match overwrite.trim() {
                "y" | "yes" | "Y" | "YES" => {
                    self.loaded_songs.remove(index);
                }
                _ => {
                    println!("Create song aborted!");
                    return;
                }
            }
        }
        self.loaded_songs.push(Song::new(song_name));
        println!("Created song!")
    }

    fn edit_ui(&mut self) {
        loop {
            println!("Which song would you like to edit?");
            let mut edit_index = String::new();
            io::stdin().read_line(&mut edit_index).expect("Failed to read user input!");
            let edit_index = edit_index.trim();
            if let Ok(index) = edit_index.parse::<usize>() {
                if let Some(_song) = self.loaded_songs.get(index) {
                    // Editing Song ui

                    // TODO
                    println!("Editing Song...");
                    println!("Done editing Song!");
                    break
                }
            }
            println!("{edit_index} is not a valid song index!")
        } 
    }

    fn load_ui(&mut self) {
        // Get file from the user
        println!("Select a .song file to load");
        // Show file dialog
        let file = FileDialog::new()
            .add_filter("songs", &["song"])
            .set_directory("/")
            .pick_file();
        match file {
            Some(file_path) => {
                if let Ok(f) = File::open(&file_path) {
                    let mut buf_reader = BufReader::new(f);
                    let mut serialized_data: Vec<u8> = Vec::new();
                    if let Err(_err) = buf_reader.read_to_end(&mut serialized_data) {
                        println!("could not read from file!");
                        return;
                    }

                    match Song::deserialize(&serialized_data) {
                        Ok(song) => {
                            self.loaded_songs.push(song);
                            println!("Loaded Song!");
                            return;
                        }
                        Err(err) => {
                            println!("{err}");
                        }
                    }
                }
            }
            None => {
                println!("No files selected or file failed to open!");
                return;
            }
        }
    }

    fn save_ui(&self) {
        loop {
            println!("Which song would you like to save?");
            let mut save_index = String::new();
            io::stdin().read_line(&mut save_index).expect("Failed to read user input!");
            let save_index = save_index.trim();
            if let Ok(index) = save_index.parse::<usize>() {
                if let Some(song) = self.loaded_songs.get(index) {
                    println!("saving song...");
                    song.write_to_song_file(song.name().clone());
                    println!("saving complete!");
                    break
                }
            }
            println!("{save_index} is not a valid song index!")
        } 
    }

    fn compile_ui(&self) {
        loop {
            println!("Which song would you like to compile?");
            let mut compile_index: String = String::new();
            io::stdin().read_line(&mut compile_index).expect("Failed to read user input!");
            let compile_index = compile_index.trim();
            if let Ok(index) = compile_index.parse::<usize>() {
                if let Some(song) = self.loaded_songs.get(index) {
                    println!("Compiling song...");
                    // TODO get wavoptions from user optionally
                    song.write_to_wav_file(song.name().clone(), &WavOptions::default());
                    println!("Compilation complete!");
                    break
                }
            }
            println!("{compile_index} is not a valid song index!")
        } 
    }
}