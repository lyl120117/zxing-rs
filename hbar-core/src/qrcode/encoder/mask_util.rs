use crate::qrcode::encoder::ByteMatrix;

pub struct MaskUtil;

impl MaskUtil {
    const N1: i32 = 3;
    const N2: i32 = 3;
    const N3: i32 = 40;
    const N4: i32 = 10;

    /**
     * Apply mask penalty rule 1 and return the penalty. Find repetitive cells with the same color and
     * give penalty to them. Example: 00000 or 11111.
     */
    pub fn apply_mask_penalty_rule1(matrix: &mut ByteMatrix) -> i32 {
        MaskUtil::apply_mask_penalty_rule1_internal(matrix, true)
            + MaskUtil::apply_mask_penalty_rule1_internal(matrix, false)
    }

    /**
     * Apply mask penalty rule 2 and return the penalty. Find 2x2 blocks with the same color and give
     * penalty to them. This is actually equivalent to the spec's rule, which is to find MxN blocks and give a
     * penalty proportional to (M-1)x(N-1), because this is the number of 2x2 blocks inside such a block.
     */
    pub fn apply_mask_penalty_rule2(matrix: &mut ByteMatrix) -> i32 {
        let mut penalty = 0;
        let array = matrix.get_array();
        let width = matrix.get_width();
        let height = matrix.get_height();
        for y in 0..height - 1 {
            let y = y as usize;
            let array_y = &array[y];
            for x in 0..width - 1 {
                let x = x as usize;
                let value = array_y[x];
                if value == array_y[x + 1]
                    && value == array[y + 1][x]
                    && value == array[y + 1][x + 1]
                {
                    penalty += 1;
                }
            }
        }
        return MaskUtil::N2 * penalty;
    }

    /**
     * Apply mask penalty rule 3 and return the penalty. Find consecutive runs of 1:1:3:1:1:4
     * starting with black, or 4:1:1:3:1:1 starting with white, and give penalty to them.  If we
     * find patterns like 000010111010000, we give penalty once.
     */
    pub fn apply_mask_penalty_rule3(matrix: &mut ByteMatrix) -> i32 {
        let mut num_penalties = 0;
        let array = matrix.get_array();
        let width = matrix.get_width() as usize;
        let height = matrix.get_height() as usize;
        for y in 0..height {
            let y = y as usize;
            for x in 0..width {
                let x = x as usize;
                let array_y = &array[y]; // We can at least optimize this access
                if x + 6 < width
                    && array_y[x] == 1
                    && array_y[x + 1] == 0
                    && array_y[x + 2] == 1
                    && array_y[x + 3] == 1
                    && array_y[x + 4] == 1
                    && array_y[x + 5] == 0
                    && array_y[x + 6] == 1
                    && (MaskUtil::is_white_horizontal(array_y, x as i32 - 4, x as i32)
                        || MaskUtil::is_white_horizontal(array_y, x as i32 + 7, x as i32 + 11))
                {
                    num_penalties += 1;
                }
            }
        }

        num_penalties * MaskUtil::N3
    }

    fn is_white_horizontal(row_array: &Vec<i32>, from: i32, to: i32) -> bool {
        let from = from.max(0);
        let to = to.min(row_array.len() as i32);
        for i in from..to {
            if row_array[i as usize] == 1 {
                return false;
            }
        }
        return true;
    }

    fn is_white_vertical(array: &Vec<Vec<i32>>, col: i32, from: i32, to: i32) -> bool {
        let from = from.max(0);
        let to = to.min(array.len() as i32);
        for i in from..to {
            if array[i as usize][col as usize] == 1 {
                return false;
            }
        }
        return true;
    }

    /**
     * Apply mask penalty rule 4 and return the penalty. Calculate the ratio of dark cells and give
     * penalty if the ratio is far from 50%. It gives 10 penalty for 5% distance.
     */
    pub fn apply_mask_penalty_rule4(matrix: &mut ByteMatrix) -> i32 {
        let mut num_dark_cells = 0;
        let array = matrix.get_array();
        let width = matrix.get_width();
        let height = matrix.get_height();
        for y in 0..height {
            let array_y = &array[y as usize];
            for x in 0..width {
                if array_y[x as usize] == 1 {
                    num_dark_cells += 1;
                }
            }
        }
        let num_total_cells = matrix.get_height() * matrix.get_width();
        let five_percent_variances =
            (num_dark_cells * 2 - num_total_cells).abs() * 10 / num_total_cells;

        five_percent_variances * MaskUtil::N4
    }

    /**
     * Return the mask bit for "getMaskPattern" at "x" and "y". See 8.8 of JISX0510:2004 for mask
     * pattern conditions.
     */
    pub fn get_data_mask_bit(mask_pattern: i32, x: i32, y: i32) -> bool {
        let intermediate;
        let temp;
        match mask_pattern {
            0 => {
                intermediate = (y + x) & 0x1;
            }
            1 => {
                intermediate = y & 0x1;
            }
            2 => {
                intermediate = x % 3;
            }
            3 => {
                intermediate = (y + x) % 3;
            }
            4 => {
                intermediate = ((y / 2) + (x / 3)) & 0x1;
            }
            5 => {
                temp = y * x;
                intermediate = (temp & 0x1) + (temp % 3);
            }
            6 => {
                temp = y * x;
                intermediate = ((temp & 0x1) + (temp % 3)) & 0x1;
            }
            7 => {
                temp = y * x;
                intermediate = ((temp % 3) + ((y + x) & 0x1)) & 0x1;
            }
            _ => {
                panic!("Invalid mask pattern: {}", mask_pattern);
            }
        }

        intermediate == 0
    }

    /**
     * Helper function for applyMaskPenaltyRule1. We need this for doing this calculation in both
     * vertical and horizontal orders respectively.
     */
    fn apply_mask_penalty_rule1_internal(matrix: &mut ByteMatrix, is_horizontal: bool) -> i32 {
        let mut penalty = 0;
        let i_limit;
        let j_limit;
        if is_horizontal {
            i_limit = matrix.get_height();
            j_limit = matrix.get_width();
        } else {
            i_limit = matrix.get_width();
            j_limit = matrix.get_height();
        }
        let array = matrix.get_array();
        for i in 0..i_limit {
            let mut num_same_bit_cells = 0;
            let mut prev_bit = -1;
            for j in 0..j_limit {
                let bit;
                if is_horizontal {
                    bit = array[i as usize][j as usize]
                } else {
                    bit = array[j as usize][i as usize]
                }
                if bit == prev_bit {
                    num_same_bit_cells += 1;
                } else {
                    if num_same_bit_cells >= 5 {
                        penalty += MaskUtil::N1 + (num_same_bit_cells - 5);
                    }
                    num_same_bit_cells = 1; // Include the cell itself.
                    prev_bit = bit;
                }
            }
            if num_same_bit_cells >= 5 {
                penalty += MaskUtil::N1 + (num_same_bit_cells - 5);
            }
        }
        penalty
    }
}
