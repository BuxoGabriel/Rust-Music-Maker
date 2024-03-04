use std::fmt::Display;

use super::note::Note;
use super::serializable::Serializable;

/// Represents a musical instrument or part. Can only play one [Note] at a time and multiple Parts are part of a [Song]
pub struct Part {
    pub name: String,
    pub notes: Vec<Note>
}

impl Part {
    pub fn new(name: String) -> Self {
        Part { name, notes: Vec::new()}
    }

    // Checks if the part has a note at a certain time
    pub fn has_note_at_beat(&self, beat: f32) -> Option<&Note> {
        for note in &self.notes {
            if note.plays_at(beat) {
                return Some(note)
            }
        }
        None
    }

    pub fn add_note(&mut self, note: Note) -> Result<(), &'static str>{
        for note_i in &self.notes {
            if note_i.plays_at(note.beat) || note_i.plays_at(note.end_beat()) {
                return Err("can't add note inside another notes play time");
            }
        }
        self.notes.push(note);
        Ok(())
    }

    pub fn duration(&self) -> f32 {
        let mut final_note_end: f32 = 0.0;
        for note in &self.notes {
            let note_end = note.end_beat();
            if note_end > final_note_end {
                final_note_end = note_end;
            }
        }
        final_note_end
    }
}


impl Default for Part {
    fn default() -> Self {
        Part { 
            name: "Melody".to_string(),
            notes: vec![
            Note::new(0.0, 1.0, 440.0, 0.5).unwrap(),
            Note::new(1.0, 1.0, 440.0, 0.5).unwrap(),
            Note::new(2.0, 1.0, 293.99, 0.5).unwrap()
            ]
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}\n", self.name)?;
        write!(f, "notes:\n")?;
        for (index, note) in self.notes.iter().enumerate() {
            write!(f, "\t{note}\n")?;
        }
        write!(f, "\t]");
        write!(f, "}}")
    }
}

impl Serializable for Part {
    /// Serializes a `Part` struct into a byte representation
    /// u16: name_len
    /// name_len: name
    /// u16: num_notes
    /// (notes)
    fn serialize(&self) -> Result<Vec<u8>, &'static str> {
        let mut serialized_data = Vec::new();
        // Serialize the name
        let name_as_bytes = self.name.as_bytes();
        let name_len = name_as_bytes.len();
        if name_len > u16::MAX as usize {
            return Err("Could not serialize part. Name too long!");
        }
        let name_len = name_len as u16;
        serialized_data.extend(name_len.to_le_bytes());
        serialized_data.extend(name_as_bytes);
        // Serialize number of notes
        let num_notes = self.notes.len() as u16;
        serialized_data.extend(num_notes.to_le_bytes());
        // Serialize each note
        for note in &self.notes {
            match note.serialize() {
                Ok(serialized_note) => {
                    serialized_data.extend(serialized_note)
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(serialized_data)
    }

    fn deserialize(serialized_data: &[u8]) -> Result<Self, &'static str> {
        // Deserialize name
        if serialized_data.len() < 2 {
            return Err("Invalid serialized data! Insufficient length for part name size!");
        }
        let name_len_bytes = &serialized_data[..2];
        let name_len = u16::from_le_bytes(name_len_bytes.try_into().unwrap()) as usize;
        if serialized_data.len() < 2 + name_len {
            return Err("Invalid serialized data! Insufficient length for part name");
        }
        let name_bytes = &serialized_data[2..(2+name_len as usize)];
        let name = String::from_utf8_lossy(name_bytes).into_owned();
        // Deserialize number of notes
        if (&serialized_data[(2+name_len as usize)..] as &[u8]).len() < 2 {
            return Err("Invalid serialized data! Insufficent length for number of notes!");
        }
        let num_notes_bytes = &serialized_data[(2+name_len)..(4+name_len)];
        let num_notes = u16::from_le_bytes(num_notes_bytes.try_into().unwrap());
        // Deserialize individual notes
        let mut remaining_bytes: &[u8] = &serialized_data[(4+name_len)..];
        let mut notes = Vec::new();
        for _ in 0..num_notes {
            if remaining_bytes.len() < 2 {
                return Err("Invalid serialized data! Insufficient length for note size");
            }
            let note_bytes = &remaining_bytes[..16];
            notes.push(Note::deserialize(note_bytes)?);
            remaining_bytes = &remaining_bytes[16..];
        }
        Ok(Self { name, notes })
    }
}
