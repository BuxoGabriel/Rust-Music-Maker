use std::{f32::consts::PI, fs::File, io::Write};

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
    parts: Vec<Part>
}

impl Song {
    fn new() -> Self {
        Song { parts: vec![
            Part::default(),
            Part { notes: vec![
                Note { frequency: 293.99, volume: 0.25, time: 0.0, duration: 1.0 }, 
                Note { frequency: 293.99, volume: 0.25, time: 1.0, duration: 0.5 },
                Note { frequency: 150.00, volume: 0.25, time: 1.5, duration: 1.5 }] 
            }] }
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

    fn write_to_file(&self, file_path: &str, options: &WavOptions) {
        let mut file = File::create(file_path).expect("Failed to create file");

        let data_size: u32 = (self.duration() * (options.bits_per_sample as u32 * options.sample_rate * options.num_channels as u32) as f32 / 8.0) as u32;

        let header = WavHeader::new(data_size, &options);
        let _ = file.write_all(header.as_bytes());

        let _ = file.write_all(&self.compile_parts_into_bytes(options));
    }
}

fn main() {
    // get file options
    let wav_options = WavOptions::default();

    // load / create song
    let song = Song::new();

    // create wav file
    song.write_to_file("output.wav", &wav_options);

    // Signal Completion
    print!("created wav file")
}
