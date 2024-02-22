use std::{f32::consts::PI, fs::File, io::{self, BufReader, Read, Write}};
use rfd::FileDialog;

const BITS_PER_SAMPLE: u16 = 16;
const MAX_AMPLITUDE: i16 = i16::MAX;

// WavHeader Struct contains header information for wav file
// Using repr(C, packed) macro in order to give the struct the packed C represesentation
// and prevent padding being added for efficiency
#[repr(C, packed)]
struct WavHeader {
    chunk_id: [u8; 4],
    chunk_size: u32,
    format: [u8; 4],
    subchunk1_id: [u8; 4],
    subchunk1_size: u32,
    audio_format: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
    subchunk2_id: [u8; 4],
    subchunk2_size: u32
}

impl WavHeader {
    // creates a new WavHeader with standard information predetermined with the rest coming from an options struct
    fn new(data_size: u32, options: &WavOptions) -> Self {
        WavHeader {
            chunk_id: *b"RIFF",
            chunk_size: data_size + 36,
            format: *b"WAVE",
            subchunk1_id: *b"fmt ",
            subchunk1_size: 16,
            audio_format: 1,
            num_channels: options.num_channels,
            sample_rate: options.sample_rate,
            byte_rate: options.sample_rate * options.num_channels as u32 * BITS_PER_SAMPLE as u32 / 8,
            block_align: options.num_channels * BITS_PER_SAMPLE / 8,
            bits_per_sample: BITS_PER_SAMPLE,
            subchunk2_id: *b"data",
            subchunk2_size: data_size
        }
    }

    // turns the header into bytes so it may be written to a file
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const u8, 
                std::mem::size_of::<Self>()
            )
        }
    }
}

/// Represents a certain pitch at a certain time at a certain volume. Is part of a [Part]
struct Note {
    time: f32,
    duration: f32,
    frequency: f32,
    volume: f32
}

impl Note {
    fn new(time: f32, duration: f32, frequency: f32, volume: f32) -> Result<Self, &'static str> {
        if volume > 1.0 {
            return Err("Note must have volume in range [0, 1]");
        }
        Ok(Note {time, duration, frequency, volume})
    }

    fn end_time(&self) -> f32 {
        self.time + self.duration
    }

    fn plays_at(&self, time: f32) -> bool {
        if time >= self.time && time < self.end_time() {
            return true;
        }
        false
    }

    fn get_sample_amplitude(&self, time: f32) -> i16 {
        ((time * 2.0 * PI * self.frequency).sin() * self.volume * MAX_AMPLITUDE as f32) as i16
    }
}

/// Represents a musical instrement or part. Can only play one [Note] at a time and multiple Parts are part of a [Song]
struct Part {
    name: String,
    notes: Vec<Note>
}

impl Part {
    fn new(name: String) -> Self {
        Part { name, notes: Vec::new()}
    }

    // Checks if the part has a note at a certain time
    fn has_note(&self, time: f32) -> Option<&Note> {
        for note in &self.notes {
            if note.plays_at(time) {
                return Some(note)
            }
        }
        None
    }

    fn add_note(&mut self, note: Note) -> Result<(), &'static str>{
        for note_i in &self.notes {
            if note_i.plays_at(note.time) || note_i.plays_at(note.end_time()) {
                return Err("can't add note inside another notes play time");
            }
        }
        self.notes.push(note);
        Ok(())
    }

    fn duration(&self) -> f32 {
        let mut final_note_end: f32 = 0.0;
        for note in &self.notes {
            let note_end = note.time + note.duration;
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
            Note { frequency: 440.0 /* A */, volume: 0.25, time: 0.0, duration: 1.0 },
            Note {frequency: 440.0 /* A */, volume: 0.5, time: 2.0, duration: 1.0 },
            Note {frequency: 293.99, volume: 0.5, time: 3.0, duration: 1.0 }]
        }
    }
}

struct WavOptions {
    sample_rate: u32,
    num_channels: u16,
    bits_per_sample: u16
}

impl Default for WavOptions {
    fn default() -> Self {
        WavOptions {
            sample_rate: 44100,
            num_channels: 1,
            bits_per_sample: 16
        }
    }
}


struct Song {
    name: String,
    parts: Vec<Part>
}

impl Song {
    fn new(name: String) -> Self {
        Self {
            name,
            parts: Vec::new()
        }
    }

