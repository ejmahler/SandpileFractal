
mod cache;
mod common;
mod compute;
mod render;

use std::time::Instant;
use common::InitialCell;

fn main()
{
	let initial_configuration = [
		InitialCell{x: 0, y: 0, value: 1 << 22},
	];

	let fractal_data = if let Some(data) = cache::load_from_cache(&initial_configuration) {
		println!("Reusing cached fractal data");
		data
	} else {
		let begin = Instant::now();
		let result = compute::compute_fractal_data(&initial_configuration);
		let end = Instant::now();
		let duration = end.duration_since(begin);

		println!("{} iterations computed in {} seconds. redistributions: {}", result.total_iterations, duration.as_secs(), result.total_redistributions);
		match cache::save_to_cache(&result) {
			Ok(()) => println!("Saved fractal data to cache file"),
			Err(()) => println!("Failed to save fractal data to cache file"),
		};

		result
	};
	
	render::render_fractal_data(&fractal_data.sand_data, &fractal_data.count_data, fractal_data.side_length);
}
