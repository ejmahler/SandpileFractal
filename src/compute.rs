
extern crate rayon;

use std::cmp::{min, max};
use self::rayon::prelude::*;
use common::{InitialCell, FractalResult};

const TOPPLE_AMOUNT: u32 = 4;
const TOPPLE_WIDTH: usize = 3;
const MARGIN: usize = TOPPLE_WIDTH / 2;

pub fn compute_fractal_data(initial_configuration: &[InitialCell]) -> FractalResult {

	let mut side_length = initial_configuration.iter().map(|entry| max(entry.x, entry.y)).max().unwrap() + 1;


	let mut counting_array: Vec<u32> = vec![0; side_length * side_length];
	let mut write_array: Vec<u32> = vec![0; side_length * side_length];

	for entry in initial_configuration {
		write_array[entry.y * side_length + entry.x] = entry.value;
	}

	
	let mut total_iterations = 0;
	let mut total_redistributions: i64 = 0;
	{
		let mut read_array = write_array.clone();
		let mut next_check = maybe_reallocate(&mut write_array, &mut read_array, &mut counting_array, &mut side_length);

		loop
		{
			total_iterations = total_iterations+1;

			let mut current_redist = 0;

			for i in 0..TOPPLE_WIDTH {
				let offset = i * side_length;
				let limit = (side_length - i) / TOPPLE_WIDTH;

				let read_iter = read_array[offset..].par_chunks(side_length * TOPPLE_WIDTH).take(limit);
				let write_iter = write_array[offset..].par_chunks_mut(side_length * TOPPLE_WIDTH).take(limit);
				let counting_iter = counting_array[offset..].par_chunks_mut(side_length * TOPPLE_WIDTH).take(limit);

				current_redist += read_iter.zip(write_iter).zip(counting_iter).map(|((input_chunk, output_chunk), counting_chunk)| process_row(input_chunk, output_chunk, counting_chunk, side_length)).sum();
			}
			
			if current_redist > 0 {
				total_redistributions += current_redist as i64;

				next_check -= 1;
				if next_check == 0 {
					next_check = maybe_reallocate(&mut write_array, &mut read_array, &mut counting_array, &mut side_length);
				} else {
					copy_data(&write_array, &mut read_array);
				}
			} else {
				break;
			}
		}
	}

	FractalResult {
		initial_configuration: initial_configuration.to_vec(),
		sand_data: write_array.into_iter().map(|value| value as u8).collect(),
		count_data: counting_array,
		side_length: side_length,

		total_redistributions: total_redistributions,
		total_iterations: total_iterations,
	}
}

fn process_row(input_data: &[u32], output_data: &mut [u32], counting_data: &mut [u32], width: usize) -> i32 {

	assert!(input_data.len() == width * TOPPLE_WIDTH);
	assert!(output_data.len() == width * TOPPLE_WIDTH);

	let data_start = MARGIN * width + MARGIN;
	let data_end = (MARGIN + 1) * width - MARGIN;

	let mut num_redistributions = 0;
	for i in data_start..data_end {
		let val = input_data[i];
		if val >= TOPPLE_AMOUNT {
			num_redistributions += 1;
			counting_data[i] += 1;

			let distribute = val / TOPPLE_AMOUNT;

			output_data[i - width] += distribute;

			output_data[i - 1] += distribute;
			output_data[i] -= distribute * TOPPLE_AMOUNT;
			output_data[i + 1] += distribute;

			output_data[i + width] += distribute;
		}
	}
	num_redistributions
}

fn maybe_reallocate(main_array: &mut Vec<u32>, secondary_array: &mut Vec<u32>, counting_array: &mut Vec<u32>, side_length: &mut usize) -> usize {

	// find the bounds of the fractal data, so that we can re-center it inside the new array
	let mut miny = 0;
	'outer_miny: for y in 0..*side_length {
		for x in 0..*side_length {
			let index = y * *side_length + x;
			if main_array[index] >= TOPPLE_AMOUNT {
				miny = y;
				break 'outer_miny;
			}
		}
	}

	let mut maxy = 0;
	'outer_maxy: for y in (0..*side_length).rev() {
		for x in (0..*side_length).rev() {
			let index = y * *side_length + x;
			if main_array[index] >= TOPPLE_AMOUNT {
				maxy = y;
				break 'outer_maxy;
			}
		}
	}

	let mut minx = 0;
	'outer_minx: for x in 0..*side_length {
		for y in miny..maxy {
			let index = y * *side_length + x;
			if main_array[index] >= TOPPLE_AMOUNT {
				minx = x;
				break 'outer_minx;
			}
		}
	}

	let mut maxx = 0;
	'outer_maxx: for x in (0..*side_length).rev() {
		for y in (miny..maxy).rev() {
			let index = y * *side_length + x;
			if main_array[index] >= TOPPLE_AMOUNT {
				maxx = x;
				break 'outer_maxx;
			}
		}
	}

	let closest_vertical = min(miny, *side_length - maxy - 1);
	let closest_horizontal = min(minx, *side_length - maxx - 1);

	let closest = min(closest_vertical, closest_horizontal);

	if closest < MARGIN {
		const MIN_SIZE: usize = 800;
		const STANDARD_INCREASE: usize = 8;

		let new_side_length = max(MIN_SIZE, *side_length + STANDARD_INCREASE);
		let increase = new_side_length - *side_length;

		let mut new_main_array = vec![0; new_side_length * new_side_length];
		let mut new_counting_array = new_main_array.clone();

		let size_x = maxx - minx + 1;
		let size_y = maxy - miny + 1;

		let new_x_begin = new_side_length/2 - size_x/2;
		let new_y_begin = new_side_length/2 - size_y/2;

		rayon::join(
			|| {
				let old_data_rows = main_array.chunks(*side_length).skip(miny).take(size_y);
				let new_data_rows = new_main_array.chunks_mut(new_side_length).skip(new_y_begin).take(size_y);

				for (old_row, new_row) in old_data_rows.zip(new_data_rows) {
					new_row[new_x_begin..new_x_begin+size_x].copy_from_slice(&old_row[minx..minx+size_x]);
				}
			},
			|| {
				let old_counting_rows = counting_array.chunks(*side_length).skip(miny).take(size_y);
				let new_counting_rows = new_counting_array.chunks_mut(new_side_length).skip(new_y_begin).take(size_y);

				for (old_row, new_row) in old_counting_rows.zip(new_counting_rows) {
					new_row[new_x_begin..new_x_begin+size_x].copy_from_slice(&old_row[minx..minx+size_x]);
				}
			}
		);

		*secondary_array = new_main_array.clone();
		*main_array = new_main_array;
		*counting_array = new_counting_array;
		*side_length = new_side_length;

		increase / 4
	} else {
		copy_data(&main_array, &mut *secondary_array);

		closest / 2
	}
}

fn copy_data<T: Copy + Sync + Send>(src: &[T], dst: &mut [T]) {
	let chunk_size = src.len() / 8;
	src.par_chunks(chunk_size).zip(dst.par_chunks_mut(chunk_size)).for_each(|(input_chunk, output_chunk)| output_chunk.copy_from_slice(input_chunk));
}