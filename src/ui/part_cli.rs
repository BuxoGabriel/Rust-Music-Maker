use std::io::{self, Write};

use super::choice_ui::{self, Choice};
use crate::music::{Note, Part};

pub fn edit_part_ui(part: &mut Part) {
    println!("Editing part: {}", part.name);
    println!("Notes:");
    for note in part.notes.iter() {
        println!("{}", note);
    }
    println!("What would you like to do?");
    let choices = vec![
        Choice::new("add note".to_string(), Box::from(add_note_ui)),
        Choice::new("delete note".to_string(), Box::from(delete_note_ui))
    ];
    choice_ui::ui_offer_choices(&choices, part);
}

fn add_note_ui(part: &mut Part) {
    let mut buf = String::new();
    // Get beat to play on from user
    print!("beat to play on: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();
    let beat = buf.trim().parse::<f32>().expect("failed to parse user input as float!");
    // Get duration of note in beats from user
    print!("duration in beats: ");
    io::stdout().flush().unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let duration = buf.trim().parse::<f32>().expect("failed to parse user input as float!");
    // Get frequency of note from user
    print!("frequency: ");
    io::stdout().flush().unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let frequency = buf.trim().parse::<f32>().expect("failed to parse user input as float!");
    // Get volume of note from user
    print!("volume: ");
    io::stdout().flush().unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let volume = buf.trim().parse::<f32>().expect("failed to parse user input as float!");
    // Create note from user input
    let note = Note::new(beat, duration, frequency, volume).expect("Failed to make note!");
    // Add note to part
    part.add_note(note).expect("Could not add note to part!");
}

fn delete_note_ui(part: &mut Part) {
    println!("{part}");
    println!("Which note would you like to delete?");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read user input!");
    let note_index = buf.trim().parse::<usize>().unwrap() - 1;
    part.notes.remove(note_index);
    println!("Successfully deleted note!")
}