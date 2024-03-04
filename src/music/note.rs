use std::fmt::Display;

use super::serializable::Serializable;

/// Represents a certain pitch at a certain time at a certain volume. Is part of a [Part]
pub struct Note {
    // what beat it plays on
    pub beat: f32,  
    // how long it plays in beats
    pub duration: f32, 
    pub frequency: f32,
    pub volume: f32
}

impl Note {
    pub fn new(beat: f32, duration: f32, frequency: f32, volume: f32) -> Result<Self, &'static str> {
        if volume > 1.0 {
            return Err("Note must have volume in range [0, 1]");
        }
        Ok(Note {beat, duration, frequency, volume})
    }

    pub fn end_beat(&self) -> f32 {
        self.beat + self.duration
    }

    pub fn plays_at(&self, beat: f32) -> bool {
        if beat >= self.beat && beat < self.end_beat() {
            return true
        }
        false
    }

    pub fn get_sample_amplitude(&self, time: f32) -> i16 {
        ((time * 2.0 * std::f32::consts::PI * self.frequency).sin() * self.volume * crate::wav::MAX_AMPLITUDE as f32) as i16
    }
}

impl Serializable for Note {
    /// Serializes a `Note` struct into a byte representation
    /// f32: beat
    /// f32: duration
    /// f32: frequency
    /// f32: volume
    fn serialize(&self) -> Result<Vec<u8>, &'static str> {
        let mut serialized_data = Vec::new();
        // Serialize the time
        let time_bytes = self.beat.to_le_bytes();
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
        let beat_bytes = &serialized_data[0..4];
        let beat = f32::from_le_bytes(beat_bytes.try_into().unwrap());
        // Deserialize the duration
        let dur_bytes = &serialized_data[4..8];
        let duration = f32::from_le_bytes(dur_bytes.try_into().unwrap());
        // Deserialize the frequency
        let freq_bytes = &serialized_data[8..12];
        let frequency = f32::from_le_bytes(freq_bytes.try_into().unwrap());
        // Deserialize the volume
        let vol_bytes = &serialized_data[12..16];
        let volume = f32::from_le_bytes(vol_bytes.try_into().unwrap());
        Ok(Self { beat, duration, frequency, volume })
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Note(beat: {}, duration: {}, frequency: {}, volume: {})", self.beat, self.duration, self.frequency, self.volume)
    }
}