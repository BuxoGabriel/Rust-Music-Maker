use std::fs::File;
use std::io::Write;
use crate::wav::{WavOptions, WavHeader};
use super::part::Part;
use super::note::Note;
use super::serializable::Serializable;

///
pub struct Song {
    pub name: String,
    pub bpm: u16,
    pub parts: Vec<Part>
}

impl Song {
    pub fn new(name: String, bpm: u16) -> Self {
        Self {
            name,
            bpm, 
            parts: Vec::new()
        }
    }

    pub fn duration(&self)-> f32 {
        let mut longest_part = 0.0;
        for part in &self.parts {
            let part_duration = part.duration();
            if part_duration > longest_part {
                longest_part = part_duration
            }
        }
        longest_part
    }

    pub fn compile_parts_into_samples(&self, options: &WavOptions) -> Vec<i16> {
        let num_samples: usize = (beat_in_seconds(self.duration(), self.bpm as f32) * options.sample_rate as f32) as usize;
        let mut samples = Vec::with_capacity(num_samples);
        for i in 0..num_samples {
            let time = i as f32 / options.sample_rate as f32;
            let mut sample_amplitude: i16 = 0;
            for part in &self.parts {
                match part.has_note_at_beat(second_in_beats(time, self.bpm as f32)) {
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

    pub fn compile_parts_into_bytes(&self, options: &WavOptions) -> Vec<u8> {
        let samples = self.compile_parts_into_samples(options);
        let mut bytes = Vec::with_capacity(samples.capacity() * 2);

        for sample in &samples {
            bytes.extend(&sample.to_le_bytes())
        }
        bytes
    }

    pub fn write_to_wav_file(&self, file_name: String, options: &WavOptions) {
        let mut file_name = String::from(file_name);
        file_name.push_str(".wav");
        println!("Writing to file {file_name}!");
        let mut file = File::create(file_name.as_str()).expect("Failed to create file");

        let data_size: u32 = (self.duration() * (options.bits_per_sample as u32 * options.sample_rate * options.num_channels as u32) as f32 / 8.0) as u32;

        let header = WavHeader::new(data_size, &options);
        let _ = file.write_all(header.as_bytes());

        let _ = file.write_all(&self.compile_parts_into_bytes(options));
    }

    pub fn write_to_song_file(&self, mut file_name: String) {
        file_name.push_str(".song");
        println!("Writing to file {file_name}!");
        let mut file = File::create(file_name.as_str()).expect("Failed to create file");
        match &self.serialize() {
            Ok(serialized_data) => {
                if let Err(_e) = file.write_all(&serialized_data) {
                    println!("Failed to write serialized data!");
                    return;
                }
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }
}

impl Default for Song {
    fn default() -> Self {
        let mut base = Part::new("base".to_string());
        let _ = base.add_note(Note { frequency: 293.99, volume: 0.25, beat: 0.0, duration: 1.0 });
        _ = base.add_note(Note { frequency: 293.99, volume: 0.25, beat: 1.0, duration: 0.5 });
        _ =base.add_note(Note { frequency: 150.00, volume: 0.25, beat: 1.5, duration: 1.5 });

        Song { name: "Demo Song".to_string(), bpm: 120, parts: vec![Part::default(), base]
        }
    }
}

impl Serializable for Song {
    /// Serializes a `Song` struct into a byte representation
    /// u16: name_len
    /// name_len: name
    /// u16: bpm
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
        // Serialize bpm
        serialized_data.extend(self.bpm.to_le_bytes());
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
        // Deserialize bpm
        if (&serialized_data[(2+name_len)..] as &[u8]).len() < 2 {
            return Err("Invalid serialized data! Insufficent length for bpm!");
        }
        let bpm_bytes = &serialized_data[(2+name_len)..(4+name_len)];
        let bpm = u16::from_le_bytes(bpm_bytes.try_into().unwrap());
        // Deserialize number of parts
        if (&serialized_data[(4+name_len)..] as &[u8]).len() < 2 {
            return Err("Invalid serialized data! Insufficent length for number of parts!");
        }
        let num_parts_bytes = &serialized_data[(4+name_len)..(6+name_len)];
        let num_parts = u16::from_le_bytes(num_parts_bytes.try_into().unwrap());
        // Deserialize parts        
        let mut remaining_data = &serialized_data[(6+name_len)..];
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
        Ok(Self { name, bpm, parts })
    }
}

fn beat_in_seconds(beat: f32, bpm: f32) -> f32 {
    beat / bpm * 60.0
}

fn second_in_beats(seconds: f32, bpm: f32) -> f32 {
    seconds / 60.0 * bpm
}