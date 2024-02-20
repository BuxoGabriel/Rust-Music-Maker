use std::{f32::consts::PI, fs::File, io::{self, Write}};

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
    fn new(time: f32, duration: f32, frequency: f32, volume: f32) -> Result<Self, String> {
        if volume > 1.0 {
            return Err("Note must have volume in range [0, 1]".to_string());
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
    notes: Vec<Note>
}

impl Part {
    fn new() -> Self {
        Part { notes: Vec::new()}
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

    fn add_note(&mut self, note: Note) -> Result<(), String>{
        for note_i in &self.notes {
            if note_i.plays_at(note.time) || note_i.plays_at(note.end_time()) {
                return Err("can't add note inside another notes play time".to_string());
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
        Part { notes: vec![
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
        Song { name, parts: vec![
            Part::default(),
            Part { notes: vec![
                Note { frequency: 293.99, volume: 0.25, time: 0.0, duration: 1.0 }, 
                Note { frequency: 293.99, volume: 0.25, time: 1.0, duration: 0.5 },
                Note { frequency: 150.00, volume: 0.25, time: 1.5, duration: 1.5 }] 
            }]
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

    fn write_to_file(&self, file_name: &str, options: &WavOptions) {
        let mut file_name = String::from(file_name);
        file_name.push_str(".wav");
        println!("Writing to file {file_name}!");
        let mut file = File::create(file_name.as_str()).expect("Failed to create file");

        let data_size: u32 = (self.duration() * (options.bits_per_sample as u32 * options.sample_rate * options.num_channels as u32) as f32 / 8.0) as u32;

        let header = WavHeader::new(data_size, &options);
        let _ = file.write_all(header.as_bytes());

        let _ = file.write_all(&self.compile_parts_into_bytes(options));
    }
}

struct SongEditor {
    loaded_songs: Vec<Song>
}

impl SongEditor {
    fn new() -> Self {
        SongEditor { loaded_songs: Vec::new() }
    }

    fn ui(&mut self) {
        println!("Hello! Welcome to Song Maker!");
        'ui: loop {
            self.show_songs_ui();
            println!("Select one of the options listed below:");
            println!("\t1. Load Song");
            println!("\t2. Create Song");
            println!("\t3. Edit Song");
            println!("\t4. Export Song to wav file");
            println!("\t5. Exit Song Maker");
            let mut option = String::new();
            loop {
                io::stdin().read_line(&mut option).expect("Could not read user input");
                match option.trim() {
                    "1" => {
                        self.load_ui();
                        break;
                    }
                    "2" => {
                        self.create_ui();
                        break;
                    }
                    "3" => {
                        self.edit_ui();
                        break;
                    }
                    "4" => {
                        self.compile_ui();
                        break;
                    }
                    "5" | "q" | "Q" => {
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
        // TODO
    }

    fn save(&self) {
        // TODO
    }

    fn compile_ui(&self) {
        loop {
            println!("Which song would you like to compile?");
            let mut compile_index = String::new();
            io::stdin().read_line(&mut compile_index).expect("Failed to read user input!");
            let compile_index = compile_index.trim();
            if let Ok(index) = compile_index.parse::<usize>() {
                if let Some(song) = self.loaded_songs.get(index) {
                    println!("compiling song...");
                    song.write_to_file(song.name.as_str(), &WavOptions::default());
                    println!("compilation_complete!");
                    break
                }
            }
            println!("{compile_index} is not a valid song index!")
        } 
    }
}

fn main() {
    // // get file options
    // let wav_options = wavoptions::default();

    // // load / create song
    // let song = song::new();

    // // create wav file
    // song.write_to_file("output", &wav_options);

    // // signal completion
    // print!("created wav file")
    let mut song_editor = SongEditor::new();
    song_editor.ui()
}
