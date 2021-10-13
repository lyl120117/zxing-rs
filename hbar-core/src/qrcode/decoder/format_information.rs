use super::ErrorCorrectionLevel;
use crate::{Error, ResultError};

use std::rc::Rc;
/**
 * <p>Encapsulates a QR Code's format information, including the data mask used and
 * error correction level.</p>
 *
 * @author Sean Owen
 * @see DataMask
 * @see ErrorCorrectionLevel
 */
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FormatInformation {
    errorCorrectionLevel: Rc<ErrorCorrectionLevel>,
    dataMask: i32,
}

impl FormatInformation {
    const FORMAT_INFO_MASK_QR: i32 = 0x5412;

    /**
     * See ISO 18004:2006, Annex C, Table C.1
     */
    const FORMAT_INFO_DECODE_LOOKUP: [[i32; 2]; 32] = [
        [0x5412, 0x00],
        [0x5125, 0x01],
        [0x5E7C, 0x02],
        [0x5B4B, 0x03],
        [0x45F9, 0x04],
        [0x40CE, 0x05],
        [0x4F97, 0x06],
        [0x4AA0, 0x07],
        [0x77C4, 0x08],
        [0x72F3, 0x09],
        [0x7DAA, 0x0A],
        [0x789D, 0x0B],
        [0x662F, 0x0C],
        [0x6318, 0x0D],
        [0x6C41, 0x0E],
        [0x6976, 0x0F],
        [0x1689, 0x10],
        [0x13BE, 0x11],
        [0x1CE7, 0x12],
        [0x19D0, 0x13],
        [0x0762, 0x14],
        [0x0255, 0x15],
        [0x0D0C, 0x16],
        [0x083B, 0x17],
        [0x355F, 0x18],
        [0x3068, 0x19],
        [0x3F31, 0x1A],
        [0x3A06, 0x1B],
        [0x24B4, 0x1C],
        [0x2183, 0x1D],
        [0x2EDA, 0x1E],
        [0x2BED, 0x1F],
    ];

    fn new(formatInfo: i32) -> ResultError<FormatInformation> {
        // Bits 3,4
        let errorCorrectionLevel = ErrorCorrectionLevel::forBits((formatInfo >> 3) & 0x03)?;
        // Bottom 3 bits
        let dataMask = formatInfo & 0x07;
        Ok(FormatInformation {
            errorCorrectionLevel,
            dataMask,
        })
    }

    fn numBitsDiffering(a: i32, b: i32) -> i32 {
        let result = a ^ b;
        result.count_ones() as i32
    }

    /**
     * @param maskedFormatInfo1 format info indicator, with mask still applied
     * @param maskedFormatInfo2 second copy of same info; both are checked at the same time
     *  to establish best match
     * @return information about the format it specifies, or {@code null}
     *  if doesn't seem to match any known pattern
     */
    pub fn decodeFormatInformation(
        maskedFormatInfo1: i32,
        maskedFormatInfo2: i32,
    ) -> ResultError<Option<FormatInformation>> {
        match FormatInformation::doDecodeFormatInformation(maskedFormatInfo1, maskedFormatInfo2)? {
            Some(formatInfo) => Ok(Some(formatInfo)),
            // Should return null, but, some QR codes apparently
            // do not mask this info. Try again by actually masking the pattern
            // first
            None => FormatInformation::doDecodeFormatInformation(
                maskedFormatInfo1 ^ FormatInformation::FORMAT_INFO_MASK_QR,
                maskedFormatInfo2 ^ FormatInformation::FORMAT_INFO_MASK_QR,
            ),
        }
    }

    fn doDecodeFormatInformation(
        maskedFormatInfo1: i32,
        maskedFormatInfo2: i32,
    ) -> ResultError<Option<FormatInformation>> {
        // Find the int in FORMAT_INFO_DECODE_LOOKUP with fewest bits differing
        let mut bestDifference = i32::MAX;
        let mut bestFormatInfo = 0;
        for decodeInfo in FormatInformation::FORMAT_INFO_DECODE_LOOKUP {
            let targetInfo = decodeInfo[0];
            if targetInfo == maskedFormatInfo1 || targetInfo == maskedFormatInfo2 {
                return Ok(Some(FormatInformation::new(decodeInfo[1])?));
            }
            let mut bitsDifference =
                FormatInformation::numBitsDiffering(maskedFormatInfo1, targetInfo);
            if bitsDifference < bestDifference {
                bestFormatInfo = decodeInfo[1];
                bestDifference = bitsDifference;
            }
            if maskedFormatInfo1 != maskedFormatInfo2 {
                // also try the other option
                bitsDifference = FormatInformation::numBitsDiffering(maskedFormatInfo2, targetInfo);
                if bitsDifference < bestDifference {
                    bestFormatInfo = decodeInfo[1];
                    bestDifference = bitsDifference;
                }
            }
        }
        // Hamming distance of the 32 masked codes is 7, by construction, so <= 3 bits
        // differing means we found a match
        if bestDifference <= 3 {
            return Ok(Some(FormatInformation::new(bestFormatInfo)?));
        }

        return Ok(None);
    }

    pub fn getErrorCorrectionLevel(&self) -> Rc<ErrorCorrectionLevel> {
        Rc::clone(&self.errorCorrectionLevel)
    }

    pub fn getDataMask(&self) -> i32 {
        self.dataMask
    }
}
