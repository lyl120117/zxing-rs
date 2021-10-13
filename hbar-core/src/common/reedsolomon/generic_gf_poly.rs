use crate::common::reedsolomon::GenericGF;
use crate::Error;

use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)] // we implement the Copy trait
pub struct GenericGFPoly {
    field: Rc<GenericGF>,
    coefficients: Vec<i32>,
}

impl GenericGFPoly {
    /**
     * @param field the {@link GenericGF} instance representing the field to use
     * to perform computations
     * @param coefficients coefficients as ints representing elements of GF(size), arranged
     * from most significant (highest-power term) coefficient to least significant
     * @throws IllegalArgumentException if argument is null or empty,
     * or if leading coefficient is 0 and this is not a
     * constant polynomial (that is, it is not the monomial "0")
     */
    pub fn new(field: Rc<GenericGF>, coefficients: Vec<i32>) -> Result<GenericGFPoly, Error> {
        if coefficients.len() == 0 {
            return Err(Error::IllegalArgumentException(format!(
                "coefficients length is zero"
            )));
        }

        let coefficients_length = coefficients.len();
        let coefficients_;
        if coefficients_length > 1 && coefficients[0] == 0 {
            // Leading term must be non-zero for anything except the constant polynomial "0"
            let mut first_non_zero = 1;
            while first_non_zero < coefficients_length && coefficients[first_non_zero] == 0 {
                first_non_zero += 1
            }
            if first_non_zero == coefficients_length {
                coefficients_ = vec![0]
            } else {
                coefficients_ = (&coefficients[first_non_zero..]).to_vec()
            }
        } else {
            coefficients_ = coefficients
        }

        Ok(GenericGFPoly {
            field: field,
            coefficients: coefficients_,
        })
    }

    pub fn get_coefficients(&self) -> &Vec<i32> {
        &self.coefficients
    }

    /**
     * @return degree of this polynomial
     */
    pub fn get_degree(&self) -> i32 {
        self.coefficients.len() as i32 - 1
    }

    /**
     * @return true iff this polynomial is the monomial "0"
     */
    pub fn is_zero(&self) -> bool {
        self.coefficients[0] == 0
    }

    /**
     * @return coefficient of x^degree term in this polynomial
     */
    pub fn get_coefficient(&self, degree: i32) -> Result<i32, Error> {
        if degree < 0 {
            return Err(Error::IllegalArgumentException(format!(
                "get coefficient error degree: {}",
                degree
            )));
        }
        Ok(self.coefficients[self.coefficients.len() - 1 - degree as usize])
    }

    /**
     * @return evaluation of this polynomial at a given point
     */
    pub fn evaluate_at(&self, a: i32) -> Result<i32, Error> {
        if a < 0 {
            return Err(Error::IllegalArgumentException(format!(
                "evaluate at error a: {}",
                a
            )));
        }
        if a == 0 {
            // Just return the x^0 coefficient
            return self.get_coefficient(0);
        }
        if a == 1 {
            // Just the sum of the coefficients
            let mut result = 0;
            for coefficient in &self.coefficients {
                result = GenericGF::add_or_subtract(result, *coefficient)
            }
            return Ok(result);
        }
        let mut result = self.coefficients[0];
        let size = self.coefficients.len();
        for i in 0..size {
            result =
                GenericGF::add_or_subtract(self.field.multiply(a, result)?, self.coefficients[i])
        }
        Ok(result)
    }

