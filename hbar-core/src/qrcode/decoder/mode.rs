use crate::qrcode::decoder::Version;
use strum_macros::EnumString;
use strum_macros::ToString;

#[derive(Debug, PartialEq, Eq, Hash, EnumString, ToString, Clone)]
pub enum Mode {
    Terminator([i32; 3], i32),
    Numeric([i32; 3], i32),
    Alphanumeric([i32; 3], i32),
    StructuredAppend([i32; 3], i32),
    Byte([i32; 3], i32),
    ECI([i32; 3], i32),
    Kanji([i32; 3], i32),
    Fnc1FirstPosition([i32; 3], i32),
    Fnc1SecondPosition([i32; 3], i32),
    Hanzi([i32; 3], i32),
}

impl Mode {
    pub fn get_terminator() -> Mode {
        Mode::Terminator([0, 0, 0], 0x00)
    }
    pub fn get_numeric() -> Mode {
        Mode::Numeric([10, 12, 14], 0x01)
    }
    pub fn get_alphanumeric() -> Mode {
        Mode::Alphanumeric([9, 11, 13], 0x02)
    }
    pub fn get_structured_append() -> Mode {
        Mode::StructuredAppend([0, 0, 0], 0x03)
    }
    pub fn get_byte() -> Mode {
        Mode::Byte([8, 16, 16], 0x04)
    }
    pub fn get_eci() -> Mode {
        Mode::ECI([0, 0, 0], 0x07)
    }
    pub fn get_kanji() -> Mode {
        Mode::Kanji([8, 10, 12], 0x08)
    }
    pub fn get_fnc1_first_position() -> Mode {
        Mode::Fnc1FirstPosition([0, 0, 0], 0x05)
    }
    pub fn get_fnc1_second_position() -> Mode {
        Mode::Fnc1SecondPosition([0, 0, 0], 0x09)
    }
    pub fn get_hanzi() -> Mode {
        Mode::Hanzi([8, 10, 12], 0x0D)
    }

    pub fn get_bits(&self) -> i32 {
        match self {
            Mode::Terminator(_, bits) => *bits,
            Mode::Numeric(_, bits) => *bits,
            Mode::Alphanumeric(_, bits) => *bits,
            Mode::StructuredAppend(_, bits) => *bits,
            Mode::Byte(_, bits) => *bits,
            Mode::ECI(_, bits) => *bits,
            Mode::Kanji(_, bits) => *bits,
            Mode::Fnc1FirstPosition(_, bits) => *bits,
            Mode::Fnc1SecondPosition(_, bits) => *bits,
            Mode::Hanzi(_, bits) => *bits,
        }
    }

    fn character_count_bits_for_versions(&self) -> [i32; 3] {
        match self {
            Mode::Terminator(character_count_bits_for_versions, _) => {
                *character_count_bits_for_versions
            }
            Mode::Numeric(character_count_bits_for_versions, _) => {
                *character_count_bits_for_versions
            }
            Mode::Alphanumeric(character_count_bits_for_versions, _) => {
                *character_count_bits_for_versions
            }
            Mode::StructuredAppend(character_count_bits_for_versions, _) => {
                *character_count_bits_for_versions
            }
            Mode::Byte(character_count_bits_for_versions, _) => *character_count_bits_for_versions,
            Mode::ECI(character_count_bits_for_versions, _) => *character_count_bits_for_versions,
            Mode::Kanji(character_count_bits_for_versions, _) => *character_count_bits_for_versions,
            Mode::Fnc1FirstPosition(character_count_bits_for_versions, _) => {
                *character_count_bits_for_versions
            }
            Mode::Fnc1SecondPosition(character_count_bits_for_versions, _) => {
                *character_count_bits_for_versions
            }
            Mode::Hanzi(character_count_bits_for_versions, _) => *character_count_bits_for_versions,
        }
    }

    pub fn get_character_count_bits(&self, version: &Version) -> i32 {
        let number = version.get_version_number();
        let offset;
        if number <= 9 {
            offset = 0;
        } else if number <= 26 {
            offset = 1;
        } else {
            offset = 2;
        }
        self.character_count_bits_for_versions()[offset]
    }
}
