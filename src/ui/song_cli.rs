use std::io;

use crate::{music::Song, wav::WavOptions};

use super::part_cli::edit_part_ui;

pub fn edit_song_ui(song: &mut Song) {
    //  user is presented with options to edit part
    println!("Song name: {}", &song.name);
    println!("Song parts:");
    let parts = &song.parts;
    for (index, part) in parts.iter().enumerate() {
        println!("\t{}. {}", index + 1, &part.name)
    }
    println!("Which part would you like to edit?");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read user input!");
    let edit_index = buf.trim();
    let part_index = edit_index.parse::<usize>().expect("Could not parse user input as number!") - 1;
    if let Some(part) = song.parts.get_mut(part_index) {
        // If user selects valid part show part edit ui
        edit_part_ui(part);
        println!("Done editing Song!");
        return;
    }
    else {
        println!("{part_index} is not a valid song index!");
    }
}

fn save_song_ui(song: &mut Song) {
    println!("saving song...");
    song.write_to_song_file(song.name.clone());
    println!("saving complete!");
}

fn compile_song_ui(song: &mut Song) {
    println!("Compiling song...");
    // TODO get wavoptions from user optionally
    song.write_to_wav_file(song.name.clone(), &WavOptions::default());
    println!("Compilation complete!");
}