use std::io;

enum Accidental {
    Sharp,
    Flat,
}

impl Accidental {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'b' => Some(Accidental::Flat),
            '#' => Some(Accidental::Sharp),
            _ => None
        }
    }
}

pub fn select_note_ui() -> Result<f32, &'static str> {
    println!("Write a note in the form <Note a-g|A-G><Accidental b|#><Octave 1<=o<=7> e.g. A4 or C#6");
    let mut buf = String::new();
    if let Err(_) = io::stdin().read_line(&mut buf) {
        return Err("Failed to read user input!");
    }
    match freq_from_str(buf.trim()) {
        Some(freq) => {
            Ok(freq)
        },
        None => {
            Err("Input not recognised as a valid note!")
        }
    }
}

fn freq_from_str(note: &str) -> Option<f32> {
    let mut char_iter = note.chars();
    let char = char_iter.next()?;
    let next_char = char_iter.next()?;
    let accidental = Accidental::from_char(next_char);
    // Get frequency from note and accidental
    let base_freq = match char {
        'a' | 'A' => {
            if let Some(accidental) = &accidental {
                match accidental {
                    Accidental::Flat => {
                        Some(25.96_f32)
                    },
                    Accidental::Sharp => {
                        Some(29.14_f32)
                    }
                }
            } else {
                Some(27.5_f32)
            }
        },
        'b' | 'B' => {
            if let Some(accidental) = &accidental {
                match accidental {
                    Accidental::Flat => {
                        Some(29.14_f32)
                    },
                    Accidental::Sharp => {
                        None
                    }
                }
            } else {
                Some(30.87_f32)
            }
        },
        'c' | 'C' => {
            if let Some(accidental) = &accidental {
                match accidental {
                    Accidental::Flat => {
                        None
                    },
                    Accidental::Sharp => {
                        Some(34.65_f32)
                    }
                }
            } else {
                Some(32.70_f32)
            }
        }
        'd' | 'D' => {
            if let Some(accidental) = &accidental {
                match accidental {
                    Accidental::Flat => {
                        Some(34.65_f32)
                    },
                    Accidental::Sharp => {
                        Some(38.89_f32)
                    }
                }
            } else {
                Some(36.71_f32)
            }
        },
        'e' | 'E' => {
            if let Some(accidental) = &accidental {
                match accidental {
                    Accidental::Flat => {
                        Some(38.87_f32)
                    },
                    Accidental::Sharp => {
                        None
                    }
                }
            } else {
                Some(41.20_f32)
            }
        },
        'f' | 'F' => {
            if let Some(accidental) = &accidental {
                match accidental {
                    Accidental::Flat => {
                        None
                    },
                    Accidental::Sharp => {
                        Some(46.25_f32)
                    }
                }
            } else {
                Some(43.65_f32)
            }
        }
        'g' | 'G' => {
            if let Some(accidental) = &accidental {
                match accidental {
                    Accidental::Flat => {
                        Some(46.25_f32)
                    },
                    Accidental::Sharp => {
                        Some(51.91_f32)
                    }
                }
            } else {
                Some(49.00_f32)
            }
        },
        _ => None
    };
    // If there is an accidental then next char is octave otherwise the char that would be the accidental is the octave char
    let octave = match &accidental {
        Some(_) => {
            char_iter.next()
        },
        None => {
            Some(next_char)
        }
    };
    // There should always be an octave otherwise return None
    let octave = octave?.to_string().parse::<u8>();
    if let Ok(octave) = octave {
        if octave >= 1 && octave <= 7 {
            Some(base_freq? * (2_u8.pow(octave.into()) as f32))
        }
        else {
            None
        }
    }
    else {
        None
    }
}