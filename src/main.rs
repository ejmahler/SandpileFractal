
extern crate image;
extern crate rayon;

use std::time::Instant;
use image::ImageBuffer;
use rayon::prelude::*;


fn main()
{
	let mut side_length = 320;

	let sandpile_value = 2<<22;
	let mut read_array: Vec<u32> = vec![0; side_length * side_length];


	read_array[(side_length/2) * side_length + side_length/2] = sandpile_value;

	let mut write_array = read_array.clone();


	let begin = Instant::now();
	let mut total_iterations = 0;
	let mut total_redistributions: i64 = 0;
	let mut next_check = side_length / 2 - 1;

	loop
	{
		total_iterations = total_iterations+1;

		let mut current_redist = 0;

		for i in 0..3 {
			let offset = i * side_length;
			let limit = (side_length - i) / 3;

			let read_iter = read_array[offset..].par_chunks(side_length * 3).take(limit);
			let write_iter = write_array[offset..].par_chunks_mut(side_length * 3).take(limit);

			current_redist += read_iter.zip(write_iter).map(|(input_chunk, output_chunk)| process_row(input_chunk, output_chunk, side_length)).sum();
		}

		if current_redist > 0 {
			total_redistributions += current_redist as i64;

			next_check -= 1;
			if next_check == 0 {
				next_check = maybe_reallocate(&mut write_array, &mut read_array, &mut side_length);
			} else {
				copy_data(&write_array, &mut read_array);
			}
		} else {
			break;
		}
	}
	let end = Instant::now();
	let duration = end.duration_since(begin);

	println!("{} iterations computed in {} seconds. redistributions: {}", total_iterations, duration.as_secs(), total_redistributions);
	let mut img = ImageBuffer::new(side_length as u32, side_length as u32);

	for (fractal_value, (x,y, pixel)) in read_array.iter().zip(img.enumerate_pixels_mut()) {
		if x == 0 || y == 0 {
			*pixel = image::Rgb([0,0,0]);
		} else {
			let color = match *fractal_value {
				0 => image::Rgb([0,0,0]),
				1 => image::Rgb([0,200,255]),
				2 => image::Rgb([255,235,125]),
				3 => image::Rgb([250,50,20]),
				_ => image::Rgb([255,0,255]),
			};

			*pixel = color;
		}
	}

	img.save("output.png").unwrap();
}

fn process_row(input_data: &[u32], output_data: &mut [u32], width: usize) -> i32 {

	assert!(input_data.len() == width * 3);
	assert!(output_data.len() == width * 3);

	let mut num_redistributions = 0;
	for i in (width+1)..(width*2-1) {
		let val = input_data[i];

		if val > 3 {
			num_redistributions += 1;

			let rem = val % 4;
			let distribute = val / 4;

			output_data[i - width] += distribute;
			output_data[i - 1] += distribute;
			output_data[i] -= val - rem;
			output_data[i + 1] += distribute;
			output_data[i + width] += distribute;
		}
	}
	num_redistributions
}

fn maybe_reallocate(main_array: &mut Vec<u32>, secondary_array: &mut Vec<u32>, side_length: &mut usize) -> usize {

	// find the bounds of the fractal data, so that we can re-center it inside the new array
	let mut miny = 1;
	'outer_miny: for y in 1..*side_length {
		for x in 1..*side_length {
			let index = y * *side_length + x;
			if main_array[index] >= 4 {
				miny = y;
				break 'outer_miny;
			}
		}
	}

	let mut maxy = 1;
	'outer_maxy: for y in (1..*side_length).rev() {
		for x in (1..*side_length).rev() {
			let index = y * *side_length + x;
			if main_array[index] >= 4 {
				maxy = y;
				break 'outer_maxy;
			}
		}
	}

	let mut minx = 1;
	'outer_minx: for x in 1..*side_length {
		for y in miny..maxy {
			let index = y * *side_length + x;
			if main_array[index] >= 4 {
				minx = x;
				break 'outer_minx;
			}
		}
	}

	let mut maxx = 1;
	'outer_maxx: for x in (1..*side_length).rev() {
		for y in (miny..maxy).rev() {
			let index = y * *side_length + x;
			if main_array[index] >= 4 {
				maxx = x;
				break 'outer_maxx;
			}
		}
	}

	let closest_vertical = std::cmp::min(miny, *side_length - maxy - 1);
	let closest_horizontal = std::cmp::min(minx, *side_length - maxx - 1);

	let closest = std::cmp::min(closest_vertical, closest_horizontal);

	if closest == 0 {
		let increase = 8;
		let new_side_length = *side_length + increase;

		let mut new_main_array = vec![0; new_side_length * new_side_length];

		let size_x = maxx - minx + 1;
		let size_y = maxy - miny + 1;

		let new_x_begin = new_side_length/2 - size_x/2;
		let new_y_begin = new_side_length/2 - size_y/2;
		{
			let old_rows = main_array.chunks(*side_length).skip(miny).take(size_y);
			let new_rows = new_main_array.chunks_mut(new_side_length).skip(new_y_begin).take(size_y);

			for (old_row, new_row) in old_rows.zip(new_rows) {
				new_row[new_x_begin..new_x_begin+size_x].copy_from_slice(&old_row[minx..minx+size_x]);
			}
		}

		*secondary_array = new_main_array.clone();
		*main_array = new_main_array;
		*side_length = new_side_length;

		increase / 2
	} else {
		copy_data(&main_array, &mut *secondary_array);

		closest
	}
}

fn copy_data<T: Copy + Sync + Send>(src: &[T], dst: &mut [T]) {
	let chunk_size = src.len() / 8;
	src.par_chunks(chunk_size).zip(dst.par_chunks_mut(chunk_size)).for_each(|(input_chunk, output_chunk)| output_chunk.copy_from_slice(input_chunk));
}
