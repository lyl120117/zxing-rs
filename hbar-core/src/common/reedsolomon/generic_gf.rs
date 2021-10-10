use crate::common::reedsolomon::GenericGFPoly;
use crate::Error;

pub enum GenericGFEnum {
    AztecData12,
    AztecData10,
    AztecData6,
    AztecParam,
    QrCodeField256,
    DataMatrixField256,
    AztecData8,
    MaxicodeField64,
}

impl GenericGFEnum {
    pub fn get(&self) -> GenericGF {
        match self {
            GenericGFEnum::AztecData12 => GenericGF::new(0x1069, 4096, 1), // x^12 + x^6 + x^5 + x^3 + 1
            GenericGFEnum::AztecData10 => GenericGF::new(0x409, 1024, 1),  // x^10 + x^3 + 1
            GenericGFEnum::AztecData6 => GenericGF::new(0x43, 64, 1),      // x^6 + x + 1
            GenericGFEnum::AztecParam => GenericGF::new(0x13, 16, 1),      // x^4 + x + 1
            GenericGFEnum::QrCodeField256 => GenericGF::new(0x011D, 256, 0), // x^8 + x^4 + x^3 + x^2 + 1
            GenericGFEnum::DataMatrixField256 => GenericGF::new(0x012D, 256, 1), // x^8 + x^5 + x^3 + x^2 + 1
            GenericGFEnum::AztecData8 => GenericGFEnum::DataMatrixField256.get(),
            GenericGFEnum::MaxicodeField64 => GenericGFEnum::AztecData6.get(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)] // we implement the Copy trait
pub struct GenericGF {
    exp_table: Vec<i32>,
    log_table: Vec<i32>,
    size: i32,
    primitive: i32,
    generator_base: i32,
}

/**
 * Create a representation of GF(size) using the given primitive polynomial.
 *
 * @param primitive irreducible polynomial whose coefficients are represented by
 *  the bits of an int, where the least-significant bit represents the constant
 *  coefficient
 * @param size the size of the field
 * @param b the factor b in the generator polynomial can be 0- or 1-based
 *  (g(x) = (x+a^b)(x+a^(b+1))...(x+a^(b+2t-1))).
 *  In most cases it should be 1, but for QR code it is 0.
 */
impl GenericGF {
    pub fn new(primitive: i32, size: i32, b: i32) -> GenericGF {
        let generator_base = b;
        if size < 1 {
            panic!("size: {} too small", size)
        }
        let u_size = size as usize;
        let mut exp_table: Vec<i32> = vec![0; u_size];
        let mut log_table: Vec<i32> = vec![0; u_size];
        let mut x: i32 = 1;
        for i in 0..u_size {
            exp_table[i] = x;
            x *= 2; // we're assuming the generator alpha is 2
            if x >= size {
                x ^= primitive;
                x &= size - 1;
            }
        }
        for i in 0..u_size - 1 {
            if exp_table[i] < 0 {
                panic!("exp_table[{}]: {} too small", i, exp_table[i])
            }
            log_table[exp_table[i] as usize] = i as i32;
        }
        GenericGF {
            exp_table: exp_table,
            log_table: log_table,
            size: size,
            primitive: primitive,
            generator_base: generator_base,
        }
    }

    pub fn get_zero(&self) -> Result<GenericGFPoly, Error> {
        GenericGFPoly::new(self.clone(), vec![0])
    }

    pub fn get_one(&self) -> Result<GenericGFPoly, Error> {
        GenericGFPoly::new(self.clone(), vec![1])
    }

    /**
     * @return the monomial representing coefficient * x^degree
     */
    pub fn build_monomial(&self, degree: i32, coefficient: i32) -> Result<GenericGFPoly, Error> {
        if degree < 0 {
            return Err(Error::IllegalArgumentException(format!(
                "build monomial error degree: {}",
                degree
            )));
        }

        if coefficient == 0 {
            return self.get_zero();
        }

        let mut coefficients = vec![0; degree as usize + 1];
        coefficients[0] = coefficient;

        GenericGFPoly::new(self.clone(), coefficients)
    }

    /**
     * Implements both addition and subtraction -- they are the same in GF(size).
     *
     * @return sum/difference of a and b
     */
    pub fn add_or_subtract(a: i32, b: i32) -> i32 {
        a ^ b
    }

    /**
     * @return 2 to the power of a in GF(size)
     */
    pub fn exp(&self, a: i32) -> Result<i32, Error> {
        if a < 0 {
            return Err(Error::IllegalArgumentException(format!(
                "exp error a: {}",
                a
            )));
        }
        Ok(self.exp_table[a as usize])
    }

    /**
     * @return base 2 log of a in GF(size)
     */
    pub fn log(&self, a: i32) -> Result<i32, Error> {
        if a <= 0 {
            return Err(Error::IllegalArgumentException(format!(
                "log error a: {}",
                a
            )));
        }
        Ok(self.log_table[a as usize])
    }

    /**
     * @return multiplicative inverse of a
     */
    pub fn inverse(&self, a: i32) -> Result<i32, Error> {
        if a <= 0 {
            return Err(Error::ArithmeticException(format!(
                "inverse error a: {}",
                a
            )));
        }

        let len = self.size - self.log_table[a as usize] - 1;
        if len < 0 {
            panic!("inverse error len: {}", len)
        }

        Ok(self.exp_table[len as usize])
    }

    /**
     * @return product of a and b in GF(size)
     */
    pub fn multiply(&self, a: i32, b: i32) -> Result<i32, Error> {
        if a < 0 || b < 0 {
            return Err(Error::IllegalArgumentException(format!(
                "multiply error a: {}, b: {}",
                a, b
            )));
        }
        if a == 0 || b == 0 {
            return Ok(0);
        }

        Ok(
            self.exp_table[((self.log_table[a as usize] + self.log_table[b as usize])
                % (self.size - 1)) as usize],
        )
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }

    pub fn get_generator_base(&self) -> i32 {
        self.generator_base
    }
}
