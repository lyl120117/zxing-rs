use crate::qrcode::decoder::ErrorCorrectionLevel;
use crate::WriterException;
/**
 * <p>Encapsulates the parameters for one error-correction block in one symbol version.
 * This includes the number of data codewords, and the number of times a block with these
 * parameters is used consecutively in the QR code version's format.</p>
 */
#[derive(Debug, Clone)]
pub struct ECB {
    count: i32,
    data_codewords: i32,
}

impl ECB {
    pub fn new(count: i32, data_codewords: i32) -> ECB {
        ECB {
            count: count,
            data_codewords: data_codewords,
        }
    }
    pub fn get_count(&self) -> i32 {
        self.count
    }
    pub fn get_data_codewords(&self) -> i32 {
        self.data_codewords
    }
}

/**
* <p>Encapsulates a set of error-correction blocks in one symbol version. Most versions will
* use blocks of differing sizes within one version, so, this encapsulates the parameters for
* each set of blocks. It also holds the number of error-correction codewords per block since it
* will be the same across all blocks within one version.</p>
*/

#[derive(Debug, Clone)]
pub struct ECBlocks {
    ec_codewords_per_block: i32,
    ec_blocks: Vec<ECB>,
}

impl ECBlocks {
    pub fn new(ec_codewords_per_block: i32, ec_blocks: Vec<ECB>) -> ECBlocks {
        ECBlocks {
            ec_codewords_per_block: ec_codewords_per_block,
            ec_blocks: ec_blocks,
        }
    }

    pub fn get_ec_codewords_per_block(&self) -> i32 {
        self.ec_codewords_per_block
    }

    pub fn get_num_blocks(&self) -> i32 {
        let mut total = 0;
        for ecb_block in self.ec_blocks.iter() {
            total += ecb_block.get_count();
        }
        return total;
    }

    pub fn get_total_ec_codewords(&self) -> i32 {
        self.ec_codewords_per_block * self.get_num_blocks()
    }

    pub fn get_ec_blocks(&self) -> &Vec<ECB> {
        &self.ec_blocks
    }
}

#[derive(Debug, Clone)]
pub struct Version {
    version_decode_info: Vec<i32>,
    version_number: i32,
    alignment_pattern_centers: Vec<i32>,
    ec_blocks: Vec<ECBlocks>,
    total_codewords: i32,
}

fn build_versions() -> Vec<Version> {
    vec![
        Version::new(
            1,
            vec![],
            vec![
                ECBlocks::new(7, vec![ECB::new(1, 19)]),
                ECBlocks::new(10, vec![ECB::new(1, 16)]),
                ECBlocks::new(13, vec![ECB::new(1, 13)]),
                ECBlocks::new(17, vec![ECB::new(1, 9)]),
            ],
        ),
        Version::new(
            2,
            vec![6, 18],
            vec![
                ECBlocks::new(10, vec![ECB::new(1, 34)]),
                ECBlocks::new(16, vec![ECB::new(1, 28)]),
                ECBlocks::new(22, vec![ECB::new(1, 22)]),
                ECBlocks::new(28, vec![ECB::new(1, 16)]),
            ],
        ),
        Version::new(
            3,
            vec![6, 22],
            vec![
                ECBlocks::new(15, vec![ECB::new(1, 55)]),
                ECBlocks::new(26, vec![ECB::new(1, 44)]),
                ECBlocks::new(18, vec![ECB::new(2, 17)]),
                ECBlocks::new(22, vec![ECB::new(2, 13)]),
            ],
        ),
        Version::new(
            4,
            vec![6, 26],
            vec![
                ECBlocks::new(20, vec![ECB::new(1, 80)]),
                ECBlocks::new(18, vec![ECB::new(2, 32)]),
                ECBlocks::new(26, vec![ECB::new(2, 24)]),
                ECBlocks::new(16, vec![ECB::new(4, 9)]),
            ],
        ),
        Version::new(
            5,
            vec![6, 30],
            vec![
                ECBlocks::new(26, vec![ECB::new(1, 108)]),
                ECBlocks::new(24, vec![ECB::new(2, 43)]),
                ECBlocks::new(18, vec![ECB::new(2, 15), ECB::new(2, 16)]),
                ECBlocks::new(22, vec![ECB::new(2, 11), ECB::new(2, 12)]),
            ],
        ),
        Version::new(
            6,
            vec![6, 34],
            vec![
                ECBlocks::new(18, vec![ECB::new(2, 68)]),
                ECBlocks::new(16, vec![ECB::new(4, 27)]),
                ECBlocks::new(24, vec![ECB::new(4, 19)]),
                ECBlocks::new(28, vec![ECB::new(4, 15)]),
            ],
        ),
        Version::new(
            7,
            vec![6, 22, 38],
            vec![
                ECBlocks::new(20, vec![ECB::new(2, 78)]),
                ECBlocks::new(18, vec![ECB::new(4, 31)]),
                ECBlocks::new(18, vec![ECB::new(2, 14), ECB::new(4, 15)]),
                ECBlocks::new(26, vec![ECB::new(4, 13), ECB::new(1, 14)]),
            ],
        ),
        Version::new(
            8,
            vec![6, 24, 42],
            vec![
                ECBlocks::new(24, vec![ECB::new(2, 97)]),
                ECBlocks::new(22, vec![ECB::new(2, 38), ECB::new(2, 39)]),
                ECBlocks::new(22, vec![ECB::new(4, 18), ECB::new(2, 19)]),
                ECBlocks::new(26, vec![ECB::new(4, 14), ECB::new(2, 15)]),
            ],
        ),
        Version::new(
            9,
            vec![6, 26, 46],
            vec![
                ECBlocks::new(30, vec![ECB::new(2, 116)]),
                ECBlocks::new(22, vec![ECB::new(3, 36), ECB::new(2, 37)]),
                ECBlocks::new(20, vec![ECB::new(4, 16), ECB::new(4, 17)]),
                ECBlocks::new(24, vec![ECB::new(4, 12), ECB::new(4, 13)]),
            ],
        ),
    ]
}

