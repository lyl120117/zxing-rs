use strum_macros::EnumString;
use strum_macros::ToString;

#[derive(Debug, PartialEq, Eq, Hash, EnumString, ToString)]
pub enum BarcodeFormat {
    /** Aztec 2D barcode format. */
    Aztec,

    /** CODABAR 1D format. */
    CodeBar,

    /** Code 39 1D format. */
    Code39,

    /** Code 93 1D format. */
    Code93,

    /** Code 128 1D format. */
    Code128,

    /** Data Matrix 2D barcode format. */
    DataMatrix,

    /** EAN-8 1D format. */
    Ean8,

    /** EAN-13 1D format. */
    Ean13,

    /** ITF (Interleaved Two of Five) 1D format. */
    ITF,

    /** MaxiCode 2D barcode format. */
    MaxiCode,

    /** PDF417 format. */
    PDF417,

    /** QR Code 2D barcode format. */
    QRCode,

    /** RSS 14 */
    RSS14,

    /** RSS EXPANDED */
    RssExpanded,

    /** UPC-A 1D format. */
    UpcA,

    /** UPC-E 1D format. */
    UpcE,

    /** UPC/EAN extension format. Not a stand-alone format. */
    UpcEanExtension,
}