    fn duration(&self)-> f32 {
        let mut longest_part = 0.0;
        for part in &self.parts {
            let part_duration = part.duration();
            if part_duration > longest_part {
                longest_part = part_duration
            }
        }
        longest_part
    }

    fn compile_parts_into_samples(&self, options: &WavOptions) -> Vec<i16> {
        let num_samples: usize = (self.duration() * options.sample_rate as f32) as usize;
        let mut samples = Vec::with_capacity(num_samples);
        for i in 0..num_samples {
            let time = i as f32 / options.sample_rate as f32;
            let mut sample_amplitude: i16 = 0;
            for part in &self.parts {
                match part.has_note(i as f32 / options.sample_rate as f32) {
                    Some(note) => {
                        sample_amplitude += note.get_sample_amplitude(time)
                    },
                    None => ()
                };
            }
            samples.push(sample_amplitude);
        }
        samples
    }

    fn compile_parts_into_bytes(&self, options: &WavOptions) -> Vec<u8> {
        let samples = self.compile_parts_into_samples(options);
        let mut bytes = Vec::with_capacity(samples.capacity() * 2);

        for sample in &samples {
            bytes.extend(&sample.to_le_bytes())
        }
        bytes
    }

    fn write_to_wav_file(&self, file_name: String, options: &WavOptions) {
        let mut file_name = String::from(file_name);
        file_name.push_str(".wav");
        println!("Writing to file {file_name}!");
        let mut file = File::create(file_name.as_str()).expect("Failed to create file");

        let data_size: u32 = (self.duration() * (options.bits_per_sample as u32 * options.sample_rate * options.num_channels as u32) as f32 / 8.0) as u32;

        let header = WavHeader::new(data_size, &options);
        let _ = file.write_all(header.as_bytes());

        let _ = file.write_all(&self.compile_parts_into_bytes(options));
    }

