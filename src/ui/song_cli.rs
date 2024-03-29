use std::io::{self, Write};

use crate::{music::{Part, Song}, ui::choice_ui::{self, Choice}, wav::WavOptions};

use super::part_cli;

pub fn edit_song_ui(song: &mut Song) {
    let choices = vec![
        Choice::new("Export Song to .wav".to_string(), Box::from(compile_song_ui)),
        Choice::new("Save Song".to_string(), Box::from(save_song_ui)),
        Choice::new("Add Part".to_string(), Box::from(add_part_ui)),
        Choice::new("Delete Part".to_string(), Box::from(delete_part_ui)),
        Choice::new("Edit Part".to_string(), Box::from(edit_part_ui)),
        Choice::new("Change Name".to_string(), Box::from(change_name_ui)),
        Choice::new("Change BPM(Beats Per Minute)".to_string(), Box::from(change_bpm_ui)),
    ];
    loop {
        println!("Song editor: Editing {}", song.name);
        show_parts_ui(song);
        if let Some(result) = choice_ui::ui_offer_choices(&choices, song) {
            if let Err(err) = result {
                println!("{err}");
            }
        }
        else {
            break
        }
    }
    println!("You have left Song Editor");
}

fn show_parts_ui(song: &Song) {
    println!("Song Name: {}", song.name);
    println!("Song Parts:");
    for (index, part) in song.parts.iter().enumerate() {
        println!("\t{}. {}", index + 1, part.name);
    }
}

fn compile_song_ui(song: &mut Song) -> Result<(), &'static str> {
    println!("Compiling song...");
    // TODO get wavoptions from user optionally
    let result = song.write_to_wav_file(song.name.clone(), &WavOptions::default());
    println!("Compilation complete!");
    result
}

fn save_song_ui(song: &mut Song) -> Result<(), &'static str>{
    println!("Saving song...");
    match song.write_to_song_file(song.name.clone()) {
        Ok(()) => {
            println!("Saving complete!");
            Ok(())
        },
        Err(err) => Err(err)
    }
}

fn add_part_ui(song: &mut Song) -> Result<(), &'static str> {
    // Get part name
    print!("Part Name: ");
    if let Err(_) = io::stdout().flush() {
        return Err("Failed to flush stdout! Exiting!");
    }
    let mut part_name = String::new();
    if let Err(_) = io::stdin().read_line(&mut part_name) {
        return Err("Failed to read user input!");
    }
    let part = Part::new(part_name.trim().to_string());
    song.parts.push(part);
    println!("Added part!");
    Ok(())
}

fn delete_part_ui(song: &mut Song) -> Result<(), &'static str>{
    println!("Which part would you like to delete?");
    match select_part_ui(song) {
        Ok((index, _part)) => {
            song.parts.remove(index);
            Ok(())
        }
        Err(err) => Err(err)
    }
}

fn edit_part_ui(song: &mut Song) -> Result<(), &'static str>{
    //  user is presented with options to edit part
    println!("Which part would you like to edit?");
    match select_part_ui(song) {
        Ok((_index, part)) => {
            part_cli::edit_part_ui(part);
            println!("Done editing Song!");
            Ok(())
        },
        Err(err) => Err(err)
    }
}

fn change_name_ui(song: &mut Song) -> Result<(), &'static str> {
    // Get new Song name from user
    print!("New song name: ");
    if let Err(_) = io::stdout().flush() {
        return Err("Failed to flush stdout! Exiting!");
    }
    let mut buf = String::new();
    if let Err(_) = io::stdin().read_line(&mut buf) {
        return Err("Failed to read user input!");
    }
    let old_name = song.name.clone();
    song.name = buf.trim().to_string();
    println!("Changed name from {old_name} to {}!", song.name);
    Ok(())
}

fn change_bpm_ui(song: &mut Song) -> Result<(), &'static str> {
    // Get new BPM from user
    print!("New BPM: ");
    if let Err(_) = io::stdout().flush() {
        return Err("Failed to flush stdout! Exiting!");
    }
    let mut buf = String::new();
    if let Err(_) = io::stdin().read_line(&mut buf) {
        return Err("Failed to read user input!");
    }
    let old_bpm = song.bpm;
    match buf.trim().parse::<u16>() {
        Ok(bpm) => {
            song.bpm = bpm;
            println!("Changed bpm from {old_bpm} to {}!", song.bpm);
            Ok(())
        },
        Err(_) => {
            return Err("Failed to parse provided bpm as an integer!")
        }
    }
}

fn select_part_ui<'a>(song: &'a mut Song) -> Result<(usize, &'a mut Part), &'static str> {
    print!("Select a part by name or number: ");
    io::stdout().flush().expect("Stdout failed to flush! Exiting!");
    let mut buf = String::new();
    if let Err(_err) = io::stdin().read_line(&mut buf) {
        println!("Failed to read user input");
        return Err("Failed to read user input");
    }
    let buf = buf.trim();
    if let Some((index, _)) = song.parts.iter().enumerate().find(|(_, part)| part.name.to_lowercase() == buf.to_lowercase()) {
        if let Some(part) = song.parts.get_mut(index) {
            return Ok((index, part))
        }
        else {
            return Err("Failed to parse index as part or a part name!")
        }
    }
    match buf.parse::<usize>() {
        Ok(index) => {
            if let Some(part) = song.parts.get_mut(index - 1) {
                Ok((index - 1, part))
            }
            else {
                Err("Failed to parse index as part or a part name!")
            }
        },
        Err(_) => {
            Err("Failed to parse index as part or a part name!")
        }
    }
}