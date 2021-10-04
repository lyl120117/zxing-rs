#[derive(Debug, Clone)]
pub struct BitArray {
    bits: Vec<u32>,
    size: usize,
}

impl BitArray {
    pub fn new() -> BitArray {
        BitArray {
            bits: vec![0],
            size: 0,
        }
    }

    /**
     * Appends the least-significant bits, from value, in order from most-significant to
     * least-significant. For example, appending 6 bits from 0x000001E will append the bits
     * 0, 1, 1, 1, 1, 0 in that order.
     *
     * @param value {@code int} containing bits to append
     * @param numBits bits from value to append
     */
    pub fn append_bits(&mut self, value: u32, num_bits: usize) {
        if num_bits > 32 {
            panic!("Num bits must be between 0 and 32")
        }
        let mut next_size = self.size;
        self.ensure_capacity(next_size + num_bits);
        println!(
            "append_bits bits: {:?}, value: {}, num_bits: {}",
            self.bits, value, num_bits
        );
        for num_bits_left in (0..num_bits).rev() {
            if (value & (1 << num_bits_left)) != 0 {
                let index = num_bits_left / 32;
                self.bits[index] |= 1 << (next_size & 0x1F);
            }
            next_size += 1;
        }
        self.size = next_size;
    }

    pub fn ensure_capacity(&mut self, size: usize) {
        println!("ensure_capacity size: {}, bits: {}", size, self.bits.len());
        if size > self.bits.len() * 32 {
            let bits_size = (size + 31) / 32;
            for _ in self.bits.len()..bits_size {
                self.bits.push(0);
            }
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
}
