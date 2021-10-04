use crate::qrcode::decoder::Version;
use strum_macros::EnumString;
use strum_macros::ToString;

#[derive(Debug, PartialEq, Eq, Hash, EnumString, ToString, Copy, Clone)]
pub enum Mode {
    Terminator([u32; 3], usize),
    Numeric([u32; 3], usize),
    Alphanumeric([u32; 3], usize),
    StructuredAppend([u32; 3], usize),
    Byte([u32; 3], usize),
    ECI([u32; 3], usize),
    Kanji([u32; 3], usize),
    Fnc1FirstPosition([u32; 3], usize),
    Fnc1SecondPosition([u32; 3], usize),
    Hanzi([u32; 3], usize),
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

    pub fn get_bits(&self) -> usize {
        match self {
            Mode::Terminator(character_count_bits_for_versions, bits) => *bits,
            Mode::Numeric(character_count_bits_for_versions, bits) => *bits,
            Mode::Alphanumeric(character_count_bits_for_versions, bits) => *bits,
            Mode::StructuredAppend(character_count_bits_for_versions, bits) => *bits,
            Mode::Byte(character_count_bits_for_versions, bits) => *bits,
            Mode::ECI(character_count_bits_for_versions, bits) => *bits,
            Mode::Kanji(character_count_bits_for_versions, bits) => *bits,
            Mode::Fnc1FirstPosition(character_count_bits_for_versions, bits) => *bits,
            Mode::Fnc1SecondPosition(character_count_bits_for_versions, bits) => *bits,
            Mode::Hanzi(character_count_bits_for_versions, bits) => *bits,
        }
    }
}
