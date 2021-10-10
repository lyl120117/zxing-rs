pub struct MathUtils;

impl MathUtils {
    /**
     * @param a_x point A x coordinate
     * @param a_y point A y coordinate
     * @param b_x point B x coordinate
     * @param b_y point B y coordinate
     * @return Euclidean distance between points A and B
     */
    pub fn distance_f32(a_x: f32, a_y: f32, b_x: f32, b_y: f32) -> f32 {
        let x_diff = a_x - b_x;
        let y_diff = a_y - b_y;

        x_diff.powi(2) + y_diff.powi(2)
    }

    /**
     * @param a_x point A x coordinate
     * @param a_y point A y coordinate
     * @param b_x point B x coordinate
     * @param b_y point B y coordinate
     * @return Euclidean distance between points A and B
     */
    pub fn distance_i32(a_x: i32, a_y: i32, b_x: i32, b_y: i32) -> f32 {
        let x_diff = (a_x - b_x) as f32;
        let y_diff = (a_y - b_y) as f32;

        x_diff.powi(2) + y_diff.powi(2)
    }
}

#[cfg(test)]
mod math_utils_test {
    use super::*;

    #[test]
    fn pow() {
        let x_diff = 2f32;
        assert_eq!(4f32, x_diff.powi(2));
    }
}
