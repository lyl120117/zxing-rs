use crate::common::reedsolomon::{GenericGF, GenericGFPoly};
use crate::Error;

pub struct ReedSolomonEncoder {
    field: GenericGF,
    cached_generators: Vec<GenericGFPoly>,
}

impl ReedSolomonEncoder {
    pub fn new(field: &GenericGF) -> Result<ReedSolomonEncoder, Error> {
        let mut cached_generators = Vec::new();
        cached_generators.push(GenericGFPoly::new(field.clone(), vec![1])?);
        Ok(ReedSolomonEncoder {
            field: field.clone(),
            cached_generators: cached_generators,
        })
    }

    pub fn build_generator(&mut self, degree: i32) -> Result<GenericGFPoly, Error> {
        if degree > self.cached_generators.len() as i32 {
            let mut last_generator = self.cached_generators.get(self.cached_generators.len() - 1);
            let mut last_generator = match last_generator {
                Some(generator) => generator.clone(),
                None => {
                    return Err(Error::IllegalArgumentException(
                        "Build generator error".to_string(),
                    ))
                }
            };
            for d in self.cached_generators.len()..(degree + 1) as usize {
                let other = GenericGFPoly::new(
                    self.field.clone(),
                    vec![
                        1,
                        self.field
                            .exp(d as i32 - 1 + self.field.get_generator_base())?,
                    ],
                )?;
                let next_generator = last_generator.multiply_by_other(&other)?;
                self.cached_generators.push(next_generator.clone());
                last_generator = next_generator;
            }
        }
        let last_generator = self.cached_generators.get(degree as usize);
        let last_generator = match last_generator {
            Some(generator) => generator.clone(),
            None => {
                return Err(Error::IllegalArgumentException(
                    "Build generator error".to_string(),
                ))
            }
        };
        Ok(last_generator)
    }

    pub fn encode(&mut self, to_encode: &mut Vec<i32>, ec_bytes: i32) -> Result<(), Error> {
        if ec_bytes <= 0 {
            return Err(Error::IllegalArgumentException(format!(
                "No error correction bytes ec_bytes is: {}",
                ec_bytes
            )));
        }
        let data_bytes = to_encode.len() as i32 - ec_bytes;
        if data_bytes <= 0 {
            return Err(Error::IllegalArgumentException(
                "No data bytes provided".to_string(),
            ));
        }
        let data_bytes = data_bytes as usize;
        let generator = self.build_generator(ec_bytes)?;
        let info_coefficients = (&to_encode[0..data_bytes]).to_vec();
        let mut info = GenericGFPoly::new(self.field.clone(), info_coefficients)?;
        info = info.multiply_by_monomial(ec_bytes, 1)?;
        let remainder = &info.divide(&generator)?[1];
        let coefficients = remainder.get_coefficients();
        let num_zero_coefficients = ec_bytes - coefficients.len() as i32;
        if num_zero_coefficients < 0 {
            return Err(Error::IllegalArgumentException(format!(
                "Error num zero coefficients: {}",
                num_zero_coefficients
            )));
        }
        let num_zero_coefficients = num_zero_coefficients as usize;
        for i in 0..num_zero_coefficients {
            to_encode[data_bytes + i] = 0;
        }
        let start = data_bytes + num_zero_coefficients;

        for i in 0..coefficients.len() {
            to_encode[start + i] = coefficients[i]
        }
        Ok(())
    }
}
