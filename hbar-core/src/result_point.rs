use crate::common::MathUtils;

#[derive(Debug, PartialEq, Clone)]
pub struct ResultPoint {
    x: f32,
    y: f32,
}

use std::fmt;
impl fmt::Display for ResultPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl ResultPoint {
    pub fn new(x: f32, y: f32) -> ResultPoint {
        ResultPoint { x: x, y: y }
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }

    /**
     * Orders an array of three ResultPoints in an order [A,B,C] such that AB is less than AC
     * and BC is less than AC, and the angle between BC and BA is less than 180 degrees.
     *
     * @param patterns array of three {@code ResultPoint} to order
     */
    pub fn order_best_patterns(patterns: &mut Vec<&ResultPoint>) {
        // Find distances between pattern centers
        let zero_one_distance = ResultPoint::distance(patterns[0], patterns[1]);
        let one_two_distance = ResultPoint::distance(patterns[1], patterns[2]);
        let zero_two_distance = ResultPoint::distance(patterns[0], patterns[2]);

        let mut point_a;
        let point_b;
        let mut point_c;
        // Assume one closest to other two is B; A and C will just be guesses at first
        if one_two_distance >= zero_one_distance && one_two_distance >= zero_two_distance {
            point_b = patterns[0];
            point_a = patterns[1];
            point_c = patterns[2];
        } else if zero_two_distance >= one_two_distance && zero_two_distance >= zero_one_distance {
            point_b = patterns[1];
            point_a = patterns[0];
            point_c = patterns[2];
        } else {
            point_b = patterns[2];
            point_a = patterns[0];
            point_c = patterns[1];
        }

        // Use cross product to figure out whether A and C are correct or flipped.
        // This asks whether BC x BA has a positive z component, which is the arrangement
        // we want for A, B, C. If it's negative, then we've got it flipped around and
        // should swap A and C.
        if ResultPoint::cross_product_z(point_a, point_b, point_c) < 0.0f32 {
            let temp = point_a;
            point_a = point_c;
            point_c = temp;
        }
        patterns[0] = point_a;
        patterns[1] = point_b;
        patterns[2] = point_c;
    }

    /**
     * @param pattern1 first pattern
     * @param pattern2 second pattern
     * @return distance between two points
     */
    pub fn distance(pattern1: &ResultPoint, pattern2: &ResultPoint) -> f32 {
        MathUtils::distance_f32(pattern1.x, pattern1.y, pattern2.x, pattern2.y)
    }

    /**
     * Returns the z component of the cross product between vectors BC and BA.
     */
    fn cross_product_z(point_a: &ResultPoint, point_b: &ResultPoint, point_c: &ResultPoint) -> f32 {
        let b_x = point_b.x;
        let b_y = point_b.y;
        ((point_c.x - b_x) * (point_a.y - b_y)) - ((point_c.y - b_y) * (point_a.x - b_x))
    }
}

#[cfg(test)]
mod result_point_tests {
    use super::ResultPoint;
    #[test]
    fn equals() {
        let r1 = ResultPoint::new(1.0, 3.0);
        let r2 = ResultPoint::new(2.0, 3.0);
        assert_ne!(r1, r2);
        assert_eq!(r1, r1.clone());
    }
}
