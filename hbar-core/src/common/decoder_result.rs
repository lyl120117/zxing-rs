pub struct DecoderResult {
    rawBytes: Vec<u8>,
    numBits: i32,
    text: String,
    byteSegments: Vec<Vec<u8>>,
    ecLevel: String,
    structuredAppendParity: i32,
    structuredAppendSequenceNumber: i32,
    symbologyModifier: i32,
}

impl DecoderResult {
    pub fn new(
        rawBytes: Vec<u8>,
        text: String,
        byteSegments: Vec<Vec<u8>>,
        ecLevel: String,
    ) -> Self {
        DecoderResult::new3(rawBytes, text, byteSegments, ecLevel, -1, -1, 0)
    }

    pub fn new1(
        rawBytes: Vec<u8>,
        text: String,
        byteSegments: Vec<Vec<u8>>,
        ecLevel: String,
        symbologyModifier: i32,
    ) -> Self {
        DecoderResult::new3(
            rawBytes,
            text,
            byteSegments,
            ecLevel,
            -1,
            -1,
            symbologyModifier,
        )
    }

    pub fn new2(
        rawBytes: Vec<u8>,
        text: String,
        byteSegments: Vec<Vec<u8>>,
        ecLevel: String,
        saSequence: i32,
        saParity: i32,
    ) -> Self {
        DecoderResult::new3(
            rawBytes,
            text,
            byteSegments,
            ecLevel,
            saSequence,
            saParity,
            0,
        )
    }

    pub fn new3(
        rawBytes: Vec<u8>,
        text: String,
        byteSegments: Vec<Vec<u8>>,
        ecLevel: String,
        saSequence: i32,
        saParity: i32,
        symbologyModifier: i32,
    ) -> Self {
        let numBits = rawBytes.len() * 8;
        DecoderResult {
            rawBytes: rawBytes,
            numBits: numBits as i32,
            text: text,
            byteSegments: byteSegments,
            ecLevel: ecLevel,
            structuredAppendParity: saParity,
            structuredAppendSequenceNumber: saSequence,
            symbologyModifier: symbologyModifier,
        }
    }

    /**
     * @return raw bytes representing the result, or {@code null} if not applicable
     */
    pub fn getRawBytes(&self) -> &Vec<u8> {
        &self.rawBytes
    }

    /**
     * @return how many bits of {@link #getRawBytes()} are valid; typically 8 times its length
     * @since 3.3.0
     */
    pub fn getNumBits(&self) -> i32 {
        self.numBits
    }

    /**
     * @param numBits overrides the number of bits that are valid in {@link #getRawBytes()}
     * @since 3.3.0
     */
    pub fn setNumBits(&mut self, numBits: i32) {
        self.numBits = numBits;
    }

    /**
     * @return text representation of the result
     */
    pub fn getText(&self) -> &String {
        &self.text
    }

    /**
     * @return list of byte segments in the result, or {@code null} if not applicable
     */
    pub fn getByteSegments(&self) -> &Vec<Vec<u8>> {
        &self.byteSegments
    }

    /**
     * @return name of error correction level used, or {@code null} if not applicable
     */
    pub fn getECLevel(&self) -> &String {
        &self.ecLevel
    }

    // /**
    //  * @return number of errors corrected, or {@code null} if not applicable
    //  */
    // pub fn getErrorsCorrected(&self) -> i32{
    //     self.error
    // }
    pub fn hasStructuredAppend(&self) -> bool {
        self.structuredAppendParity >= 0 && self.structuredAppendSequenceNumber >= 0
    }

    pub fn getStructuredAppendParity(&self) -> i32 {
        self.structuredAppendParity
    }

    pub fn getStructuredAppendSequenceNumber(&self) -> i32 {
        self.structuredAppendSequenceNumber
    }

    pub fn getSymbologyModifier(&self) -> i32 {
        self.symbologyModifier
    }
}
