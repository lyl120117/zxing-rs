use crate::ResultError;
use crate::Results;
use crate::{Binarizer, BinaryBitmap, LuminanceSource};
use crate::{DecodeHintType, DecodeHintValue};

use std::collections::HashMap;

/**
 * Implementations of this interface can decode an image of a barcode in some format into
 * the String it encodes. For example, {@link com.google.zxing.qrcode.QRCodeReader} can
 * decode a QR code. The decoder may optionally receive hints from the caller which may help
 * it decode more quickly or accurately.
 *
 * See {@link MultiFormatReader}, which attempts to determine what barcode
 * format is present within the image as well, and then decodes it accordingly.
 *
 */
pub trait Reader<B, S>
where
    B: Binarizer<S>,
    S: LuminanceSource,
{
    /**
     * Locates and decodes a barcode in some format within an image.
     *
     * @param image image of barcode to decode
     * @return String which the barcode encodes
     * @throws NotFoundException if no potential barcode is found
     * @throws ChecksumException if a potential barcode is found but does not pass its checksum
     * @throws FormatException if a potential barcode is found but format is invalid
     */
    fn decode(&self, image: &BinaryBitmap<B, S>) -> ResultError<Results>;

    /**
     * Locates and decodes a barcode in some format within an image. This method also accepts
     * hints, each possibly associated to some data, which may help the implementation decode.
     *
     * @param image image of barcode to decode
     * @param hints passed as a {@link Map} from {@link DecodeHintType}
     * to arbitrary data. The
     * meaning of the data depends upon the hint type. The implementation may or may not do
     * anything with these hints.
     * @return String which the barcode encodes
     * @throws NotFoundException if no potential barcode is found
     * @throws ChecksumException if a potential barcode is found but does not pass its checksum
     * @throws FormatException if a potential barcode is found but format is invalid
     */
    fn decode_hints(
        &self,
        image: &BinaryBitmap<B, S>,
        hints: &HashMap<DecodeHintType, DecodeHintValue>,
    ) -> ResultError<Results>;

    /**
     * Resets any internal state the implementation has after a decode, to prepare it
     * for reuse.
     */
    fn reset(&self);
}
