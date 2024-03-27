use std::io::{self, Write};

use crate::music::Note;
use super::choice_ui::{Choice, self};

pub fn edit_note_ui(note: &mut Note) {
    let choices = vec![
        Choice::new("Change Starting Beat".to_string(), Box::from(change_note_start_ui)),
        Choice::new("Change Note Duration".to_string(), Box::from(change_note_duration_ui)),
        Choice::new("Change Note Frequency".to_string(), Box::from(change_note_pitch_ui)),
        Choice::new("Change Note Volume".to_string(), Box::from(change_note_volume_ui))
    ];
    loop {
        println!("Note editor\nNote: {note}");
        if let Some(_) = choice_ui::ui_offer_choices(&choices, note) {}
        else {
            break
        }
    }
    println!("You have left part editor!");
}

pub fn change_note_start_ui(note: &mut Note) -> Result<(), &'static str> {
    print!("New Note Starting Beat: ");
    if let Err(_) = io::stdout().flush() {
        return Err("Failed to flush stdout! Exiting!")
    }
    let mut buf = String::new();
    if let Err(_) = io::stdin().read_line(&mut buf) {
        return Err("Failed to read user input!")
    }
    match buf.trim().parse::<f32>() {
        Ok(beat) => {
            let old_beat = note.beat;
            note.beat = beat;
            println!("Changed starting beat from {old_beat} to {}!", note.beat);
            Ok(())
        }
        Err(_) => {
            Err("Failed to parse user input as starting beat!")
        }
    }
}

pub fn change_note_duration_ui(note: &mut Note) -> Result<(), &'static str> {
    print!("New Note duration in beats: ");
    if let Err(_) = io::stdout().flush() {
        return Err("Failed to flush stdout! Exiting!");
    }
    let mut buf = String::new();
    if let Err(_) = io::stdin().read_line(&mut buf) {
        return Err("Failed to read user input!")
    }
    match buf.trim().parse::<f32>() {
        Ok(duration) => {
            let old_dur = note.duration;
            note.duration = duration;
            println!("Changed note duration from {old_dur} to {}!", note.duration);
            Ok(())
        }
        Err(_) => {
            return Err("Failed to parse user input as duration!");
        }
    }
}

pub fn change_note_pitch_ui(note: &mut Note) -> Result<(), &'static str>{
    print!("New Note Frequency: ");
    if let Err(_) = io::stdout().flush() {
        return Err("Failed to flush stdout! Exiting!");
    }
    let mut buf = String::new();
    if let Err(_) = io::stdin().read_line(&mut buf) {
        return Err("Failed to read user input!")
    }
    match buf.trim().parse::<f32>() {
        Ok(freq) => {
            let old_freq = note.frequency;
            note.frequency = freq;
            println!("Changed note frequency from {old_freq} to {}!", note.frequency);
            Ok(())
        }
        Err(_) => {
            return Err("Failed to parse user input as frequency!");
        }
    }
}

pub fn change_note_volume_ui(note: &mut Note) -> Result<(), &'static str>{
    print!("New Note Volume: ");
    if let Err(_) = io::stdout().flush() {
        return Err("Failed to flush stdout! Exiting!");
    }
    let mut buf = String::new();
    if let Err(_) = io::stdin().read_line(&mut buf) {
        return Err("Failed to read user input!")
    }
    match buf.trim().parse::<f32>() {
        Ok(vol) => {
            let old_vol = note.volume;
            note.volume = vol;
            println!("Changed note volume from {old_vol} to {}!", note.volume);
            Ok(())
        }
        Err(_) => {
            return Err("Failed to parse user input as volume!");
        }
    }
}