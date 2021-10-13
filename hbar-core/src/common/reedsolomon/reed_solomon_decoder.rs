use crate::common::reedsolomon::{GenericGF, GenericGFPoly};
use crate::{Error, ResultError};

use std::borrow::{Borrow, BorrowMut};
use std::rc::Rc;

pub struct ReedSolomonDecoder {
    field: Rc<GenericGF>,
}

impl ReedSolomonDecoder {
    pub fn new(field: Rc<GenericGF>) -> Self {
        ReedSolomonDecoder { field }
    }

    /**
     * <p>Decodes given set of received codewords, which include both data and error-correction
     * codewords. Really, this means it uses Reed-Solomon to detect and correct errors, in-place,
     * in the input.</p>
     *
     * @param received data and error-correction codewords
     * @param twoS number of error-correction codewords available
     * @throws ReedSolomonException if decoding fails for any reason
     */
    pub fn decode(&self, received: &mut Vec<i32>, twoS: i32) -> ResultError<()> {
        let poly = GenericGFPoly::new(Rc::clone(&self.field), received.to_vec())?;

        let mut syndromeCoefficients = vec![0; twoS as usize];
        let syndromeCoefficients_len = syndromeCoefficients.len();
        let mut noError = true;
        for i in 0..twoS {
            let eval = poly.evaluate_at(self.field.exp(i + self.field.get_generator_base())?)?;
            syndromeCoefficients[syndromeCoefficients_len - 1 - i as usize] = eval;
            if eval != 0 {
                noError = false;
            }
        }
        if noError {
            return Ok(());
        }

        let syndrome = GenericGFPoly::new(Rc::clone(&self.field), syndromeCoefficients)?;
        let sigmaOmega =
            self.runEuclideanAlgorithm(self.field.build_monomial(twoS, 1)?, syndrome, twoS)?;

        let sigma = &sigmaOmega[0];
        let omega = &sigmaOmega[1];
        let errorLocations = self.findErrorLocations(sigma)?;
        let errorMagnitudes = self.findErrorMagnitudes(omega, &errorLocations)?;
        for i in 0..errorLocations.len() {
            let position = received.len() as i32 - 1 - self.field.log(errorLocations[i])?;
            if position < 0 {
                return Err(Error::ReedSolomonException(String::from(
                    "Bad error location",
                )));
            }
            received[position as usize] =
                GenericGF::add_or_subtract(received[position as usize], errorMagnitudes[i]);
        }
        Ok(())
    }

    fn runEuclideanAlgorithm(
        &self,
        a: GenericGFPoly,
        b: GenericGFPoly,
        R: i32,
    ) -> ResultError<Vec<GenericGFPoly>> {
        // Assume a's degree is >= b's'
        let mut a = a;
        let mut b = b;
        if a.get_degree() < b.get_degree() {
            let temp = a;
            a = b;
            b = temp;
        }

        let mut rLast = a;
        let mut r = b;
        let mut tLast = self.field.get_zero()?;
        let mut t = self.field.get_one()?;

        // Run Euclidean algorithm until r's degree is less than R/2
        while 2 * r.get_degree() >= R {
            let rLastLast = rLast;
            let tLastLast = tLast;
            rLast = r;
            tLast = t.clone();

            // Divide rLastLast by rLast, with quotient in q and remainder in r
            if rLast.is_zero() {
                // Oops, Euclidean algorithm already terminated?
                return Err(Error::ReedSolomonException(String::from(
                    "r_{i-1} was zero",
                )));
            }
            r = rLastLast;
            let mut q = self.field.get_zero()?;
            let denominatorLeadingTerm = rLast.get_coefficient(rLast.get_degree())?;
            let dltInverse = self.field.inverse(denominatorLeadingTerm)?;
            while r.get_degree() >= rLast.get_degree() && !r.is_zero() {
                let degreeDiff = r.get_degree() - rLast.get_degree();
                let scale = self
                    .field
                    .multiply(r.get_coefficient(r.get_degree())?, dltInverse)?;
                q = q.add_or_subtract(&self.field.build_monomial(degreeDiff, scale)?)?;
                r = r.add_or_subtract(&rLast.multiply_by_monomial(degreeDiff, scale)?)?;
            }

            t = q.multiply_by_other(&tLast)?.add_or_subtract(&tLastLast)?;
            if r.get_degree() >= rLast.get_degree() {
                return Err(Error::IllegalStateException(format!(
                    "Division algorithm failed to reduce polynomial? r: {:?}, rLast: {:?}",
                    r, rLast
                )));
            }
        }

        let sigmaTildeAtZero = t.get_coefficient(0)?;
        if sigmaTildeAtZero == 0 {
            return Err(Error::ReedSolomonException(String::from(
                "sigmaTilde(0) was zero",
            )));
        }

        let inverse = self.field.inverse(sigmaTildeAtZero)?;
        let sigma = t.multiply_by_scalar(inverse)?;
        let omega = r.multiply_by_scalar(inverse)?;
        Ok(vec![sigma, omega])
    }

    fn findErrorLocations(&self, errorLocator: &GenericGFPoly) -> ResultError<Vec<i32>> {
        // This is a direct application of Chien's search
        let numErrors = errorLocator.get_degree();
        if numErrors == 1 {
            return Ok(vec![errorLocator.get_coefficient(1)?]);
        }
        let mut result = vec![0; numErrors as usize];
        let mut e = 0;
        for i in 0..self.field.get_size() {
            if e >= numErrors {
                break;
            }
            if errorLocator.evaluate_at(i)? == 0 {
                result[e as usize] = self.field.inverse(i)?;
                e += 1;
            }
        }
        if e != numErrors {
            return Err(Error::ReedSolomonException(String::from(
                "Error locator degree does not match number of roots",
            )));
        }

        Ok(result)
    }

    fn findErrorMagnitudes(
        &self,
        errorEvaluator: &GenericGFPoly,
        errorLocations: &Vec<i32>,
    ) -> ResultError<Vec<i32>> {
        // This is directly applying Forney's Formula
        let s = errorLocations.len();
        let mut result = vec![0; s];
        for i in 0..s {
            let xiInverse = self.field.inverse(errorLocations[i])?;
            let mut denominator = 1;
            for j in 0..s {
                if i != j {
                    //denominator = field.multiply(denominator,
                    //    GenericGF.addOrSubtract(1, field.multiply(errorLocations[j], xiInverse)));
                    // Above should work but fails on some Apple and Linux JDKs due to a Hotspot bug.
                    // Below is a funny-looking workaround from Steven Parkes
                    let term = self.field.multiply(errorLocations[j], xiInverse)?;
                    let termPlus1 = if (term & 0x1) == 0 {
                        term | 1
                    } else {
                        term & !1
                    };
                    denominator = self.field.multiply(denominator, termPlus1)?;
                }
            }
            result[i] = self.field.multiply(
                errorEvaluator.evaluate_at(xiInverse)?,
                self.field.inverse(denominator)?,
            )?;
            if self.field.get_generator_base() != 0 {
                result[i] = self.field.multiply(result[i], xiInverse)?;
            }
        }

        Ok(result)
    }
}