    pub fn add_or_subtract(&self, other: &GenericGFPoly) -> Result<GenericGFPoly, Error> {
        if self.field != other.field {
            return Err(Error::IllegalArgumentException(
                "GenericGFPolys do not have same GenericGF field".to_string(),
            ));
        }
        if self.is_zero() {
            return Ok(other.clone());
        }
        if other.is_zero() {
            return Ok(self.clone());
        }

        let mut smaller_coefficients = &self.coefficients;
        let mut larger_coefficients = &other.coefficients;
        if smaller_coefficients.len() > larger_coefficients.len() {
            let tmp = smaller_coefficients;
            smaller_coefficients = larger_coefficients;
            larger_coefficients = tmp;
        }
        let mut sum_diff: Vec<i32> = vec![0; larger_coefficients.len()];
        let length_diff = larger_coefficients.len() - smaller_coefficients.len();
        // Copy high-order terms only found in higher-degree polynomial's coefficients
        for i in 0..length_diff {
            sum_diff[i] = larger_coefficients[i]
        }
        for i in length_diff..larger_coefficients.len() {
            sum_diff[i] = GenericGF::add_or_subtract(
                smaller_coefficients[i - length_diff],
                larger_coefficients[i],
            )
        }

        GenericGFPoly::new(self.field.clone(), sum_diff)
    }

    pub fn multiply_by_other(&self, other: &GenericGFPoly) -> Result<GenericGFPoly, Error> {
        if self.field != other.field {
            return Err(Error::IllegalArgumentException(
                "GenericGFPolys do not have same GenericGF field".to_string(),
            ));
        }
        if self.is_zero() || other.is_zero() {
            return self.field.get_zero();
        }
        let a_coefficients = &self.coefficients;
        let a_length = a_coefficients.len();
        let b_coefficients = &other.coefficients;
        let b_length = b_coefficients.len();
        let mut product = vec![0; a_length + b_length - 1];
        for i in 0..a_length {
            let a_coeff = a_coefficients[i];
            for j in 0..b_length {
                product[i + j] = GenericGF::add_or_subtract(
                    product[i + j],
                    self.field.multiply(a_coeff, b_coefficients[j])?,
                );
            }
        }

        GenericGFPoly::new(self.field.clone(), product)
    }

    pub fn multiply_by_scalar(&self, scalar: i32) -> Result<GenericGFPoly, Error> {
        if scalar == 0 {
            return self.field.get_zero();
        }

        if scalar == 1 {
            return Ok(self.clone());
        }
        let size = self.coefficients.len();
        let mut product = vec![0; size];
        for i in 0..size {
            product[i] = self.field.multiply(self.coefficients[i], scalar)?;
        }

        GenericGFPoly::new(self.field.clone(), product)
    }

    pub fn multiply_by_monomial(
        &self,
        degree: i32,
        coefficient: i32,
    ) -> Result<GenericGFPoly, Error> {
        if degree < 0 {
            return Err(Error::IllegalArgumentException(format!(
                "multiply by monomial error degree: {}",
                degree
            )));
        }

        if coefficient == 0 {
            return self.field.get_zero();
        }

        let size = self.coefficients.len();
        let mut product = vec![0; size + degree as usize];
        for i in 0..size {
            product[i] = self.field.multiply(self.coefficients[i], coefficient)?;
        }

        GenericGFPoly::new(self.field.clone(), product)
    }

    pub fn divide(&self, other: &GenericGFPoly) -> Result<Vec<GenericGFPoly>, Error> {
        if self.field != other.field {
            return Err(Error::IllegalArgumentException(
                "GenericGFPolys do not have same GenericGF field".to_string(),
            ));
        }
        if other.is_zero() {
            return Err(Error::IllegalArgumentException("Divide by 0".to_string()));
        }
        let mut quotient = self.field.get_zero()?;
        let mut remainder = self.clone();

        let denominator_leading_term = other.get_coefficient(other.get_degree())?;
        let inverse_denominator_leading_term = self.field.inverse(denominator_leading_term)?;

        while remainder.get_degree() >= other.get_degree() && !remainder.is_zero() {
            let degree_difference = remainder.get_degree() - other.get_degree();
            let scale = self.field.multiply(
                remainder.get_coefficient(remainder.get_degree())?,
                inverse_denominator_leading_term,
            )?;
            let term = other.multiply_by_monomial(degree_difference, scale)?;
            let iteration_quotient = self.field.build_monomial(degree_difference, scale)?;
            quotient = quotient.add_or_subtract(&iteration_quotient)?;
            let tmp = remainder.add_or_subtract(&term)?;
            remainder = tmp.clone();
        }

        Ok(vec![quotient, remainder])
    }
}
