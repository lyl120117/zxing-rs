use crate::common::SystemTimes;
use crate::BarcodeFormat;
use crate::ResultPoint;
use crate::{ResultMetadataType, ResultMetadataValue};

use std::collections::HashMap;

pub struct Results {
    text: String,
    raw_bytes: Vec<u8>,
    num_bits: i32,
    result_points: Option<Vec<ResultPoint>>,
    format: BarcodeFormat,
    result_metadata: HashMap<ResultMetadataType, ResultMetadataValue>,
    timestamp: u64,
}
use std::fmt;
impl fmt::Display for Results {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "text: {}", self.text)
    }
}

impl Results {
    pub fn new(
        text: String,
        raw_bytes: Vec<u8>,
        result_points: Option<Vec<ResultPoint>>,
        format: BarcodeFormat,
    ) -> Results {
        Results::new1(
            text,
            raw_bytes,
            result_points,
            format,
            SystemTimes::timestamp(),
        )
    }

    pub fn new1(
        text: String,
        raw_bytes: Vec<u8>,
        result_points: Option<Vec<ResultPoint>>,
        format: BarcodeFormat,
        timestamp: u64,
    ) -> Results {
        let num_bits = if raw_bytes.is_empty() { 0 } else { 8 };
        Results::new2(text, raw_bytes, num_bits, result_points, format, timestamp)
    }

    pub fn new2(
        text: String,
        raw_bytes: Vec<u8>,
        num_bits: i32,
        result_points: Option<Vec<ResultPoint>>,
        format: BarcodeFormat,
        timestamp: u64,
    ) -> Results {
        Results {
            text: text,
            raw_bytes: raw_bytes,
            num_bits: num_bits,
            result_points: result_points,
            format: format,
            result_metadata: HashMap::new(),
            timestamp: timestamp,
        }
    }

    /**
     * @return raw text encoded by the barcode
     */
    pub fn get_text(&self) -> &String {
        &self.text
    }

    /**
     * @return raw bytes encoded by the barcode, if applicable, otherwise {@code null}
     */
    pub fn get_raw_bytes(&self) -> &Vec<u8> {
        &self.raw_bytes
    }

    /**
     * @return how many bits of {@link #get_raw_bytes()} are valid; typically 8 times its length
     * @since 3.3.0
     */
    pub fn get_num_bits(&self) -> i32 {
        self.num_bits
    }

    /**
     * @return points related to the barcode in the image. These are typically points
     *         identifying finder patterns or the corners of the barcode. The exact meaning is
     *         specific to the type of barcode that was decoded.
     */
    pub fn get_result_points(&self) -> &Option<Vec<ResultPoint>> {
        &self.result_points
    }

    /**
     * @return {@link BarcodeFormat} representing the format of the barcode that was decoded
     */
    pub fn get_barcode_format(&self) -> &BarcodeFormat {
        &self.format
    }

    /**
     * @return {@link Map} mapping {@link ResultMetadataType} keys to values. May be
     *   {@code null}. This contains optional metadata about what was detected about the barcode,
     *   like orientation.
     */
    pub fn get_result_metadata(&self) -> &HashMap<ResultMetadataType, ResultMetadataValue> {
        &self.result_metadata
    }

    pub fn put_metadata(&mut self, t: ResultMetadataType, value: ResultMetadataValue) {
        self.result_metadata.insert(t, value);
    }

    pub fn put_all_metadata(&mut self, metadata: HashMap<ResultMetadataType, ResultMetadataValue>) {
        for (t, value) in metadata.into_iter() {
            self.result_metadata.insert(t, value);
        }
    }

    pub fn add_result_points(&mut self, new_points: &Vec<ResultPoint>) {
        let old_points = &self.result_points;
        if let None = old_points {
            self.result_points = Some(new_points.clone());
        } else if new_points.len() > 0 {
            let mut all_points = Vec::new();
            for p in old_points.as_ref().unwrap() {
                all_points.push(p.clone())
            }
            for p in new_points {
                all_points.push(p.clone())
            }
            self.result_points = Some(all_points);
        }
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
}