    fn write_to_song_file(&self, mut file_name: String) {
        file_name.push_str(".song");
        println!("Writing to file {file_name}!");
        let mut file = File::create(file_name.as_str()).expect("Failed to create file");
        match &self.serialize() {
            Ok(serialized_data) => {
                file.write_all(&serialized_data);
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }
}

impl Default for Song {
    fn default() -> Self {
        Song { name: "Demo Song".to_string(), parts: vec![
            Part::default(),
            Part { name: "base".to_string(), notes: vec![
                Note { frequency: 293.99, volume: 0.25, time: 0.0, duration: 1.0 }, 
                Note { frequency: 293.99, volume: 0.25, time: 1.0, duration: 0.5 },
                Note { frequency: 150.00, volume: 0.25, time: 1.5, duration: 1.5 }] 
            }]
        }
    }
}

trait Serializable {
    fn serialize(&self) -> Result<Vec<u8>, &'static str>;
    fn deserialize(serialized_data: &[u8]) -> Result<Self, &'static str>
    where 
        Self: Sized;
}

impl Serializable for Note {
    /// Serializes a `Note` struct into a byte representation
    /// f32: time
    /// f32: duration
    /// f32: frequency
    /// f32: volume
    fn serialize(&self) -> Result<Vec<u8>, &'static str> {
        let mut serialized_data = Vec::new();
        // Serialize the time
        let time_bytes = self.time.to_le_bytes();
        serialized_data.extend(time_bytes);
        // Serialize the duration
        let dur_bytes = self.duration.to_le_bytes();
        serialized_data.extend(dur_bytes);
        // Serialize the frequency
        let freq_bytes = self.frequency.to_le_bytes();
        serialized_data.extend(freq_bytes);
        // Serialize the volume
        let vol_bytes = self.volume.to_le_bytes();
        serialized_data.extend(vol_bytes);
        Ok(serialized_data)
    }

    fn deserialize(serialized_data: &[u8]) -> Result<Self, &'static str> {
        if serialized_data.len() != 16 {
            return Err("Invalid serialized data! Insuffient data for note");
        }
        // Deserialize the time
        let time_bytes = &serialized_data[0..4];
        let time = f32::from_le_bytes(time_bytes.try_into().unwrap());
        // Deserialize the duration
        let dur_bytes = &serialized_data[4..8];
        let duration = f32::from_le_bytes(dur_bytes.try_into().unwrap());
        // Deserialize the frequency
        let freq_bytes = &serialized_data[8..12];
        let frequency = f32::from_le_bytes(freq_bytes.try_into().unwrap());
        // Deserialize the volume
        let vol_bytes = &serialized_data[12..16];
        let volume = f32::from_le_bytes(vol_bytes.try_into().unwrap());
        Ok(Self { time, duration, frequency, volume })
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

impl Serializable for Song {
    /// Serializes a `Song` struct into a byte representation
    /// u16: name_len
    /// name_len: name
    /// u16: num parts
    /// (parts) u16: size_of_part
    /// (parts) size_of_part: part
    fn serialize(&self) -> Result<Vec<u8>, &'static str> {
        let mut serialized_data: Vec<u8> = Vec::new();
        // Serialize the name
        let name_as_bytes = self.name.as_bytes();
        let name_len = name_as_bytes.len();
        if name_len > u16::MAX as usize {
            return Err("Could not serialize song. Name too long!");
        }
        let name_len = name_len as u16;
        serialized_data.extend(name_len.to_le_bytes());
        serialized_data.extend(name_as_bytes);
        // Serialize number of parts
        let num_parts: u16 = self.parts.len() as u16;
        serialized_data.extend(num_parts.to_le_bytes());
        // Serialize each part
        for part in &self.parts {
            match part.serialize() {
                Ok(serialized_part) => {
                    let size_of_part = serialized_part.len() as u16;
                    serialized_data.extend(size_of_part.to_le_bytes());
                    serialized_data.extend(serialized_part);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(serialized_data)
    }

    fn deserialize(serialized_data: &[u8]) -> Result<Self, &'static str> {
        if serialized_data.len() < 2 {
            return Err("Invalid serialized data! Insufficient length for song name size!");
        }
        // Deserialize name
        let name_len_bytes = &serialized_data[..2];
        let name_len = u16::from_le_bytes(name_len_bytes.try_into().unwrap()) as usize;
        if serialized_data.len() < 2 + name_len {
            return Err("Invalid serialized data! Insufficent length for song name!");
        }
        let name_bytes = &serialized_data[2..(2+name_len)];
        let name = String::from_utf8_lossy(name_bytes).into_owned();
        // Deserialize number of parts
        if (&serialized_data[(2+name_len)..] as &[u8]).len() < 2 {
            return Err("Invalid serialized data! Insufficent length for number of parts!");
        }
        let num_parts_bytes = &serialized_data[(2+name_len)..(4+name_len)];
        let num_parts = u16::from_le_bytes(num_parts_bytes.try_into().unwrap());
        // Deserialize parts        
        let mut remaining_data = &serialized_data[(4+name_len)..];
        let mut parts = Vec::new();
        for _ in 0..num_parts {
            if remaining_data.len() < 2 {
                return Err("Invalid serialized data! Missing part length data!");
            }
            // Deserialize part len
            let part_size_bytes: &[u8] = &remaining_data[..2];
            let part_size = u16::from_le_bytes(part_size_bytes.try_into().unwrap()) as usize;
            // Deserialize part
            if remaining_data.len() < 2 + part_size {
                return Err("Invalid serialized data! Missing part data!");
            } 
            let part_bytes = &remaining_data[2..(2+part_size)];
            parts.push(Part::deserialize(part_bytes)?);
            remaining_data = &remaining_data[(2+part_size)..];
        }
        Ok(Self { name, parts })
    }
}

struct SongEditor {
    loaded_songs: Vec<Song>
}

impl SongEditor {
    fn new() -> Self {
        SongEditor { loaded_songs: vec![Song::default()] }
    }

    fn ui(&mut self) {
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
            println!("\t{index}. {}", song.name);
        }
    }

    fn create_ui(&mut self) {
        print!("Song name: ");
        io::stdout().flush().expect("Failed to flush stdout! Exiting!");
        let mut song_name = String::new();
        io::stdin().read_line(&mut song_name).expect("Failed to read song name!");
        let song_name = song_name.trim().to_string();
        if let Some((index, _song)) = self.loaded_songs.iter().enumerate().find(|(_index, song)| song.name == song_name) {
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
                if let Some(song) = self.loaded_songs.get(index) {
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
                    song.write_to_song_file(song.name.clone());
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
                    song.write_to_wav_file(song.name.clone(), &WavOptions::default());
                    println!("Compilation complete!");
                    break
                }
            }
            println!("{compile_index} is not a valid song index!")
        } 
    }
}

fn main() {
    let mut song_editor = SongEditor::new();
    song_editor.ui()
}