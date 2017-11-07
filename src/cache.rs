extern crate bincode;
extern crate serde;

use std::fs;

use self::bincode::{serialize_into, deserialize_from, Infinite};
use common::{InitialCell, FractalResult};

const CACHE_FILE: &'static str = "fractaldata.cache";

pub fn load_from_cache(initial_configuration: &[InitialCell]) -> Option<FractalResult> {
	if let Ok(mut file) = fs::File::open(CACHE_FILE) {
		let fractal_data: FractalResult = deserialize_from(&mut file, Infinite).unwrap();
		if fractal_data.initial_configuration == initial_configuration {
			return Some(fractal_data);
		} else {
			fs::remove_file(CACHE_FILE).unwrap();
		}
	}

	None
}

pub fn save_to_cache(fractal_data: &FractalResult) -> Result<(),()> {
	
	if let Ok(mut file) = fs::File::create(CACHE_FILE) {
		serialize_into(&mut file, fractal_data, Infinite).unwrap();
		Ok(())
	} else {
		Err(())
	}
}