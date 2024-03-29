use std::{fs::File, io::{self, BufReader, Read, Write}};
use rfd::FileDialog;

use crate::music::{Serializable, Song, SongEditor};
use super::{choice_ui::{self, Choice}, song_cli};

pub fn ui(editor: &mut SongEditor) {
    let choices = vec![
        Choice::new("Load Song".to_string(), Box::from(load_song_ui)),
        Choice::new("Add Song".to_string(), Box::from(add_song_ui)),
        Choice::new("Delete Song".to_string(), Box::from(delete_song_ui)),
        Choice::new("Edit Song".to_string(), Box::from(edit_song_ui)),
    ];
    println!("Hello! Welcome to Song Maker!");
    loop {
        println!("Song Maker: Editing Songs");
        show_songs_ui(editor);
        if let Some(res) = choice_ui::ui_offer_choices(&choices, editor) {
            if let Err(err) = res {
                println!("{err}");
            }
        }
        else {
            break
        }
    }
    println!("Goodbye!")
}

fn add_song_ui(editor: &mut SongEditor) -> Result<(), &'static str>{
    // Get song name from user
    print!("Song name: ");
    io::stdout().flush().expect("Failed to flush stdout! Exiting!");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read song name!");
    let song_name = buffer.trim().to_string();
    if let Some((index, _song)) = editor.loaded_songs.iter().enumerate().find(|(_index, song)| song.name.as_str() == song_name) {
        // If a song already exists with chosen name ask if they would like to overwrite
        println!("A song already exists with this name would you like to overwrite it? (y/n)");
        buffer.clear();
        io::stdin().read_line(&mut buffer).expect("Failed to read user input!");
        match buffer.trim() {
            "y" | "yes" | "Y" | "YES" => {
                // If user decides to overwrite file delete file from memory
                editor.loaded_songs.remove(index);
            }
            _ => {
                // Else fail and return early
                println!("Add song aborted!");
                return Err("Add song aborted because didn't overwrite existing song with same name!");
            }
        }
    }
    buffer.clear();
    print!("Song BPM: ");
    io::stdout().flush().expect("Failed to flush stdout! Exiting!");
    io::stdin().read_line(&mut buffer).expect("Failed to read user input!");
    match buffer.trim().parse::<u16>() {
        Ok(bpm) => {
            // Create new song and add it to editor
            editor.loaded_songs.push(Song::new(song_name, bpm));
            println!("Created song!");
            Ok(())
        },
        Err(_) => {
            println!("BPM must be an integer value!");
            Err("non integer value provided for bpm")
        }
    }
}

fn delete_song_ui(editor: &mut SongEditor) -> Result<(), &'static str> {
    println!("Which song would you like to delete?");
    match select_song_ui(editor) {
        Ok((index, _)) => {
            editor.loaded_songs.remove(index);
            Ok(())
        },
        Err(err) => Err(err)
    }
}

fn load_song_ui(editor: &mut SongEditor) -> Result<(), &'static str> {
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

fn edit_song_ui(editor: &mut SongEditor) -> Result<(), &'static str> {
    // User selects song to edit
    println!("Which song would you like to edit?");
    match select_song_ui(editor) {
        Ok((_, song)) => {
            song_cli::edit_song_ui(song);
            println!("Done editing song!");
            Ok(())
        },
        Err(err) => Err(err)
    }
}

fn show_songs_ui(editor: &mut SongEditor) {
    println!("Your Songs:");
    for (index, song) in editor.loaded_songs.iter().enumerate() {
        println!("\t{}. {}", index + 1, song.name);
    }
}

fn select_song_ui<'a>(editor: &'a mut SongEditor) -> Result<(usize, &'a mut Song), &'static str> {
    print!("Select a song by name or number: ");
    io::stdout().flush().expect("Stdout failed to flush! Exiting!");
    let mut buf = String::new();
    if let Err(_err) = io::stdin().read_line(&mut buf) {
        println!("Failed to read user input");
        return Err("Failed to read user input");
    }
    let buf = buf.trim();
    if let Some((index, _)) = editor.loaded_songs.iter().enumerate().find(|(_, song)| song.name.to_lowercase() == buf.to_lowercase()) {
        if let Some(song) = editor.loaded_songs.get_mut(index) {
            return Ok((index, song))
        }
        else {
            return Err("Failed to parse index as song or a song name!")
        }
    }
    match buf.parse::<usize>() {
        Ok(index) => {
            if let Some(song) = editor.loaded_songs.get_mut(index - 1) {
                Ok((index - 1, song))
            }
            else {
                Err("Failed to parse index as song or a song name!")
            }
        },
        Err(_) => {
            println!("Not a recognised song!");
            Err("Failed to parse index as song or a song name!")
        }
    }
}