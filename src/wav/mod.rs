pub const MAX_AMPLITUDE: i16 = i16::MAX;

const BITS_PER_SAMPLE: u16 = 16;

// WavHeader Struct contains header information for wav file
// Using repr(C, packed) macro in order to give the struct the packed C represesentation
// and prevent padding being added for efficiency
#[repr(C, packed)]
pub struct WavHeader {
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
    pub fn new(data_size: u32, options: &WavOptions) -> Self {
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
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const u8, 
                std::mem::size_of::<Self>()
            )
        }
    }
}

pub struct WavOptions {
    pub sample_rate: u32,
    pub num_channels: u16,
    pub bits_per_sample: u16
}

impl WavOptions {
    pub fn new(sample_rate:u32, num_channels: u16, bits_per_sample: u16) -> Self {
        WavOptions { sample_rate, num_channels, bits_per_sample }
    }
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