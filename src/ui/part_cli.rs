use std::io::{self, Write};

use super::choice_ui::{self, Choice};
use crate::{music::{Note, Part}, ui::note_cli};

pub fn edit_part_ui(part: &mut Part) {
    let choices = vec![
        Choice::new("Add Note".to_string(), Box::from(add_note_ui)),
        Choice::new("Delete Note".to_string(), Box::from(delete_note_ui)),
        Choice::new("Change Name".to_string(), Box::from(change_name_ui)),
        // Todo Change Volume
        Choice::new("Edit Note".to_string(), Box::from(edit_note_ui))
    ];
    loop {
        println!("Part editor: Editing {}", part.name);
        show_notes_ui(part);
        if let Some(_) = choice_ui::ui_offer_choices(&choices, part) {}
        else {
            break
        }
    }
    println!("You have left part editor!");
}

fn show_notes_ui(part: &Part) {
    println!("Part Name: {}", part.name);
    println!("Part Notes:");
    for (index, note) in part.notes.iter().enumerate() {
        println!("{}. {}", index + 1, note);
    }
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

fn change_name_ui(part: &mut Part) {
    // Get new Part name from user
    print!("New part name: ");
    io::stdout().flush().expect("Failed to flush stdout! Exiting!");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read user input!");
    let old_name = part.name.clone();
    part.name = buf.trim().to_string();
    println!("Changed name from {old_name} to {}!", part.name);
}

fn edit_note_ui(part: &mut Part) {
    //  user is presented with options to edit note
    println!("Which note would you like to edit?");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read user input!");
    let edit_index = buf.trim();
    let note_index = edit_index.parse::<usize>().expect("Could not parse user input as number!") - 1;
    if let Some(note) = part.notes.get_mut(note_index) {
        // If user selects valid note show note edit ui
        note_cli::edit_note_ui(note);
        println!("Done editing Note!");
        return;
    }
    else {
        println!("{note_index} is not a valid note index!");
    }
}