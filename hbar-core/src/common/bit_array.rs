#[derive(Clone)]
pub struct BitArray {
    bits: Vec<i32>,
    size: i32,
}

use std::fmt;
impl fmt::Display for BitArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.size {
            if (i & 0x07) == 0 {
                write!(f, " ").unwrap();
            }
            if self.get(i) {
                write!(f, "X").unwrap();
            } else {
                write!(f, ".").unwrap();
            }
        }
        write!(f, " size: {}", self.size)
    }
}

impl BitArray {
    pub fn new() -> BitArray {
        BitArray {
            bits: vec![0],
            size: 0,
        }
    }

    pub fn new1(size: u32) -> BitArray {
        BitArray {
            bits: BitArray::make_array(size),
            size: 0,
        }
    }

    fn make_array(size: u32) -> Vec<i32> {
        let size = ((size + 31) / 32) as usize;
        vec![0; size]
    }

    /**
     * Appends the least-significant bits, from value, in order from most-significant to
     * least-significant. For example, appending 6 bits from 0x000001E will append the bits
     * 0, 1, 1, 1, 1, 0 in that order.
     *
     * @param value {@code int} containing bits to append
     * @param numBits bits from value to append
     */
    pub fn append_bits(&mut self, value: i32, num_bits: i32) {
        if num_bits > 32 {
            panic!("Num bits must be between 0 and 32")
        }
        let mut next_size = self.size;
        self.ensure_capacity(next_size + num_bits);
        for num_bits_left in (0..num_bits).rev() {
            if (value & (1 << num_bits_left)) != 0 {
                self.bits[next_size as usize / 32] |= 1 << (next_size & 0x1F);
            }
            next_size += 1;
        }
        self.size = next_size;
    }

    pub fn ensure_capacity(&mut self, size: i32) {
        if size > (self.bits.len() * 32) as i32 {
            let bits_size = (size + 31) / 32;
            for _ in self.bits.len()..bits_size as usize {
                self.bits.push(0);
            }
        }
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }

    pub fn append_bit_array(&mut self, other: &BitArray) {
        let other_size = other.get_size();
        self.ensure_capacity(self.size + other_size);
        for i in 0..other_size {
            self.append_bit(other.get(i))
        }
    }

    pub fn append_bit(&mut self, bit: bool) {
        self.ensure_capacity(self.size + 1);
        if bit {
            self.bits[self.size as usize / 32] |= 1 << (self.size & 0x1F);
        }
        self.size += 1;
    }

    /**
     * @param i bit to get
     * @return true iff bit i is set
     */
    pub fn get(&self, i: i32) -> bool {
        (self.bits[i as usize / 32] & (1 << (i & 0x1F))) != 0
    }

    /**
     * Sets bit i.
     *
     * @param i bit to set
     */
    pub fn set(&mut self, i: i32) {
        self.bits[i as usize / 32] |= 1 << (i & 0x1F);
    }

    pub fn get_size_in_bytes(&self) -> i32 {
        return (self.size + 7) / 8;
    }

    /**
     *
     * @param bit_offset first bit to start writing
     * @param array array to write into. Bytes are written most-significant byte first. This is the opposite
     *  of the internal representation, which is exposed by {@link #getBitArray()}
     * @param offset position in array to start writing
     * @param num_bytes how many bytes to write
     */
    pub fn to_bytes(&self, bit_offset: i32, array: &mut Vec<i32>, offset: i32, num_bytes: i32) {
        let mut bit_offset = bit_offset;
        for i in 0..num_bytes {
            let mut the_byte: i32 = 0;
            for j in 0..8 {
                if self.get(bit_offset) {
                    the_byte |= 1 << (7 - j);
                }
                bit_offset += 1;
            }
            array[(offset + i) as usize] = the_byte;
        }
    }

    pub fn xor(&mut self, other: &BitArray) {
        if self.size != other.get_size() {
            panic!("Sizes don't match")
        }
        for i in 0..self.bits.len() {
            // The last int could be incomplete (i.e. not have 32 bits in
            // it) but there is no problem since 0 XOR 0 == 0.
            self.bits[i] ^= other.bits[i];
        }
    }

    /**
     * Clears all bits (sets to false).
     */
    pub fn clear(&mut self) {
        self.bits = vec![0; self.bits.len()]
    }
}
