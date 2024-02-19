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

// represents that there are both notes and rests
type Frequency = f32;
type Volume = f32;
enum Sound {
    Frequency((Frequency, Volume)),
    Rest
}

struct Note {
    sound: Sound,
    duration: f32
}

impl Note {
    fn new(sound: Sound, duration: f32) -> Result<Self, String> {
        if let Sound::Frequency((freq, vol)) = sound {
            if vol > 1.0 {
                return Err("Note must have volume in range [0, 1]".to_string());
            }
        }
        Ok(Note {sound, duration})
    }

    fn time_in_samples(&self, options: &WavOptions) -> u32 {
        (self.duration * options.sample_rate as f32) as u32
    }

    fn as_samples(&self, time_in_samples: u32, options: &WavOptions) -> Vec<i16>{
        // ensure not dividing by 0
        assert_ne!(options.sample_rate, 0, "Sample Rate can not be 0!");

        let mut samples = Vec::new();

        let num_samples = self.time_in_samples(options);
        for i in 0..num_samples {
            let sample: i16 = match self.sound {
                Sound::Frequency((freq, vol)) => {
                    let time = (time_in_samples + i) as f32 / options.sample_rate as f32 * freq * PI * 2.0;
                    (time.sin() * MAX_AMPLITUDE as f32 * vol) as i16
                }
                Sound::Rest => 0
            };

            samples.push(sample);
        }

        samples
    }

    fn write(&self, mut file: &File, time_in_samples: u32, options: &WavOptions) {
        let samples = self.as_samples(time_in_samples, options);
        for sample in &samples {
            file.write_all(&sample.to_le_bytes());
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
    notes: Vec<Note>
}

impl Song {
    fn new() -> Self {
        let notes = vec![Note {sound: Sound::Frequency((440.0, 0.25) /* A */), duration: 1.0 }, Note { sound: Sound::Rest, duration: 1.0 }, Note {sound: Sound::Frequency((440.0, 0.5) /* A */), duration: 1.0 }];
        Song { notes }
    }

    fn add_note(&mut self, note: Note) {
        self.notes.push(note);
    }

    fn duration(&self) -> f32 {
        let mut total_duration: f32 = 0.0;
        for note in &self.notes {
            total_duration += note.duration;
        }
        total_duration
    }

    fn write_to_file(&self, file_path: &str, options: &WavOptions) {
        let mut file = File::create(file_path).expect("Failed to create file");

        let data_size: u32 = (self.duration() * (options.bits_per_sample as u32 * options.sample_rate * options.num_channels as u32) as f32 / 8.0) as u32;

        let header = WavHeader::new(data_size, &options);
        file.write_all(header.as_bytes());

        let mut time_in_samples: u32 = 0;
        for note in &self.notes {
            note.write(&file, time_in_samples, &options);
            time_in_samples += note.time_in_samples(options);
        }
    }
}

fn main() {
    // get file options
    let wav_options = WavOptions::default();

    // load / create song
    let mut song = Song::new();

    song.add_note(Note { sound: Sound::Frequency((293.99, 0.5)), duration: 1.0 });

    // create wav file
    song.write_to_file("output.wav", &wav_options);

    // Signal Completion
    print!("created wav file")
}
