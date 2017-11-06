pub const BLOCK_SIZE: usize = 8;

fn verify_length<T>(buffer: &[T]) {
    let sqrt_size = (buffer.len() as f64).sqrt() as usize;
    assert!(sqrt_size * sqrt_size == buffer.len(), "The length of the input buffer must be a perfect square. Got {}", buffer.len());
    assert!(sqrt_size % BLOCK_SIZE == 0, "The side length of the square array must be a multiple of {}. Got {}", BLOCK_SIZE, sqrt_size);
}

#[inline(always)]
fn transpose_block<T: Copy>(buffer: &mut [T], side_length: usize, block_x: usize, block_y: usize) {
    for inner_x in 0..BLOCK_SIZE {
        for inner_y in 0..BLOCK_SIZE {
            let x = block_x * BLOCK_SIZE + inner_x;
            let y = block_y * BLOCK_SIZE + inner_y;

            let input_index = x + y * side_length;
            let output_index = y + x * side_length;

            buffer.swap(input_index, output_index);
        }
    }
}

#[inline(always)]
fn transpose_diagonal_block<T: Copy>(buffer: &mut [T], side_length: usize, diagonal_index: usize) {
    for inner_x in 0..BLOCK_SIZE {
        for inner_y in (inner_x + 1)..BLOCK_SIZE {
            let x = diagonal_index * BLOCK_SIZE + inner_x;
            let y = diagonal_index * BLOCK_SIZE + inner_y;

            let input_index = x + y * side_length;
            let output_index = y + x * side_length;

            buffer.swap(input_index, output_index);
        }
    }
}

/// Given an array of size width * height, representing a flattened 2D array,
/// transpose the rows and columns of that 2D array into the output
// Use "Loop tiling" to improve cache-friendliness
pub fn transpose_square_inplace<T: Copy>(buffer: &mut [T]) {
    verify_length(buffer);

    let side_length = (buffer.len() as f64).sqrt() as usize;

    let block_count = side_length / BLOCK_SIZE;

    for y_block in 0..block_count {
        transpose_diagonal_block(buffer, side_length, y_block);
        for x_block in (y_block + 1)..block_count {
            transpose_block(buffer, side_length, x_block, y_block);
        }
    }
}


#[cfg(test)]
mod unit_tests {
    use super::*;

    extern crate rand;
    use self::rand::{Rng, StdRng, SeedableRng};

    const RNG_SEED: [usize; 5] = [1910, 11431, 4984, 14828, 12226];

    fn random_array(length: usize) -> Vec<u32> {
        let mut result = Vec::with_capacity(length);
        let mut rng: StdRng = SeedableRng::from_seed(&RNG_SEED[..]);
        for _ in 0..length {
            result.push(rng.next_u32())
        }
        result
    }

    #[test]
    fn test_transpose() {
        for i in 0..5 {
            let side_length = i * BLOCK_SIZE;

            let len = side_length * side_length;

            let input = random_array(len);
            let mut transposed = input.clone();

            transpose_square_inplace(&mut transposed);

            for x in 0..side_length {
                for y in 0..side_length {
                    assert_eq!(input[x + y * side_length], transposed[y + x * side_length], "x = {}, y = {}", x, y);
                }
            }
        }
    }
}