pub struct Versions {
    versions: Vec<Version>,
}

impl Versions {
    pub fn new() -> Versions {
        Versions {
            versions: build_versions(),
        }
    }

    pub fn get_version_for_number(&self, version_number: i32) -> Result<&Version, WriterException> {
        if version_number < 1 || version_number > 40 {
            return Err(WriterException {
                reason: String::from("Version number only has to be between 1 and 40"),
            });
        }
        Ok(&self.versions[(version_number - 1) as usize])
    }
}

impl Version {
    pub fn new(
        version_number: i32,
        alignment_pattern_centers: Vec<i32>,
        ec_blocks: Vec<ECBlocks>,
    ) -> Version {
        let mut total = 0;
        let ec_codewords = ec_blocks[0].get_ec_codewords_per_block();
        let ecb_array = ec_blocks[0].get_ec_blocks();
        for ec_block in ecb_array {
            total += ec_block.get_count() * (ec_block.get_data_codewords() + ec_codewords);
        }

        Version {
            version_decode_info: vec![
                0x07C94, 0x085BC, 0x09A99, 0x0A4D3, 0x0BBF6, 0x0C762, 0x0D847, 0x0E60D, 0x0F928,
                0x10B78, 0x1145D, 0x12A17, 0x13532, 0x149A6, 0x15683, 0x168C9, 0x177EC, 0x18EC4,
                0x191E1, 0x1AFAB, 0x1B08E, 0x1CC1A, 0x1D33F, 0x1ED75, 0x1F250, 0x209D5, 0x216F0,
                0x228BA, 0x2379F, 0x24B0B, 0x2542E, 0x26A64, 0x27541, 0x28C69,
            ],
            version_number: version_number,
            alignment_pattern_centers: alignment_pattern_centers,
            ec_blocks: ec_blocks,
            total_codewords: total,
        }
    }

    pub fn get_version_number(&self) -> i32 {
        self.version_number
    }

    pub fn get_alignment_pattern_centers(&self) -> &Vec<i32> {
        &self.alignment_pattern_centers
    }

    pub fn get_total_codewords(&self) -> i32 {
        self.total_codewords
    }

    pub fn get_dimension_for_version(&self) -> i32 {
        self.version_number * 4 + 17
    }

    pub fn get_ec_blocks_for_level(&self, ec_level: &ErrorCorrectionLevel) -> &ECBlocks {
        &self.ec_blocks[ec_level.ordinal()]
    }
}
