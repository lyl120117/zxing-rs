use crate::qrcode::decoder::ErrorCorrectionLevel;
/**
 * <p>Encapsulates the parameters for one error-correction block in one symbol version.
 * This includes the number of data codewords, and the number of times a block with these
 * parameters is used consecutively in the QR code version's format.</p>
 */
pub struct ECB {
    count: usize,
    data_codewords: usize,
}

impl ECB {
    pub fn new(count: usize, data_codewords: usize) -> ECB {
        ECB {
            count: count,
            data_codewords: data_codewords,
        }
    }
    pub fn get_count(&self) -> usize {
        self.count
    }
    pub fn get_data_codewords(&self) -> usize {
        self.data_codewords
    }
}

/**
* <p>Encapsulates a set of error-correction blocks in one symbol version. Most versions will
* use blocks of differing sizes within one version, so, this encapsulates the parameters for
* each set of blocks. It also holds the number of error-correction codewords per block since it
* will be the same across all blocks within one version.</p>
*/
pub struct ECBlocks {
    ec_codewords_per_block: usize,
    ec_blocks: Vec<ECB>,
}

impl ECBlocks {
    pub fn new(ec_codewords_per_block: usize, ec_blocks: Vec<ECB>) -> ECBlocks {
        ECBlocks {
            ec_codewords_per_block: ec_codewords_per_block,
            ec_blocks: ec_blocks,
        }
    }

    pub fn get_ec_codewords_per_block(&self) -> usize {
        self.ec_codewords_per_block
    }

    pub fn get_num_blocks(&self) -> usize {
        let mut total = 0;
        for ecb_block in self.ec_blocks.iter() {
            total += ecb_block.get_count();
        }
        return total;
    }

    pub fn get_total_ec_codewords(&self) -> usize {
        self.ec_codewords_per_block * self.get_num_blocks()
    }

    pub fn get_ec_blocks(&self) -> &Vec<ECB> {
        &self.ec_blocks
    }
}

pub struct Version {
    version_decode_info: Vec<u32>,
    versions: Vec<Version>,
    version_number: usize,
    alignment_pattern_centers: Vec<u32>,
    ec_blocks: Vec<ECBlocks>,
    total_codewords: usize,
}

fn build_versions() -> Vec<Version> {
    todo!("buildVersions");
}

impl Version {
    pub fn new(
        version_number: usize,
        alignment_pattern_centers: Vec<u32>,
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
            versions: build_versions(),
            version_number: version_number,
            alignment_pattern_centers: alignment_pattern_centers,
            ec_blocks: ec_blocks,
            total_codewords: total,
        }
    }

    pub fn get_version_number(&self) -> usize {
        self.version_number
    }

    pub fn get_alignment_pattern_centers(&self) -> &Vec<u32> {
        &self.alignment_pattern_centers
    }

    pub fn get_total_codewords(&self) -> usize {
        self.total_codewords
    }

    pub fn get_dimension_for_version(&self) -> usize {
        self.version_number * 4 + 17
    }

    pub fn get_ec_blocks_for_level(&self, ec_level: ErrorCorrectionLevel) -> &ECBlocks {
        &self.ec_blocks[ec_level.ordinal()]
    }
}
