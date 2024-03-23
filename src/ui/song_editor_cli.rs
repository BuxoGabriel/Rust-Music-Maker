use std::{fs::File, io::{self, BufReader, Read, Write}};
use rfd::FileDialog;

use crate::music::{Serializable, Song, SongEditor};
use super::{choice_ui::{self, Choice}, song_cli::edit_song_ui};

pub fn ui(editor: &mut SongEditor) {
    let choices = vec![
        Choice::new("Load Song".to_string(), Box::from(load_ui)),
        Choice::new("Create Song".to_string(), Box::from(create_ui)),
        Choice::new("Edit Song".to_string(), Box::from(edit_ui)),
    ];
    println!("Hello! Welcome to Song Maker!");
    loop {
        if let Some(res) = choice_ui::ui_offer_choices(&choices, editor) {
            if let Err(err) = res {
                println!("{err}");
            }
        }
        else {
            break
        }
    }
    println!("Goodbye from Song Maker!")
}

fn create_ui(editor: &mut SongEditor) -> Result<(), &'static str>{
    // Get song name from user
    print!("Song name: ");
    io::stdout().flush().expect("Failed to flush stdout! Exiting!");
    let mut song_name = String::new();
    io::stdin().read_line(&mut song_name).expect("Failed to read song name!");
    let song_name = song_name.trim().to_string();
    // If a song already exists with chosen name ask if they would like to overwrite
    if let Some((index, _song)) = editor.loaded_songs.iter().enumerate().find(|(_index, song)| song.name.as_str() == song_name) {
        println!("A song already exists with this name would you like to overwrite it? (y/n)");
        let mut overwrite = String::new();
        io::stdin().read_line(&mut overwrite).expect("Failed to read user input!");
        match overwrite.trim() {
            "y" | "yes" | "Y" | "YES" => {
                // If user decides to overwrite file delete file from memory
                editor.loaded_songs.remove(index);
            }
            _ => {
                // Else fail and return early
                println!("Create song aborted!");
                return Err("Create song aborted because didn't overwrite existing song with same name!");
            }
        }
    }
    // TODO: Get BPM from user
    // Create new song and add it to editor
    editor.loaded_songs.push(Song::new(song_name, 120));
    println!("Created song!");
    Ok(())
}

fn load_ui(editor: &mut SongEditor) -> Result<(), &'static str>{
    // Get file from the user
    println!("Select a .song file to load");
    // Show file dialog
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
                println!("could not read from file!");
                return Err("could not read from file!");
            }
            // Attempt to deserialize song from file
            match Song::deserialize(&serialized_data) {
                Ok(song) => {
                    editor.loaded_songs.push(song);
                    println!("Loaded Song!");
                    Ok(())
                }
                Err(err) => {
                    println!("{err}");
                    Err(err)
                }
            }
        }
        else {
            println!("failed to open file");
            Err("failed to open file")
        }
    }
    else {
        println!("No files selected or file failed to open!");
        Err("No files selected or file failed to open!")
    }
}

fn edit_ui(editor: &mut SongEditor) -> Result<(), &'static str> {
    show_songs_ui(editor);
    // User selects song to edit
    println!("Which song would you like to edit?");
    let mut buf = String::new();
    if let Err(_err) = io::stdin().read_line(&mut buf) {
        println!("Failed to read user input");
        return Err("Failed to read user input");
    }
    let edit_index = buf.trim();
    if let Ok(song_index) = edit_index.parse::<usize>() {
        if let Some(song) = editor.loaded_songs.get_mut(song_index) {
            edit_song_ui(song);
            Ok(())
        } else {
            Err("Provided invalid song index")
        }
    } else {
        println!("Failed to parse user input as number!");
        Err("Failed to parse user input as number!")
    }
}

fn show_songs_ui(editor: &mut SongEditor) {
    println!("Available Songs:");
    for (index, song) in editor.loaded_songs.iter().enumerate() {
        println!("\t{}. {}", index + 1, song.name);
    }
}