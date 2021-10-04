pub struct BlockPair {
    data_bytes: Vec<i32>,
    error_correction_bytes: Vec<i32>,
}

impl BlockPair {
    pub fn new(data_bytes: Vec<i32>, error_correction_bytes: Vec<i32>) -> BlockPair {
        BlockPair {
            data_bytes,
            error_correction_bytes,
        }
    }

    pub fn get_data_bytes(&self) -> &Vec<i32> {
        &self.data_bytes
    }

    pub fn get_error_correction_bytes(&self) -> &Vec<i32> {
        &self.error_correction_bytes
    }
}
