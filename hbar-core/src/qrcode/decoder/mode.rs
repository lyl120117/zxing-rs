use strum_macros::EnumString;
use strum_macros::ToString;

#[derive(Debug, PartialEq, Eq, Hash, EnumString, ToString, Copy, Clone)]
pub enum ModeType {
    Terminator,
    Numeric,
    Alphanumeric,
    StructuredAppend,
    Byte,
    ECI,
    Kanji,
    Fnc1FirstPosition,
    Fnc1SecondPosition,
    Hanzi,
}
pub struct Mode {
    pub terminator: [u32; 3],
    pub numeric: [u32; 3],
    pub alphanumeric: [u32; 3],
    pub structured_append: [u32; 3],
    pub byte: [u32; 3],
    pub eci: [u32; 3],
    pub kanji: [u32; 3],
    pub fnc1_first_position: [u32; 3],
    pub fnc1_second_position: [u32; 3],
    pub hanzi: [u32; 3],
    mode: [u32; 3],
    mode_type: ModeType,
}

impl Mode {
    pub fn new() -> Mode {
        Mode {
            terminator: [0, 0, 0],
            numeric: [10, 12, 14],
            alphanumeric: [9, 11, 13],
            structured_append: [0, 0, 0],
            byte: [8, 16, 16],
            eci: [0, 0, 0],
            kanji: [8, 10, 12],
            fnc1_first_position: [0, 0, 0],
            fnc1_second_position: [0, 0, 0],
            hanzi: [8, 10, 1],
            mode: [0, 0, 0],
            mode_type: ModeType::Byte,
        }
    }

    pub fn set_bits(&mut self, mode: [u32; 3], model_type: ModeType) {
        self.mode = mode;
        self.mode_type = model_type
    }

    pub fn get_bits(&self) -> u32 {
        match self.mode_type {
            ModeType::Terminator => 0x00,
            ModeType::Numeric => 0x01,
            ModeType::Alphanumeric => 0x02,
            ModeType::StructuredAppend => 0x03,
            ModeType::Byte => 0x04,
            ModeType::ECI => 0x07,
            ModeType::Kanji => 0x08,
            ModeType::Fnc1FirstPosition => 0x05,
            ModeType::Fnc1SecondPosition => 0x09,
            ModeType::Hanzi => 0x0D,
        }
    }

    pub fn get_type(&self) -> ModeType {
        self.mode_type
    }
}
