use crate::common::BitMatrix;
use crate::{Error, ResultError};

use super::FormatInformation;
use super::Version;

pub struct BitMatrixParser {
    bitMatrix: BitMatrix,
    parsedVersion: Option<Version>,
    parsedFormatInfo: Option<FormatInformation>,
    mirror: bool,
}

impl BitMatrixParser {
    /**
     * @param bitMatrix {@link BitMatrix} to parse
     * @throws FormatException if dimension is not >= 21 and 1 mod 4
     */
    fn new(bitMatrix: &BitMatrix) -> ResultError<Self> {
        let dimension = bitMatrix.getHeight();
        if dimension < 21 || (dimension & 0x03) != 1 {
            return Err(Error::FormatException(String::from("")));
        }

        Ok(BitMatrixParser {
            bitMatrix: bitMatrix.clone(),
            parsedVersion: None,
            parsedFormatInfo: None,
            mirror: false,
        })
    }

    /**
     * <p>Reads format information from one of its two locations within the QR Code.</p>
     *
     * @return {@link FormatInformation} encapsulating the QR Code's format info
     * @throws FormatException if both format information locations cannot be parsed as
     * the valid encoding of format information
     */
    fn readFormatInformation(&mut self) -> ResultError<FormatInformation> {
        if let Some(parsedFormatInfo) = &self.parsedFormatInfo {
            return Ok(parsedFormatInfo.clone());
        }

        // Read top-left format info bits
        let mut formatInfoBits1 = 0;
        for i in 0..6 {
            formatInfoBits1 = self.copyBit(i, 8, formatInfoBits1);
        }
        // .. and skip a bit in the timing pattern ...
        formatInfoBits1 = self.copyBit(7, 8, formatInfoBits1);
        formatInfoBits1 = self.copyBit(8, 8, formatInfoBits1);
        formatInfoBits1 = self.copyBit(8, 7, formatInfoBits1);
        // .. and skip a bit in the timing pattern ...
        for j in (0..6).rev() {
            formatInfoBits1 = self.copyBit(8, j, formatInfoBits1);
        }

        // Read the top-right/bottom-left pattern too
        let dimension = self.bitMatrix.getHeight();
        let mut formatInfoBits2 = 0;
        let jMin = dimension - 7;
        for j in (jMin..dimension).rev() {
            formatInfoBits2 = self.copyBit(8, j, formatInfoBits2);
        }
        for i in dimension - 8..dimension {
            formatInfoBits2 = self.copyBit(i, 8, formatInfoBits2);
        }

        self.parsedFormatInfo =
            FormatInformation::decodeFormatInformation(formatInfoBits1, formatInfoBits2)?;
        if let Some(formatInfo) = &self.parsedFormatInfo {
            return Ok(formatInfo.clone());
        }
        Err(Error::FormatException(String::from("")))
    }

    // /**
    //  * <p>Reads version information from one of its two locations within the QR Code.</p>
    //  *
    //  * @return {@link Version} encapsulating the QR Code's version
    //  * @throws FormatException if both version information locations cannot be parsed as
    //  * the valid encoding of version information
    //  */
    // fn readVersion(&self) -> Version{

    // }

    fn copyBit(&self, i: i32, j: i32, versionBits: i32) -> i32 {
        let bit = if self.mirror {
            self.bitMatrix.get(j as u32, i as u32)
        } else {
            self.bitMatrix.get(i as u32, j as u32)
        };
        if bit {
            (versionBits << 1) | 0x1
        } else {
            versionBits << 1
        }
    }
}
